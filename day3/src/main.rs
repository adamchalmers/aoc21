fn main() {
    let lines: Vec<Vec<bool>> = read(include_str!("input.txt")).collect();
    println!("Q1: {}", power_usage(lines.iter()));

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

fn power_usage<'a, I>(mut lines: I) -> u32
where
    I: Iterator<Item = &'a Vec<bool>>,
{
    let sign = |bit: &bool| if *bit { 1 } else { -1 };
    let first_line_signs: Vec<i32> = lines.next().unwrap().iter().map(sign).collect();

    let summary = lines.fold(first_line_signs, |mut total_signs, binary| {
        for (column, bit) in binary.iter().enumerate() {
            total_signs[column] += sign(bit);
        }
        total_signs
    });
    let totals: Vec<_> = summary.iter().map(|&sign| sign > 0).collect();
    let (gamma, epsilon) = binary_to_number(&totals);
    gamma * epsilon
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
        let mut delta = 0;
        for line in &self.lines {
            if line[bit] {
                delta += 1;
            } else {
                delta -= 1;
            }
        }
        let criteria = if matches!(self.gas, Gas::Oxygen) {
            0 <= delta
        } else {
            0 > delta
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
        assert_eq!(power_usage(lines.iter()), 198);
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
