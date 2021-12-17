#![feature(array_windows)]

use std::collections::{HashMap, HashSet};

type Pair = (char, char);

fn main() {
    let mut problem = Problem::parse(include_str!("data/input.txt"));
    problem.apply_n(10);
    println!("Q1: {}", problem.q1());
    problem.apply_n(30);
    println!("Q1: {}", problem.q1());
}

struct Problem {
    /// The polymer is represented as frequencies of each pair of elements.
    pair_counts: HashMap<Pair, usize>,
    /// Polymerization rules from the problem match_pair.
    rules: HashSet<Rule>,
    last: char,
}

impl Problem {
    fn parse(s: &str) -> Self {
        let mut lines = s.lines();

        // Parse the starting polymer.
        let polymer: Vec<_> = lines.next().unwrap().chars().collect();
        let last = polymer.last().unwrap().to_owned();
        let mut pair_counts = HashMap::new();
        for [ch1, ch2] in polymer.as_slice().array_windows() {
            *pair_counts.entry((*ch1, *ch2)).or_insert(0) += 1;
        }

        // Parse the rules.
        lines.next();
        let rules = lines
            .map(|line| {
                let chars: Vec<_> = line.chars().collect();
                Rule {
                    match_pair: (chars[0], chars[1]),
                    addition: chars[6],
                }
            })
            .collect();
        Problem {
            pair_counts,
            rules,
            last,
        }
    }

    /// How many times does each element occur in the polymer?
    fn count_elements(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();
        // For each pair, add up all the times the left element appears.
        for ((l, _r), count) in &self.pair_counts {
            *counts.entry(*l).or_insert(0) += count;
        }
        // This left out the last element, so add it.
        *counts.entry(self.last).or_insert(0) += 1;
        counts
    }

    fn q1(&self) -> usize {
        let count = self.count_elements();
        let most_common_qty = count.iter().map(|(_, v)| v).max().unwrap();
        let least_common_qty = count.iter().map(|(_, v)| v).min().unwrap();
        most_common_qty - least_common_qty
    }

    /// Apply one step of polymerization.
    fn apply(&mut self) {
        let mut next_pair_counts = HashMap::new();
        for (pair @ (elem0, elem1), qty) in self.pair_counts.drain() {
            let out = self
                .rules
                .iter()
                .find(|r| r.match_pair == pair)
                .unwrap()
                .addition;
            *next_pair_counts.entry((elem0, out)).or_insert(0) += qty;
            *next_pair_counts.entry((out, elem1)).or_insert(0) += qty;
        }
        self.pair_counts = next_pair_counts;
    }

    /// Apply n steps of polymerization.
    fn apply_n(&mut self, n: u16) {
        for _ in 0..n {
            self.apply()
        }
    }
}

/// One polymerization rule, which inserts the addition element between a pair of the match_pair elements.
#[derive(Debug, Hash, PartialEq, Eq)]
struct Rule {
    /// The two elements this rule matches on.
    match_pair: (char, char),
    /// The new element to insert between the matching pair.
    addition: char,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        let mut problem = Problem::parse(include_str!("data/example.txt"));
        let expected_t0 = HashMap::from([(('N', 'N'), 1), (('N', 'C'), 1), (('C', 'B'), 1)]);
        assert_eq!(
            problem.pair_counts, expected_t0,
            "Problem::parse was incorrect"
        );

        problem.apply();

        let expected_t1 = HashMap::from([
            (('N', 'C'), 1),
            (('C', 'N'), 1),
            (('N', 'B'), 1),
            (('B', 'C'), 1),
            (('C', 'H'), 1),
            (('H', 'B'), 1),
        ]);
        assert_eq!(
            problem.pair_counts, expected_t1,
            "Problem::apply was incorrect"
        );
    }

    #[test]
    fn test_q1() {
        let mut problem = Problem::parse(include_str!("data/example.txt"));
        assert_eq!(problem.q1(), 1);
        problem.apply_n(10);
        /* After step 10,
        B occurs 1749 times,
        C occurs 298 times,
        H occurs 161 times, and
        N occurs 865 times
        */
        assert_eq!(
            problem.count_elements(),
            HashMap::from([('B', 1749), ('C', 298), ('H', 161), ('N', 865),])
        );
        assert_eq!(problem.q1(), 1588);
    }

    #[test]
    fn test_q1_real() {
        let mut problem = Problem::parse(include_str!("data/input.txt"));
        problem.apply_n(10);
        assert_eq!(problem.q1(), 2891);
    }

    #[test]
    fn test_q2() {
        let mut problem = Problem::parse(include_str!("data/example.txt"));
        /* In the above example, the most common element is
        B (occurring 2192039569602 times) and the least common element is
        H (occurring 3849876073 times);
        subtracting these produces 2188189693529.
        */
        problem.apply_n(40);
        let counts = problem.count_elements();
        assert_eq!(counts[&'B'], 2192039569602);
        assert_eq!(counts[&'H'], 3849876073);
        assert_eq!(problem.q1(), 2188189693529);
    }
}
