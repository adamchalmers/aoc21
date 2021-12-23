use crate::reduction::reduce;
use nom::{multi::many1, IResult};
use std::str::FromStr;

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
        let tree = Tree::Node {
            l: Box::new(self),
            r: Box::new(rhs),
        };
        let ts = TokenStream::try_from(tree).unwrap();
        let ts = reduce(ts);
        Tree::from_str(&ts.to_string()).unwrap()
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
    pub fn magnitude(&self) -> u16 {
        match self {
            Tree::Leaf(n) => *n as u16,
            // The magnitude of a pair is 3 times the magnitude of its left element
            // plus 2 times the magnitude of its right element.
            Tree::Node { l, r } => 3 * l.magnitude() + 2 * r.magnitude(),
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
                let parse_digits = take_while(|c: char| c.is_digit(10));
                let parse_u8 = map_res(parse_digits, u8::from_str);
                map(parse_u8, Token::Num)(input)
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
    fn magnitudes() {
        let tests = [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];
        for (input, expected_magnitude) in tests {
            let actual_magnitude = Tree::from_str(input).unwrap().magnitude();
            assert_eq!(actual_magnitude, expected_magnitude, "case {}", input);
        }
    }
}
