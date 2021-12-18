mod parse;

fn main() {
    let input = parse::parse_hex(include_str!("data/input.txt"));
    let (_rem, packets) = Packet::parse(&input).unwrap();
    println!("Q1: {}", packets.sum_versions());
}

#[derive(Eq, PartialEq, Debug)]
struct Packet {
    version: u8,
    body: PacketBody,
}

/// Forms a tree structure.
#[derive(Eq, PartialEq, Debug)]
enum PacketBody {
    // Internal node
    Literal(u64),
    // Leaf node
    Operator {
        type_id: Operation,
        subpackets: Vec<Packet>,
    },
}

#[derive(Eq, PartialEq, Debug)]
enum Operation {
    Sum,
    Product,
    Min,
    Max,
    Literal,
    Greater,
    Less,
    Equal,
}

impl Packet {
    fn sum_versions(&self) -> u64 {
        let curr = self.version as u64;
        match &self.body {
            PacketBody::Literal(_) => curr,
            PacketBody::Operator { subpackets, .. } => {
                subpackets.iter().map(|p| p.sum_versions()).sum::<u64>() + curr
            }
        }
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
}
