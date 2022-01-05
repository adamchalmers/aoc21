mod parse;

fn main() {
    let input = parse::parse_hex(include_str!("data/input.txt"));
    let (_remaining_input, packets) = Packet::parse(&input).unwrap();
    println!("Q1: {}", packets.sum_versions());
    println!("Q2: {}", packets.eval());
}

/// A tree structure that represents some number. Can be parsed out of its binary encoding.
#[derive(Eq, PartialEq, Debug)]
enum Packet {
    /// Leaf node.
    /// Represents a number directly.
    Literal { version: u8, value: u64 },
    /// Internal node.
    /// Represents the number you get from running the given operation on the given subpackets.
    Operator {
        version: u8,
        type_id: Operation,
        subpackets: Vec<Packet>,
    },
}

/// Each operator packet has an operation which it runs on the values of its subpackets.
#[derive(Eq, PartialEq, Debug)]
enum Operation {
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

impl Packet {
    /// Used for Q1. Simply sum all version numbers in every packet.
    fn sum_versions(&self) -> u64 {
        match &self {
            Packet::Literal { version, .. } => *version as u64,
            Packet::Operator {
                subpackets,
                version,
                ..
            } => subpackets.iter().map(|p| p.sum_versions()).sum::<u64>() + (*version as u64),
        }
    }

    /// Evaluate the packet's numeric expression.
    fn eval(&self) -> u64 {
        match &self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator {
                type_id,
                subpackets,
                ..
            } => match type_id {
                Operation::Sum => subpackets.iter().map(|p| p.eval()).sum(),
                Operation::Product => subpackets.iter().map(|p| p.eval()).product(),
                Operation::Min => subpackets.iter().map(|p| p.eval()).min().unwrap(),
                Operation::Max => subpackets.iter().map(|p| p.eval()).max().unwrap(),
                Operation::Greater => bool_to_u1(subpackets[0].eval() > subpackets[1].eval()),
                Operation::Less => bool_to_u1(subpackets[0].eval() < subpackets[1].eval()),
                Operation::Equal => bool_to_u1(subpackets[0].eval() == subpackets[1].eval()),
            },
        }
    }
}

fn bool_to_u1(b: bool) -> u64 {
    if b {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_versions() {
        let tests = vec![("D2FE28", 6), ("EE00D40C823060", 14), ("38006F45291200", 9)];
        for (hex, expected) in tests {
            let (_, packets) = Packet::parse(&parse::parse_hex(hex)).unwrap();
            assert_eq!(packets.sum_versions(), expected, "Failed {}", hex);
        }
    }

    #[test]
    fn test_eval() {
        let tests = vec![
            // C200B40A82 finds the sum of 1 and 2, resulting in the value 3.
            ("C200B40A82", 3),
            // 04005AC33890 finds the product of 6 and 9, resulting in the value 54.
            ("04005AC33890", 54),
            // 880086C3E88112 finds the minimum of 7, 8, and 9, resulting in the value 7.
            ("880086C3E88112", 7),
            // CE00C43D881120 finds the maximum of 7, 8, and 9, resulting in the value 9.
            ("CE00C43D881120", 9),
            // D8005AC2A8F0 produces 1, because 5 is less than 15.
            ("D8005AC2A8F0", 1),
            // F600BC2D8F produces 0, because 5 is not greater than 15.
            ("F600BC2D8F", 0),
            // 9C005AC2F8F0 produces 0, because 5 is not equal to 15.
            ("9C005AC2F8F0", 0),
            // 9C0141080250320F1802104A08 produces 1, because 1 + 3 = 2 * 2.
            ("9C0141080250320F1802104A08", 1),
        ];
        for (i, (hex, expected)) in tests.iter().enumerate() {
            let binary = parse::parse_hex(hex);
            let (_, packet) = Packet::parse(&binary).unwrap();
            assert_eq!(
                packet.eval(),
                *expected,
                "Failed test #{}, input: {}",
                i,
                hex
            )
        }
    }
}
