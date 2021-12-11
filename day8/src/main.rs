use parse::Observation;
use std::collections::HashSet;
mod parse;

fn main() {
    let observations = Observation::parse_lines(include_str!("input.txt"))
        .unwrap()
        .1;
    let q1 = count_unique_len(&observations);
    println!("Q1: {}", q1);
    // We can figure out which wire controls segment A by doing segments(7) - segments(1).
}

fn count_unique_len(observations: &[Observation]) -> usize {
    let unique_len: HashSet<_> = HashSet::from([2, 3, 4, 7]);
    observations
        .iter()
        .map(|p| {
            p.output_value
                .iter()
                .filter(|segs| unique_len.contains(&segs.len()))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_unique_len() {
        let observations = Observation::parse_lines(include_str!("example.txt"))
            .unwrap()
            .1;
        let actual = count_unique_len(&observations);
        let expected = 26;
        assert_eq!(actual, expected);
    }
}
