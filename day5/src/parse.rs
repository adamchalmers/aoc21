use crate::lines::{Line, Point, Scale};
use nom::{
    bytes::complete::{tag, take_while},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;

/// Parse a Line from the input string
fn parse_line(s: &str) -> IResult<&str, Line> {
    // Parse two points, separated by an arrow
    let parser = separated_pair(parse_point, tag(" -> "), parse_point);
    // If the parse succeeded, put those two points into a Line
    map(parser, |(p0, p1)| Line(p0, p1))(s)
}

/// Parse a point from the input string.
fn parse_point(s: &str) -> IResult<&str, Point> {
    let parser = separated_pair(parse_numbers, tag(","), parse_numbers);
    map(parser, |(x, y)| Point { x, y })(s)
}

/// Match a `Scale` from the start of the input.
pub fn parse_numbers(input: &str) -> IResult<&str, Scale> {
    map_res(take_while(|c: char| c.is_digit(10)), |input| {
        Scale::from_str(input)
    })(input)
}

// Parse the whole input.
pub fn parse_input(s: &str) -> Vec<Line> {
    let (remaining_input, lines) = separated_list1(tag("\n"), parse_line)(s).unwrap();
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
