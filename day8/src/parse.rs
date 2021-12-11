use nom::{
    bytes::complete::{tag, take_while},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::BTreeSet;
use std::convert::TryInto;

pub type Pattern = BTreeSet<Segment>;

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

fn to_segs(s: &str) -> Result<Pattern, &'static str> {
    if s.is_empty() {
        return Err("cannot have empty set of segments");
    }
    let mut set = BTreeSet::default();
    for c in s.chars() {
        set.insert(Segment::try_from(c)?);
    }
    Ok(set)
}

pub struct DisplayPanel {
    /// The ten different signal patterns this display uses, one for each digit.
    pub signal_patterns: [Pattern; 10],
    /// The signal patterns this display is currently using to show a 4-digit number.
    pub output_value: [Pattern; 4],
}

impl DisplayPanel {
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
        separated_list1(tag("\n"), DisplayPanel::parse)(input)
    }
}

/// Parse a series of patterns.
fn signal_patterns_parser(input: &str) -> IResult<&str, Vec<Pattern>> {
    separated_list1(tag(" "), segments_parser)(input)
}

/// Parse a series of consecutive characters a-g, into a set of segments aka a pattern.
fn segments_parser(input: &str) -> IResult<&str, Pattern> {
    map_res(take_while(is_segment), to_segs)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_segments() {
        let actual = segments_parser("ab").unwrap().1;
        assert_eq!(actual, BTreeSet::from([Segment::A, Segment::B]))
    }

    #[test]
    fn parse_signal_patterns() {
        let actual = signal_patterns_parser("ab def ").unwrap().1;
        assert_eq!(
            actual,
            vec![
                BTreeSet::from([Segment::A, Segment::B]),
                BTreeSet::from([Segment::D, Segment::E, Segment::F]),
            ]
        )
    }

    #[test]
    fn parse_line() {
        let actual = DisplayPanel::parse(include_str!("tiny.txt")).unwrap().1;
        assert_eq!(
            actual.signal_patterns[9],
            BTreeSet::from([Segment::A, Segment::B])
        );
    }

    #[test]
    fn parse_display_panel() {
        DisplayPanel::parse_lines(include_str!("example.txt")).unwrap();
    }
}
