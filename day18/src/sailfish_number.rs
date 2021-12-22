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

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Leaf(n) => n.to_string(),
            Self::Node { l, r } => format!("[{},{}]", l.to_string(), r.to_string()),
        };
        write!(f, "{}", s)
    }
}

impl Number {
    /// An inorder traversal yields leaves in the same left-to-right order they appear in the text representation.
    fn inorder(&self) -> Vec<u8> {
        match self {
            Self::Leaf(num) => vec![*num],
            Self::Node { l, r } => {
                let mut leaves = l.inorder();
                leaves.extend(r.inorder());
                leaves
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_and_printing_are_inverses() {
        let inputs = vec!["[[1,2],3]", "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"];
        for input in inputs {
            let (_, parsed) = Number::parse(input).unwrap();
            assert_eq!(parsed.to_string(), input);
        }
    }

    #[test]
    fn test_inorder() {
        let tests = [
            ("[1,2]", vec![1, 2]),
            ("[[1,2],3]", vec![1, 2, 3]),
            ("[[6,[5,[4,[3,2]]]],1]", vec![6, 5, 4, 3, 2, 1]),
        ];
        for (input, expected) in tests {
            let (_, parsed) = Number::parse(input).unwrap();
            assert_eq!(parsed.inorder(), expected);
        }
    }
}
