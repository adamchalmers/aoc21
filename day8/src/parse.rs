use nom::{
    bytes::complete::{tag, take_while},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl TryFrom<char> for Segment {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let seg = match value {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            _ => return Err("not a segment"),
        };
        Ok(seg)
    }
}

fn is_segment(c: char) -> bool {
    Segment::try_from(c).is_ok()
}

fn to_segs(s: &str) -> Result<HashSet<Segment>, &'static str> {
    if s.is_empty() {
        return Err("cannot have empty set of segments");
    }
    let mut set = HashSet::default();
    for c in s.chars() {
        set.insert(Segment::try_from(c)?);
    }
    Ok(set)
}

pub struct Observation {
    signal_patterns: [HashSet<Segment>; 10],
    pub output_value: [HashSet<Segment>; 4],
}

impl Observation {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (signal_patterns, output_value)) =
            separated_pair(signal_patterns_parser, tag(" | "), signal_patterns_parser)(input)?;
        let signal_patterns = signal_patterns.try_into().unwrap();
        let output_value = output_value.try_into().unwrap();
        Ok((
            input,
            Self {
                signal_patterns,
                output_value,
            },
        ))
    }
    pub fn parse_lines(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list1(tag("\n"), Observation::parse)(input)
    }
}

fn signal_patterns_parser(input: &str) -> IResult<&str, Vec<HashSet<Segment>>> {
    separated_list1(tag(" "), segments_parser)(input)
}

fn segments_parser(input: &str) -> IResult<&str, HashSet<Segment>> {
    map_res(take_while(is_segment), to_segs)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_segments() {
        let actual = segments_parser("ab").unwrap().1;
        assert_eq!(actual, HashSet::from([Segment::A, Segment::B]))
    }

    #[test]
    fn parse_signal_patterns() {
        let actual = signal_patterns_parser("ab def ").unwrap().1;
        assert_eq!(
            actual,
            vec![
                HashSet::from([Segment::A, Segment::B]),
                HashSet::from([Segment::D, Segment::E, Segment::F]),
            ]
        )
    }

    #[test]
    fn parse_line() {
        let actual = Observation::parse(include_str!("tiny.txt")).unwrap().1;
        assert_eq!(
            actual.signal_patterns[9],
            HashSet::from([Segment::A, Segment::B])
        );
    }

    #[test]
    fn parse_observation() {
        Observation::parse_lines(include_str!("example.txt")).unwrap();
    }
}
