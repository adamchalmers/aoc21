use crate::parse::{DisplayPanel, Pattern};
use std::collections::{BTreeMap, BTreeSet};

pub fn solve(display: DisplayPanel) -> usize {
    // First, figure out which number each signal pattern corresponds to.
    // Begin by bucketing each pattern according to the number of segments.
    // Sometimes a number can be uniquely identified from the pattern (i.e. 1/4/7/8),
    // othertimes there are a few possibilities (e.g. 2, 3 and 5 all have the same number of segments).
    let mut one = Pattern::default();
    let mut seven = Pattern::default();
    let mut four = Pattern::default();
    let mut two_three_five = BTreeSet::<Pattern>::default();
    let mut zero_six_nine = BTreeSet::<Pattern>::default();
    let mut eight = Pattern::default();
    for pattern in display.signal_patterns {
        match pattern.len() {
            2 => one = pattern,
            3 => seven = pattern,
            4 => four = pattern,
            5 => {
                two_three_five.insert(pattern);
            }
            6 => {
                zero_six_nine.insert(pattern);
            }
            7 => eight = pattern,
            _ => unreachable!(),
        }
    }

    // Now we can start deducing which number each signal pattern represents.
    // There are three numbers with five segments (2, 3 and 5).
    // We can identify 3 because (2 and 5) jointly use all seven segments, but
    // neither (2 and 3) nor (5 and 3) do.
    let three = {
        let mut possibilities = two_three_five.iter();
        let p = possibilities.next().unwrap();
        let q = possibilities.next().unwrap();
        let r = possibilities.next().unwrap();
        if p.union(q).count() == 7 {
            r
        } else if q.union(r).count() == 7 {
            p
        } else {
            q
        }
    };
    // Nine is a superset of four's segments.
    let nine = zero_six_nine
        .iter()
        .find(|pattern| pattern.is_superset(&four))
        .unwrap();
    // Five is a subset of nine's segments
    let five = two_three_five
        .iter()
        .find(|p| *p != three && p.is_subset(nine))
        .unwrap();

    // We now have enough information to fill in the remaining numbers, and assign
    // each pattern to its actual numeric value.
    let map = BTreeMap::from([
        // Zero is a superset of seven, but six is not.
        (
            zero_six_nine
                .iter()
                .find(|p| p.is_superset(&seven) && *p != nine)
                .unwrap(),
            0_usize,
        ),
        (&one, 1),
        // Two is the only unknown five-segment pattern left.
        (
            two_three_five
                .iter()
                .find(|p| *p != five && *p != three)
                .unwrap(),
            2,
        ),
        (three, 3),
        (&four, 4),
        (five, 5),
        (
            zero_six_nine
                .iter()
                .find(|p| !p.is_superset(&seven) && *p != nine)
                .unwrap(),
            6,
        ),
        (&seven, 7),
        (&eight, 8),
        (nine, 9),
    ]);

    // Now that we know which pattern each number has, it's easy to calculate the
    // four digit output value.
    display
        .output_value
        .iter()
        .enumerate()
        .map(|(i, digit)| {
            let n = display.output_value.len();
            let base = 10_usize.pow((n - 1 - i) as u32);
            base * map[digit]
        })
        .sum()
}
