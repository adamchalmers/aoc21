use nom::{
    bytes::complete::tag,
    bytes::complete::take_while,
    character::complete::{char, digit1, newline},
    combinator::map,
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, tuple},
};
use std::collections::HashSet;

type IResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Default)]
pub struct Point {
    x: i16,
    y: i16,
    z: i16,
}

/// Nom parser
impl Point {
    fn parse(i: &str) -> IResult<Self> {
        fn parse_number(i: &str) -> IResult<i16> {
            map_res(
                take_while(|c: char| c.is_digit(10) || c == '-'),
                |i: &str| i.parse(),
            )(i)
        }

        map(separated_list1(char(','), parse_number), |nums| Self {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        })(i)
    }
}

pub struct Scanner {
    beacons: HashSet<Point>,
}

impl Scanner {
    fn parse(i: &str) -> IResult<Self> {
        map(separated_list1(char('\n'), Point::parse), |points| Self {
            beacons: points.into_iter().collect(),
        })(i)
    }
}

pub struct Problem {
    scanners: Vec<Scanner>,
}

impl Problem {
    pub fn parse(i: &str) -> IResult<Self> {
        let p_header = tuple((tag("--- scanner "), digit1, tag(" ---\n")));
        let p = delimited(p_header, Scanner::parse, newline);
        map(separated_list1(newline, p), |scanners| Problem { scanners })(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_problem() {
        let s = include_str!("data/example.txt");
        let (unparsed, problem) = Problem::parse(s).unwrap();
        assert_eq!(unparsed, "");
        assert_eq!(problem.scanners.len(), 5);
    }
}
