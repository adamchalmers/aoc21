use crate::lines::{Line, Point};
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;

impl Line {
    /// Parse a Line from the input string.
    /// Note this is a Nom parser, so it takes in an input string, and consumes characters from it until it parses a Line.
    /// If successful, it returns the match, plus the remaining part of the input string which wasn't consumed.
    fn parse(input: &str) -> IResult<&str, Self> {
        // Parse two points, separated by an arrow
        let parse_two_points = separated_pair(Point::parse, tag(" -> "), Point::parse);
        // If the parse succeeded, put those two points into a Line
        map(parse_two_points, |(p0, p1)| Line(p0, p1))(input)
    }
}

impl Point {
    /// Parse a point from the start of the input string.
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_two_numbers = separated_pair(parse_numbers, char(','), parse_numbers);
        map(parse_two_numbers, |(x, y)| Point { x, y })(input)
    }
}

/// Parse a `u32` from the start of the input string.
pub fn parse_numbers(input: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(input)
}

/// Parse the whole problem input.
pub fn parse_input(s: &str) -> Vec<Line> {
    let (remaining_input, lines) = separated_list1(line_ending, Line::parse)(s).unwrap();
    assert!(remaining_input.is_empty());
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
        let tests = [
            ("1,2", Point { x: 1, y: 2 }, ""),
            ("1,2asdf", Point { x: 1, y: 2 }, "asdf"),
        ];
        for (input, expected_output, expected_remaining_input) in tests {
            let (remaining_input, output) = Point::parse(input).unwrap();
            assert_eq!(remaining_input, expected_remaining_input);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_parse_line() {
        let tests = [
            (
                "0,9 -> 5,9",
                Line(Point { x: 0, y: 9 }, Point { x: 5, y: 9 }),
                "",
            ),
            (
                "0,9 -> 5,9xyz",
                Line(Point { x: 0, y: 9 }, Point { x: 5, y: 9 }),
                "xyz",
            ),
        ];
        for (input, expected_output, expected_remaining_input) in tests {
            let (remaining_input, output) = Line::parse(input).unwrap();
            assert_eq!(remaining_input, expected_remaining_input);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_parse_example() {
        let input = include_str!("example.txt");
        let lines = parse_input(input);
        assert_eq!(lines.len(), 10);
    }
}
