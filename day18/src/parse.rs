use std::str::FromStr;

use crate::sailfish_number::Tree;
use nom::{
    branch::alt,
    bytes::complete::take_while_m_n,
    character::complete::{char, newline},
    combinator::map,
    combinator::map_res,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

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
        map(parse_one_digit, Tree::Leaf)(input)
    }

    /// Nom parser. Parses either Leaf or Node.
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_node, Self::parse_leaf))(input)
    }

    /// Nom parser. Parses a newline-separated list of Sailfish number pairs.
    pub fn parse_many(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list0(newline, Self::parse)(input)
    }
}

impl FromStr for Tree {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, val) = Self::parse(s).map_err(|e| e.to_string())?;
        Ok(val)
    }
}

/// Nom parser. Parses exactly one decimal digit.
fn parse_one_digit(input: &str) -> IResult<&str, u8> {
    let str_to_digit = |input| u8::from_str_radix(input, 10);

    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }

    map_res(take_while_m_n(1, 1, is_digit), str_to_digit)(input)
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
}
