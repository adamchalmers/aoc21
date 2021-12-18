use nom::{
    bits::{bits, complete::take},
    IResult,
};

fn main() {
    println!("Hello, world!");
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

/// Takes 3 bits from the BitInput, returns the remaining BitInput and a number from the first 3 bits.
fn take_n_bits(i: BitInput, n: usize) -> IResult<BitInput, u8> {
    take(n)(i)
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
        _ => unreachable!(),
    };
    let packet = Packet {
        version: header.version,
        body,
    };
    Ok((i, packet))
}

fn parse_packet_bytes(i: &[u8]) -> IResult<&[u8], Packet> {
    bits(parse_packet_bits)(i)
}

#[derive(Eq, PartialEq, Debug)]
struct Packet {
    version: u8,
    body: PacketBody,
}

#[derive(Eq, PartialEq, Debug)]
enum PacketBody {
    Literal(u16),
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
    println!("Nibble #{} is {:b}", i, nibble);
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
        let (_rem, actual) = parse_packet_bytes(&input).unwrap();
        let expected = Packet {
            version: 6,
            body: PacketBody::Literal(2021),
        };
        assert_eq!(actual, expected);
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
        assert_eq!(1024 + 512 + 256 + 128 + 64 + 32 + 5, 2021);
    }
}

// (0111)
// (1110)
// (0101)
