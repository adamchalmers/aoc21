use nom::{
    branch::alt, bytes::complete::take_while_m_n, character::complete::char, combinator::map,
    combinator::map_res, sequence::tuple, IResult,
};

/// Iterate over a Snailfish number, adding up its magnitude as you go.
pub fn parse_magnitude(input: &str) -> u16 {
    magnitude(input).unwrap().1
}

// Parse a stringified Snailfish number, adding up its magnitude.
fn magnitude(input: &str) -> IResult<&str, u16> {
    // An element can be either a pair of elements, or a number literal.
    alt((magnitude_of_pair, parse_number))(input)
}

// Parse a stringified Snailfish pair, adding up its magnitude.
fn magnitude_of_pair(input: &str) -> IResult<&str, u16> {
    let parser = tuple((char('['), magnitude, char(','), magnitude, char(']')));
    let mut discard_delimiter_parser = map(parser, |(_, l, _, r, _)| 3 * l + 2 * r);
    discard_delimiter_parser(input)
}

// Parse a sequence of digits into a number.
pub fn parse_number(input: &str) -> IResult<&str, u16> {
    map_res(
        take_while_m_n(1, 1, |c: char| c.is_digit(10)),
        |input: &str| input.parse(),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            let actual_magnitude = parse_magnitude(input);
            assert_eq!(actual_magnitude, expected_magnitude, "case {}", input);
        }
    }
}
