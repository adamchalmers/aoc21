use crate::{Operation, Packet, PacketBody};
use nom::{
    bits::{bits, complete::take},
    IResult,
};

/// Newtype around a very common type in Nom.
/// Represents a binary sequence which can be parsed one bit at a time.
/// Nom represents this as a sequence of bytes, and an offset tracking which number bit
/// is currently being read.
///
/// For example, you might start with 16 bits, pointing at the 0th bit:
///```
/// 1111000011001100
/// ^
/// ```
/// Nom represents this using the BitInput type as:
/// ```
/// ([0b11110000, 0b11001100], 0)
///     ^
/// ```
/// Lets say you parsed 3 bits from there. After that, the BitInput would be
///
/// ```
/// ([0b11110000, 0b11001100], 3)
///        ^
/// ```
/// After reading another six bits, the input would have advanced past the first byte:
///
/// ```
/// ([0b11110000, 0b11001100], 9)
///                  ^
/// ```
/// Because the first byte will never be used again, Nom optimizes by dropping the first byte
///
/// ```
///  ([0b11001100], 1)
///       ^
/// ```
type BitInput<'a> = (&'a [u8], usize);

/// How many bits can still be parsed from the BitInput.
fn bits_remaining(i: &BitInput) -> usize {
    // How far through the first byte are we?
    let bits_in_first_byte = 8 - i.1;
    // And how many bytes are left after that?
    let remaining_bytes = i.0.len() - 1;
    bits_in_first_byte + (8 * remaining_bytes)
}

/// Takes n bits from the BitInput.
/// Returns the remaining BitInput and a number parsed the first n bits.
fn take_up_to_8_bits(i: BitInput, n: u8) -> IResult<BitInput, u8> {
    take(n)(i)
}

/// Takes n bits from the BitInput.
/// Returns the remaining BitInput and a number parsed the first n bits.
fn take_up_to_16_bits(i: BitInput, n: u8) -> IResult<BitInput, u16> {
    take(n)(i)
}

/// Every packet has a header.
#[derive(Eq, PartialEq, Debug)]
struct Header {
    version: u8,
    type_id: u8,
}

impl Header {
    fn parse(i: BitInput) -> IResult<BitInput, Self> {
        let (i, version) = take_up_to_8_bits(i, 3)?;
        let (i, type_id) = take_up_to_8_bits(i, 3)?;
        Ok((i, Self { version, type_id }))
    }
}

impl Packet {
    /// Parse a Packet from a sequence of bytes.
    pub fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        // Convert the byte-offset input into a bit-offset input, then parse that.
        bits(Self::parse_from_bits)(i)
    }

    /// Parse a Packet from a sequence of bits.
    fn parse_from_bits(i: BitInput) -> IResult<BitInput, Self> {
        let (i, header) = Header::parse(i)?;
        let (i, body) = match header.type_id {
            4 => parse_literal_number(i)?,
            other => parse_operator(i, other)?,
        };
        let packet = Packet {
            version: header.version,
            body,
        };
        Ok((i, packet))
    }
}

/// Parse a PacketBody::Operator from a sequence of bits.
fn parse_operator(mut i: BitInput, type_id: u8) -> IResult<BitInput, PacketBody> {
    let mut subpackets = Vec::new();
    let (remaining_i, length_type_id) = take_up_to_8_bits(i, 1)?;
    i = remaining_i;
    if length_type_id == 0 {
        // the next 15 bits are a number that represents
        // the total length in bits of the sub-packets contained by this packet.
        let (remaining_i, total_subpacket_lengths) = take_up_to_16_bits(i, 15)?;
        i = remaining_i;

        // Parse subpackets until the length is reached.
        let initial_bits_remaining = bits_remaining(&i);
        while initial_bits_remaining - bits_remaining(&i) < (total_subpacket_lengths as usize) {
            let (j, packet) = Packet::parse_from_bits(i)?;
            i = j;
            subpackets.push(packet);
        }
    } else {
        // then the next 11 bits are a number that represents
        // the number of sub-packets immediately contained by this packet.
        let (remaining_i, num_subpackets) = take_up_to_16_bits(i, 11)?;
        i = remaining_i;
        for _ in 0..num_subpackets {
            let (remaining_i, packet) = Packet::parse_from_bits(i)?;
            subpackets.push(packet);
            i = remaining_i;
        }
    }

    Ok((
        i,
        PacketBody::Operator {
            subpackets,
            type_id: Operation::from(type_id),
        },
    ))
}

/// Parse a PacketBody::Literal from a sequence of bits.
fn parse_literal_number(mut i: BitInput) -> IResult<BitInput, PacketBody> {
    let mut half_bytes = Vec::new();
    loop {
        let (remaining_i, bit) = take_up_to_8_bits(i, 1)?;
        let (remaining_i, half_byte) = take_up_to_8_bits(remaining_i, 4)?;
        i = remaining_i;
        half_bytes.push(half_byte);
        if bit == 0 {
            break;
        }
    }
    let n = half_bytes.len() - 1;
    let num: u64 = half_bytes
        .into_iter()
        .enumerate()
        .map(|(i, b)| (n - i, b))
        .map(from_nibble)
        .sum();
    Ok((i, PacketBody::Literal(num)))
}

/// A nibble is a u4 (half a byte). But Rust doesn't have a u4 type!
/// So we store the u4s in u8s, and then use bit-shifting operations to put them into the right
/// column of the larger binary number we're working with.
fn from_nibble((i, nibble): (usize, u8)) -> u64 {
    (nibble as u64) << (4 * i)
}

/// Every type_id corresponds to a particular operation.
impl From<u8> for Operation {
    fn from(type_id: u8) -> Self {
        match type_id {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            4 => panic!("Literals should not be parsed into Operations"),
            5 => Self::Greater,
            6 => Self::Less,
            7 => Self::Equal,
            other => panic!("illegal type_id {}", other),
        }
    }
}

/// Parse an even-length string of hex into bytes.
pub fn parse_hex(s: &str) -> Vec<u8> {
    if s.len() % 2 != 0 {
        panic!("{} cannot be parsed to bytes because it's odd length.", s)
    }
    let chars = s.chars().into_iter().collect::<Vec<_>>();
    (0..chars.len() / 2)
        .map(|i| {
            let two_hex_chars = chars[2 * i..2 * i + 2].iter().collect::<String>();
            u8::from_str_radix(&two_hex_chars, 16).unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_LITERAL: &str = "D2FE28";

    #[test]
    fn test_parse_hex() {
        let expected = vec![0b11010010, 0b11111110, 0b00101000];
        let actual = parse_hex(EXAMPLE_LITERAL);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_parse_literal() {
        let input = parse_hex(EXAMPLE_LITERAL);
        let (_rem, actual) = Packet::parse(&input).unwrap();
        let expected = Packet {
            version: 6,
            body: PacketBody::Literal(2021),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_literal2() {
        let input = [0b11010001, 0b01000000];
        let (_, actual) = Packet::parse(&input).unwrap();
        let expected = Packet {
            version: 6,
            body: PacketBody::Literal(10),
        };
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_parse_literal3() {
        let input = [0b01010010, 0b00100100];
        let (_, actual) = Packet::parse(&input).unwrap();
        let expected = Packet {
            version: 2,
            body: PacketBody::Literal(20),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_operator_total_length() {
        let input = parse_hex("38006F45291200");
        let (_, actual) = Packet::parse(&input).unwrap();
        let expected = Packet {
            version: 1,
            body: PacketBody::Operator {
                type_id: Operation::from(6),
                subpackets: vec![
                    Packet {
                        version: 6,
                        body: PacketBody::Literal(10),
                    },
                    Packet {
                        version: 2,
                        body: PacketBody::Literal(20),
                    },
                ],
            },
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_operator_num_subpackets() {
        let input = parse_hex("EE00D40C823060");
        let (_, actual) = Packet::parse(&input).unwrap();
        let expected = Packet {
            version: 7,
            body: PacketBody::Operator {
                type_id: Operation::from(3),
                subpackets: vec![
                    Packet {
                        version: 2,
                        body: PacketBody::Literal(1),
                    },
                    Packet {
                        version: 4,
                        body: PacketBody::Literal(2),
                    },
                    Packet {
                        version: 1,
                        body: PacketBody::Literal(3),
                    },
                ],
            },
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parsing() {
        let tests = [
            "8A004A801A8002F478",
            "620080001611562C8802118E34",
            "C0015000016115A2E0802F182340",
            "A0016C880162017C3686B18A3D4780",
        ];
        for hex in tests {
            let input = parse_hex(hex);
            let (_, packet) = Packet::parse(&input).unwrap();
            assert!(matches!(packet.body, PacketBody::Operator { .. }));
        }
    }

    #[test]
    fn test_from_nibble() {
        let tests = vec![
            (2, 0b0111, 1024 + 512 + 256),
            (1, 0b1110, 128 + 64 + 32),
            (0, 0b0101, 4 + 1),
        ];
        for (i, input_nibble, expected) in tests {
            let actual = from_nibble((i, input_nibble));
            assert_eq!(actual, expected, "{}", input_nibble);
        }
    }
}
