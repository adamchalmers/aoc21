use crate::lines::{Line, Point};
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;

/// Parse a Line from the input string.
/// Note this is a Nom parser, so it takes in an input string, and consumes characters from it until it parses a Line.
/// If successful, it returns the match, plus the remaining part of the input string which wasn't consumed.
fn parse_line(input: &str) -> IResult<&str, Line> {
    // Parse two points, separated by an arrow
    let parse_two_points = separated_pair(parse_point, tag(" -> "), parse_point);
    // If the parse succeeded, put those two points into a Line
    map(parse_two_points, |(p0, p1)| Line(p0, p1))(input)
}

/// Parse a point from the start of the input string.
fn parse_point(input: &str) -> IResult<&str, Point> {
    let parse_two_numbers = separated_pair(parse_numbers, char(','), parse_numbers);
    map(parse_two_numbers, |(x, y)| Point { x, y })(input)
}

/// Parse a `u32` from the start of the input string.
pub fn parse_numbers(input: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(input)
}

// Parse the whole problem input.
pub fn parse_input(s: &str) -> Vec<Line> {
    let (remaining_input, lines) = separated_list1(newline, parse_line)(s).unwrap();
    assert!(remaining_input.is_empty());
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
        let s = "1,2asdf";
        let (s, p) = parse_point(s).unwrap();
        assert_eq!(s, "asdf");
        assert_eq!(p, Point { x: 1, y: 2 });
    }

    #[test]
    fn test_parse_line() {
        let s = "284,294 -> 733,743asdf";
        let (s, l) = parse_line(s).unwrap();
        assert_eq!(s, "asdf");
        assert_eq!(l, Line(Point { x: 284, y: 294 }, Point { x: 733, y: 743 }));
    }

    #[test]
    fn test_parse_example() {
        let s = include_str!("example.txt");
        let l = parse_input(s);
        assert_eq!(l.len(), 10);
    }
}
