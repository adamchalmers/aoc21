#![feature(array_windows)]

use std::collections::HashMap;

fn main() {
    let mut problem = Problem::parse(include_str!("data/input.txt"));
    problem.apply_n(10);
    println!("Q1: {}", problem.q1());
}

struct Problem {
    polymer: Vec<char>,
    rules: Vec<Rule>,
}

impl Problem {
    fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        let polymer = lines.next().unwrap().chars().collect();
        lines.next();
        let rules = lines
            .map(|line| {
                let chars: Vec<_> = line.chars().collect();
                Rule {
                    input: (chars[0], chars[1]),
                    output: chars[6],
                }
            })
            .collect();
        Problem { polymer, rules }
    }

    fn count(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();
        for element in &self.polymer {
            *counts.entry(*element).or_insert(0) += 1;
        }
        counts
    }

    fn q1(&self) -> usize {
        let count = self.count();
        let most_common_qty = count.iter().map(|(_, v)| v).max().unwrap();
        let least_common_qty = count.iter().map(|(_, v)| v).min().unwrap();
        most_common_qty - least_common_qty
    }

    fn apply(&mut self) {
        let last = *self.polymer.last().unwrap();
        let mut next: Vec<_> = self
            .polymer
            .drain(0..)
            .as_slice()
            .array_windows()
            .flat_map(|pair @ [v0, v1]| {
                if let Some(rule) = self.rules.iter().find(|r| r.input == (*v0, *v1)) {
                    [*v0, rule.output].to_vec()
                } else {
                    pair.to_vec()
                }
            })
            .collect();
        next.push(last);
        self.polymer = next;
    }

    fn apply_n(&mut self, n: u16) {
        for _ in 0..n {
            self.apply()
        }
    }
}

#[derive(Debug)]
struct Rule {
    input: (char, char),
    output: char,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        let mut problem = Problem::parse(include_str!("data/example.txt"));
        problem.apply();
        assert_eq!(problem.polymer, "NCNBCHB".chars().collect::<Vec<_>>());
        problem.apply_n(3);
        assert_eq!(
            problem.polymer,
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
                .chars()
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_q1() {
        let mut problem = Problem::parse(include_str!("data/example.txt"));
        problem.apply_n(10);
        assert_eq!(problem.q1(), 1588);
    }
}
