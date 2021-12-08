fn main() {
    let crabs = parse_positions(include_str!("input.txt"));
    println!("Q1: {}", best_position(&crabs, |n| n));
    println!("Q2: {}", best_position(&crabs, triangle_num));
}

fn parse_positions(s: &str) -> Vec<i32> {
    s.split(',').map(|s| s.parse().unwrap()).collect()
}

fn best_position<F>(crabs: &[i32], cost_fn: F) -> i32
where
    F: Fn(i32) -> i32 + Clone,
{
    let max_position = *crabs.iter().max().unwrap();
    (0..max_position)
        .map(|target_position| {
            crabs
                .iter()
                .map(|position| {
                    let distance = (target_position - position).abs();
                    cost_fn(distance)
                })
                .sum()
        })
        .min()
        .unwrap()
}

fn triangle_num(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1() {
        let crabs = parse_positions(include_str!("example.txt"));
        assert_eq!(best_position(&crabs, |n| n), 37)
    }

    #[test]
    fn test_triangle() {
        for (i, actual) in [0, 1, 3, 6].into_iter().enumerate() {
            assert_eq!(triangle_num(i as i32), actual);
        }
    }

    #[test]
    fn test_q2() {
        let crabs = parse_positions(include_str!("example.txt"));
        assert_eq!(best_position(&crabs, triangle_num), 168)
    }
}
