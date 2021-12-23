//! I need a hierarchical representation of Snailfish numbers for calculating magnitudes.
use crate::tokenstream::*;
use nom::{
    branch::alt, bytes::complete::take_while_m_n, character::complete::char, combinator::map,
    combinator::map_res, sequence::tuple, IResult,
};
use std::str::FromStr;

/// A hierarchical representation of snailfish numbers.
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
    /// Nom parser.
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_node, Self::parse_leaf))(input)
    }

    /// Nom parser. Parses Tree::Node case.
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
        map(parse_one_digit, Tree::Leaf)(input)
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

/// Nom parser. Parses exactly one decimal digit.
fn parse_one_digit(input: &str) -> IResult<&str, u8> {
    let str_to_digit = |input: &str| input.parse::<u8>();

    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }

    map_res(take_while_m_n(1, 1, is_digit), str_to_digit)(input)
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
    fn all_representations_are_equivalent() {
        let inputs = vec!["[[1,2],3]", "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"];
        for input in inputs {
            let tree1 = Tree::from_str(input).unwrap().to_string();
            let tree2 = TokenStream::from_str(input).unwrap().to_string();
            assert_eq!(tree1, tree2);
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
