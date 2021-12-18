use nom::{
    bits::{bits, complete::take},
    IResult,
};

fn main() {
    let input = parse_hex(include_str!("data/input.txt"));
    let (_rem, packets) = Packet::parse(&input).unwrap();
    println!("Q1: {}", packets.sum_versions());
}

/// Parse an even-length string of hex into bytes.
fn parse_hex(s: &str) -> Vec<u8> {
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

/// Bitwise input for Nom parsers.
type BitInput<'a> = (&'a [u8], usize);

/// Takes n bits from the BitInput.
/// Returns the remaining BitInput and a number from the first n bits.
fn take_n_bits(i: BitInput, n: u8) -> IResult<BitInput, u8> {
    println!(
        "Taking {} bits from input. {} remain",
        n,
        i.0.len() * 8 - i.1
    );
    let out = take(n)(i);
    println!("Took");
    out
}

fn parse_header_bits(i: BitInput) -> IResult<BitInput, Header> {
    let (i, version) = take_n_bits(i, 3)?;
    let (i, type_id) = take_n_bits(i, 3)?;
    Ok((i, Header { version, type_id }))
}

fn parse_packet_bits(i: BitInput) -> IResult<BitInput, Packet> {
    let (i, header) = parse_header_bits(i)?;
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

#[derive(Eq, PartialEq, Debug)]
struct Packet {
    version: u8,
    body: PacketBody,
}

#[derive(Eq, PartialEq, Debug)]
enum PacketBody {
    Literal(u16),
    Operator {
        type_id: u8,
        subpackets: Vec<Packet>,
    },
}

impl Packet {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        bits(parse_packet_bits)(i)
    }

    fn sum_versions(&self) -> u16 {
        match &self.body {
            PacketBody::Literal(_) => 0,
            PacketBody::Operator { subpackets, .. } => {
                subpackets.iter().map(|p| p.sum_versions()).sum()
            }
        }
    }
}

fn parse_operator(mut i: BitInput, type_id: u8) -> IResult<BitInput, PacketBody> {
    let mut subpackets = Vec::new();
    let (j, length_type_id) = take_n_bits(i, 1)?;
    i = j;
    if length_type_id == 0 {
        // the next 15 bits are a number that represents
        // the total length in bits of the sub-packets contained by this packet.
        let (j, total_subpacket_lengths) = take_n_bits(i, 15)?;
        i = j;

        // The length indicates when to stop parsing packets.
        let end_offset = i.1 + total_subpacket_lengths as usize;
        // Parse subpackets until the length is reached.
        while i.1 < end_offset {
            let (j, packet) = parse_packet_bits(i)?;
            i = j;
            subpackets.push(packet);
        }
    } else {
        // then the next 11 bits are a number that represents
        // the number of sub-packets immediately contained by this packet.
        let (j, num_subpackets) = take_n_bits(i, 11)?;
        i = j;
        for _ in 0..num_subpackets {
            let (j, packet) = parse_packet_bits(i)?;
            subpackets.push(packet);
            i = j;
        }
    }

    Ok((
        i,
        PacketBody::Operator {
            subpackets,
            type_id,
        },
    ))
}

fn parse_literal_number(mut i: BitInput) -> IResult<BitInput, PacketBody> {
    let mut half_bytes = Vec::new();
    loop {
        let (j, bit) = take_n_bits(i, 1)?;
        let (j, half_byte) = take_n_bits(j, 4)?;
        i = j;
        half_bytes.push(half_byte);
        if bit == 0 {
            break;
        }
    }
    let n = half_bytes.len() - 1;
    let num: u16 = half_bytes
        .into_iter()
        .enumerate()
        .map(|(i, b)| (n - i, b))
        .map(from_nibble)
        .sum();
    Ok((i, PacketBody::Literal(num)))
}

/// A nibble is half a byte.
fn from_nibble((i, nibble): (usize, u8)) -> u16 {
    (nibble as u16) << (4 * i)
}

#[derive(Eq, PartialEq, Debug)]
struct Header {
    version: u8,
    type_id: u8,
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
    fn test_parse_subpackets_total_length() {
        let input = parse_hex("38006F45291200");
        let (_, actual) = Packet::parse(&input).unwrap();
        assert_eq!(actual.version, 1);
        if let PacketBody::Operator {
            type_id,
            subpackets,
        } = actual.body
        {
            assert_eq!(subpackets.len(), 2);
            assert_eq!(type_id, 6);
        } else {
            panic!("Packet body should be Operator not Literal")
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

    #[test]
    fn test_sum_versions() {
        let tests = vec![(EXAMPLE_LITERAL, 0)];
        for (hex, expected) in tests {
            let (_, packets) = Packet::parse(&parse_hex(hex)).unwrap();
            assert_eq!(packets.sum_versions(), expected);
        }
    }
}
