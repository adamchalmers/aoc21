fn main() {
    let lines: Vec<Vec<bool>> = read(include_str!("input.txt")).collect();
    let summary = BinarySummary::ingest(lines.iter());
    println!("Q1: {}", summary.power_usage());

    let co2 = Q2 {
        lines: lines.clone(),
        gas: Gas::CO2,
    }
    .solve();
    let oxygen = Q2 {
        lines,
        gas: Gas::Oxygen,
    }
    .solve();
    println!("Q2: {}", oxygen * co2);
}

fn read(s: &str) -> impl Iterator<Item = Vec<bool>> + '_ {
    s.lines().map(|s| s.chars().map(|c| c != '0').collect())
}

#[derive(Default)]
struct BinarySummary(Vec<bool>);

impl BinarySummary {
    fn ingest<'a, I>(mut lines: I) -> Self
    where
        I: Iterator<Item = &'a Vec<bool>>,
    {
        let first: Vec<i32> = lines.next().unwrap().iter().map(|_| 0).collect();

        let summary = lines.fold(first, |mut acc, binary| {
            for (column, bit) in binary.iter().enumerate() {
                acc[column] += if *bit { 1 } else { -1 };
            }
            acc
        });
        BinarySummary(summary.iter().map(|&i| i > 0).collect())
    }

    fn power_usage(&self) -> u32 {
        let (gamma, epsilon) = binary_to_number(&self.0);
        gamma * epsilon
    }
}

fn binary_to_number(bits: &[bool]) -> (u32, u32) {
    let mut number = 0;
    let mut inverse = 0;
    for i in (0..bits.len()).rev() {
        let base = 2_u32.pow((bits.len() - i - 1).try_into().unwrap());
        if bits[i] {
            number += base;
        } else {
            inverse += base;
        }
    }
    (number, inverse)
}

struct Q2 {
    lines: Vec<Vec<bool>>,
    gas: Gas,
}

enum Gas {
    Oxygen,
    CO2,
}

impl Q2 {
    fn solve(mut self) -> u32 {
        let mut bit = 0;
        loop {
            self.apply_bit_criteria(bit);
            bit += 1;
            if self.lines.len() <= 1 {
                break;
            }
        }
        let last_line_remaining = self.lines.into_iter().next().unwrap();
        binary_to_number(&last_line_remaining).0
    }

    fn apply_bit_criteria(&mut self, bit: usize) {
        // Find the criteria
        let mut zeros = 0;
        let mut ones = 0;
        for line in &self.lines {
            if line[bit] {
                ones += 1;
            } else {
                zeros += 1;
            }
        }
        let criteria = if matches!(self.gas, Gas::Oxygen) {
            zeros <= ones
        } else {
            zeros > ones
        };
        // Apply the bit criteria
        self.lines.retain(|line| line[bit] == criteria);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1() {
        let lines: Vec<Vec<bool>> = read(include_str!("example.txt")).collect();
        let summary = BinarySummary::ingest(lines.iter());
        assert_eq!(summary.power_usage(), 198);
    }

    #[test]
    fn test_q2() {
        let lines: Vec<Vec<bool>> = read(include_str!("example.txt")).collect();
        assert_eq!(
            23,
            Q2 {
                lines: lines.clone(),
                gas: Gas::Oxygen,
            }
            .solve()
        );
        assert_eq!(
            10,
            Q2 {
                lines,
                gas: Gas::CO2,
            }
            .solve()
        );
    }
}
