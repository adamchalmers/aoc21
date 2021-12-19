use crate::sailfish_number::{Element, Number};
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

impl Number {
    /// Nom parser. Parses a Sailfish number pair.
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let parser = tuple((
            char('['),
            Element::parse,
            char(','),
            Element::parse,
            char(']'),
        ));
        let mut discard_delimiter_parser = map(parser, |(_, l, _, r, _)| Self { l, r });
        discard_delimiter_parser(input)
    }

    /// Nom parser. Parses a newline-separated list of Sailfish number pairs.
    pub fn parse_many(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list0(newline, Self::parse)(input)
    }
}

impl Element {
    /// Nom parser. Parses Element::Num case.
    fn parse_num(input: &str) -> IResult<&str, Self> {
        map(parse_one_digit, Element::Literal)(input)
    }

    /// Nom parser. Parses Element::Pair case.
    fn parse_pair(input: &str) -> IResult<&str, Self> {
        map(Number::parse, |p| Element::Pair(Box::new(p)))(input)
    }

    /// Nom parser. Parses either case of Element.
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_num, Self::parse_pair))(input)
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
            assert_eq!(Number::parse(input_str).is_ok(), should_parse);
        }
    }
}
