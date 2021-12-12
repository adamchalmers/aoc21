use nom::{
    bytes::complete::{tag, take_while},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::BTreeSet;
use std::convert::TryInto;

pub type Pattern = BTreeSet<char>;

fn is_segment(c: char) -> bool {
    ('a'..='g').contains(&c)
}

fn to_segs(s: &str) -> Result<Pattern, &'static str> {
    if s.is_empty() {
        return Err("cannot have empty set of segments");
    }
    Ok(s.chars().collect())
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
        assert_eq!(actual, BTreeSet::from(['a', 'b']))
    }

    #[test]
    fn parse_signal_patterns() {
        let actual = signal_patterns_parser("ab def ").unwrap().1;
        assert_eq!(
            actual,
            vec![BTreeSet::from(['a', 'b']), BTreeSet::from(['d', 'e', 'f']),]
        )
    }

    #[test]
    fn parse_line() {
        let actual = DisplayPanel::parse(include_str!("tiny.txt")).unwrap().1;
        assert_eq!(actual.signal_patterns[9], BTreeSet::from(['a', 'b']));
    }

    #[test]
    fn parse_display_panel() {
        DisplayPanel::parse_lines(include_str!("example.txt")).unwrap();
    }
}
