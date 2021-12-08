mod counter;
mod lines;
mod parse;
use counter::*;
use lines::*;

fn main() {
    let lines = parse::parse_input(include_str!("input.txt"));
    let (q1, q2) = solve(&lines);
    println!("Q1: {}", q1);
    println!("Q2: {}", q2);
}

fn solve(lines: &[Line]) -> (usize, usize) {
    let mut straight_counter = Counter::default();
    let mut all_counter = Counter::default();
    for l in lines {
        for p in l.points_in_line() {
            if l.is_straight() {
                straight_counter.add(p);
            }
            all_counter.add(p);
        }
    }
    (straight_counter.count_ge(2), all_counter.count_ge(2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_example() {
        let lines: Vec<_> = parse::parse_input(include_str!("example.txt"));
        let (q1, q2) = solve(&lines);
        assert_eq!(q1, 5);
        assert_eq!(q2, 12);
    }
}
