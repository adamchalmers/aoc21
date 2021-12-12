const OPENS: [char; 4] = ['<', '(', '{', '['];
const CLOSES: [char; 4] = ['>', ')', '}', ']'];

fn main() {
    let lines: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|l| check_line(l.chars(), Vec::new()))
        .collect();
    println!("Q1: {}", score_illegal_syntax(&lines));
    println!("Q2: {}", score_incomplete(lines));
}

#[derive(PartialEq, Eq, Debug)]
enum Status {
    Ok,
    Incomplete(Vec<char>),
    Illegal(char),
}

fn check_line(mut line: impl Iterator<Item = char>, mut stack: Vec<char>) -> Status {
    // Get the current character that needs procesing.
    let curr = match line.next() {
        Some(curr) => curr,
        None if stack.is_empty() => return Status::Ok,
        None => return Status::Incomplete(stack),
    };
    let curr_is_opener = OPENS.iter().any(|&c| c == curr);
    if curr_is_opener {
        stack.push(curr);
        check_line(line, stack)
    } else {
        match stack.pop() {
            None => Status::Illegal(curr), // trying to close but nothing is open
            Some(top) => {
                // Does curr match top of stack?
                let open_pos = OPENS.iter().position(|&c| c == top).unwrap();
                let close_pos = CLOSES.iter().position(|&c| c == curr).unwrap();
                if open_pos == close_pos {
                    check_line(line, stack)
                } else {
                    Status::Illegal(curr)
                }
            }
        }
    }
}

fn score_illegal_syntax(lines: &[Status]) -> u32 {
    lines
        .iter()
        .map(|status| match status {
            Status::Illegal(c) => syntax_error_score(*c) as u32,
            _ => 0,
        })
        .sum()
}

fn score_incomplete(lines: Vec<Status>) -> u64 {
    let mut all_scores: Vec<_> = lines
        .into_iter()
        .filter_map(|status| match status {
            Status::Incomplete(unmatched) => Some(incomplete_line_score(unmatched)),
            _ => None,
        })
        .collect();
    all_scores.sort_unstable();
    all_scores[all_scores.len() / 2]
}

fn syntax_error_score(c: char) -> u16 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}
fn incomplete_line_score(mut chars: Vec<char>) -> u64 {
    chars.reverse();
    chars.iter().fold(0, |mut score, char| {
        score *= 5;
        score
            + match char {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_line() {
        let line = "{([(<{}[<>[]}>{[]{[(<()>";
        let expected = Status::Illegal('}');
        let actual = check_line(line.chars(), Vec::new());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_q1() {
        let lines: Vec<_> = include_str!("example.txt")
            .lines()
            .map(|l| check_line(l.chars(), Vec::new()))
            .collect();
        assert_eq!(26397, score_illegal_syntax(&lines));
    }

    #[test]
    fn test_q2() {
        let lines: Vec<_> = include_str!("example.txt")
            .lines()
            .map(|l| check_line(l.chars(), Vec::new()))
            .collect();
        assert_eq!(288957, score_incomplete(lines));
    }
}
