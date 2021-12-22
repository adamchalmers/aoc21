#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Number {
    Leaf(u8),
    Node { l: Box<Number>, r: Box<Number> },
}

/// To add two snailfish numbers, form a pair from the left and right parameters of the addition
/// operator. For example, [1,2] + [[3,4],5] becomes [[1,2],[[3,4],5]].
impl std::ops::Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Number::Node {
            l: Box::new(self),
            r: Box::new(rhs),
        }
    }
}

impl Number {
    /// Returns true if a reduction was applied.
    /// If any pair is nested inside four pairs, the leftmost such pair explodes
    /// To explode a pair, the pair's left value is added to the first regular number to the left of
    /// the exploding pair (if any), and the pair's right value is added to the first regular number
    /// to the right of the exploding pair (if any). Exploding pairs will always consist of two
    /// regular numbers. Then, the entire exploding pair is replaced with the regular number 0.
    fn apply_explode(&mut self, depth: u8) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explode() {
        let tests = [
            // the 9 has no regular number to its left, so it is not added to any regular number
            // ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ];
        for (str_in, str_out) in tests {
            let mut num = Number::parse(str_in).unwrap().1;
            let expected_out = Number::parse(str_out).unwrap().1;
            assert!(num.apply_explode(0));
            assert_eq!(num, expected_out)
        }
    }
}
