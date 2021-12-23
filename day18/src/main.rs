mod parse;
mod reduction;
mod sailfish_number;

use sailfish_number::Tree;

fn main() {
    let sum = homework_q1(include_str!("data/input.txt"));
    println!("Q1: {}", sum.magnitude());
    println!("Q2: {}", homework_q2(include_str!("data/input.txt")));
}

fn homework_q1(s: &str) -> Tree {
    Tree::parse_many(s)
        .expect("could not parse input file")
        .1
        .into_iter()
        .reduce(|sum, item| sum + item)
        .unwrap()
}

fn homework_q2(s: &str) -> u16 {
    let nums = Tree::parse_many(s).expect("could not parse input file").1;
    nums.iter()
        .flat_map(|num0| {
            nums.iter().map(|num1| {
                let t = num0.clone() + num1.clone();
                t.magnitude()
            })
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homework_tiny() {
        let actual = homework_q1(include_str!("data/example_tiny.txt")).to_string();
        let expected = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_homework_small() {
        let actual = homework_q1(include_str!("data/example_small.txt")).to_string();
        let expected = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_homework_large() {
        let actual = homework_q1(include_str!("data/example_large.txt")).to_string();
        let expected = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
        assert_eq!(actual, expected);
    }
}
