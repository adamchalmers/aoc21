use crate::tokenstream::*;
use nom::{
    branch::alt, bytes::complete::take_while_m_n, character::complete::char, combinator::map,
    combinator::map_res, sequence::tuple, IResult,
};
use std::str::FromStr;

/// A hierarchical representation of sailfish numbers.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Tree {
    Leaf(u8),
    Node { l: Box<Tree>, r: Box<Tree> },
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Leaf(n) => n.to_string(),
            Self::Node { l, r } => format!("[{},{}]", l.to_string(), r.to_string()),
        };
        write!(f, "{}", s)
    }
}

impl From<TokenStream> for Tree {
    fn from(ts: TokenStream) -> Self {
        let s = ts.to_string();
        Tree::from_str(&s).unwrap()
    }
}

impl Tree {
    /// Nom parser. Parses a Sailfish number pair.
    fn parse_node(input: &str) -> IResult<&str, Self> {
        let parser = tuple((char('['), Tree::parse, char(','), Tree::parse, char(']')));
        let mut discard_delimiter_parser = map(parser, |(_, l, _, r, _)| Self::Node {
            l: Box::new(l),
            r: Box::new(r),
        });
        discard_delimiter_parser(input)
    }

    /// Nom parser. Parses Tree::Leaf case.
    fn parse_leaf(input: &str) -> IResult<&str, Self> {
        /// Nom parser. Parses exactly one decimal digit.
        fn parse_one_digit(input: &str) -> IResult<&str, u8> {
            let str_to_digit = |input: &str| input.parse::<u8>();

            fn is_digit(c: char) -> bool {
                c.is_digit(10)
            }

            map_res(take_while_m_n(1, 1, is_digit), str_to_digit)(input)
        }
        map(parse_one_digit, Tree::Leaf)(input)
    }

    /// Nom parser. Parses either Leaf or Node.
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_node, Self::parse_leaf))(input)
    }

    pub fn magnitude(&self) -> u16 {
        match self {
            Tree::Leaf(n) => *n as u16,
            // The magnitude of a pair is 3 times the magnitude of its left element
            // plus 2 times the magnitude of its right element.
            Tree::Node { l, r } => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

impl FromStr for Tree {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, val) = Self::parse(s).map_err(|e| e.to_string())?;
        Ok(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tests = [
            ("[1,2]", true),
            ("[[1,2],3]", true),
            ("[9,[8,7]]", true),
            ("[[1,9],[8,5]]", true),
            ("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]", true),
            ("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]", true),
            (
                "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
                true,
            ),
        ];
        for (input_str, should_parse) in tests {
            assert_eq!(Tree::parse(input_str).is_ok(), should_parse);
        }
    }

    #[test]
    fn all_representations_are_equivalent() {
        let inputs = vec!["[[1,2],3]", "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"];
        for input in inputs {
            // Parsing strings into and out of Trees
            let tree = Tree::from_str(input).unwrap();
            let s1 = tree.to_string();
            assert_eq!(s1, input);
            // Parsing strings into and out of TokenStreams
            let stream = TokenStream::from_str(&s1).unwrap();
            assert_eq!(s1, stream.to_string());
        }
    }

    #[test]
    fn magnitudes() {
        let tests = [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];
        for (input, expected_magnitude) in tests {
            let actual_magnitude = Tree::from_str(input).unwrap().magnitude();
            assert_eq!(actual_magnitude, expected_magnitude, "case {}", input);
        }
    }
}
