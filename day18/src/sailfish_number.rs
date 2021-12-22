use std::{num::ParseIntError, str::FromStr};

/// A hierarchical representation of sailfish numbers.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Tree {
    Leaf(u8),
    Node { l: Box<Tree>, r: Box<Tree> },
}

/// To add two snailfish numbers, form a pair from the left and right parameters of the addition
/// operator. For example, [1,2] + [[3,4],5] becomes [[1,2],[[3,4],5]].
impl std::ops::Add for Tree {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Tree::Node {
            l: Box::new(self),
            r: Box::new(rhs),
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Leaf(n) => n.to_string(),
            Self::Node { l, r } => format!("[{},{}]", l.to_string(), r.to_string()),
        };
        write!(f, "{}", s)
    }
}

impl Tree {
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

/// A linear representation of sailfish numbers.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Token {
    Open,
    Close,
    Num(u8),
    Comma,
}

impl From<Token> for char {
    fn from(t: Token) -> Self {
        match t {
            Token::Open => '[',
            Token::Close => ']',
            Token::Num(n) => char::from_digit(n as u32, 10).unwrap(),
            Token::Comma => ',',
        }
    }
}

struct TokenStream {
    tokens: Vec<Token>,
}

impl FromStr for TokenStream {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s
            .chars()
            .map(|ch| match ch {
                '[' => Ok(Token::Open),
                ']' => Ok(Token::Close),
                ',' => Ok(Token::Comma),
                n => Ok(Token::Num(String::from(n).parse()?)),
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { tokens })
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.tokens.iter().map(|token| char::from(*token)).collect();
        println!("STRING: {}", s);
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_representations_are_equivalent() {
        let inputs = vec!["[[1,2],3]", "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"];
        for input in inputs {
            // Parsing strings into and out of Trees
            let (_, tree) = Tree::parse(input).unwrap();
            let s1 = tree.to_string();
            assert_eq!(s1, input);
            // Parsing strings into and out of TokenStreams
            let stream = TokenStream::from_str(&s1).unwrap();
            assert_eq!(s1, stream.to_string());
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
            let (_, parsed) = Tree::parse(input).unwrap();
            assert_eq!(parsed.inorder(), expected);
        }
    }
}
