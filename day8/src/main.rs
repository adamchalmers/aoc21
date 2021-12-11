use parse::DisplayPanel;
use std::collections::BTreeSet;
mod deduction;
mod parse;

fn main() {
    let display_panels = DisplayPanel::parse_lines(include_str!("input.txt"))
        .unwrap()
        .1;
    let q1 = count_unique_len(&display_panels);
    println!("Q1: {}", q1);
    let q2: usize = display_panels.into_iter().map(deduction::solve).sum();
    println!("Q2: {}", q2);
}

fn count_unique_len(display_panels: &[DisplayPanel]) -> usize {
    let unique_len: BTreeSet<_> = BTreeSet::from([2, 3, 4, 7]);
    display_panels
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
        let display_panels = DisplayPanel::parse_lines(include_str!("example.txt"))
            .unwrap()
            .1;
        let actual = count_unique_len(&display_panels);
        let expected = 26;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve() {
        // Parse the file (which only has one line).
        let mut display_panels = DisplayPanel::parse_lines(include_str!("tiny.txt"))
            .unwrap()
            .1;
        assert_eq!(display_panels.len(), 1);
        let display_panel = display_panels.pop().unwrap();

        // Check the answer.
        let actual = deduction::solve(display_panel);
        assert_eq!(5353, actual);
    }
}
