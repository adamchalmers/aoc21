use std::{num::ParseIntError, str::FromStr};

use nom::{multi::many1, IResult};

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
pub enum Token {
    Open,
    Close,
    Num(u8),
    Comma,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Token::Open => "[".to_owned(),
            Token::Close => "]".to_owned(),
            Token::Num(n) => n.to_string(),
            Token::Comma => ",".to_owned(),
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<Tree> for TokenStream {
    type Error = String;

    fn try_from(tree: Tree) -> Result<Self, Self::Error> {
        TokenStream::from_str(&tree.to_string())
    }
}

impl TryFrom<TokenStream> for Tree {
    type Error = String;

    fn try_from(ts: TokenStream) -> Result<Self, Self::Error> {
        let s = ts.to_string();
        Tree::from_str(&s)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
}

impl FromStr for TokenStream {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(i: &str) -> nom::IResult<&str, Vec<Token>> {
            use nom::{
                branch::alt, bytes::complete::take_while, character::complete::char,
                combinator::map, combinator::map_res,
            };

            let p_open = map(char('['), |_| Token::Open);
            let p_close = map(char(']'), |_| Token::Close);
            let p_comma = map(char(','), |_| Token::Comma);

            pub fn p_num(input: &str) -> IResult<&str, Token> {
                let parse_u8 = map_res(take_while(|c: char| c.is_digit(10)), |input| {
                    u8::from_str(input)
                });
                let mut p = map(parse_u8, |n| Token::Num(n));
                p(input)
            }

            let p = alt((p_open, p_close, p_num, p_comma));
            let mut p = many1(p);
            p(i)
        }
        parse(s)
            .map(|(_, tokens)| TokenStream { tokens })
            .map_err(|e| e.to_string())
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.tokens.iter().map(|token| token.to_string()).collect();
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
            let tree = Tree::from_str(input).unwrap();
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
            let parsed = Tree::from_str(input).unwrap();
            assert_eq!(parsed.inorder(), expected);
        }
    }
}
