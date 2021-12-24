use crate::magnitude::{parse_magnitude, parse_number};
use crate::reduction::reduce;
use nom::multi::many0;
use std::str::FromStr;

/// A linear representation of snailfish numbers.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TokenStream(pub Vec<Token>);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Token {
    Open,
    Close,
    Num(u16),
    Comma,
}

impl FromStr for TokenStream {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = Self::parse(s).map_err(|e| e.to_string())?;
        Ok(t.1)
    }
}

impl TokenStream {
    /// Parse the token stream out of a string.
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::{branch::alt, character::complete::char, combinator::map};

        // Parsers for each of the 4 token types
        let p_open = map(char('['), |_| Token::Open);
        let p_close = map(char(']'), |_| Token::Close);
        let p_comma = map(char(','), |_| Token::Comma);
        let p_num = map(parse_number, Token::Num);

        // Parse the token stream.
        let p_any_token = alt((p_open, p_close, p_num, p_comma));
        map(many0(p_any_token), TokenStream)(input)
    }

    pub fn magnitude(self) -> u16 {
        parse_magnitude(&self.to_string())
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.0.iter().map(|token| token.to_string()).collect();
        write!(f, "{}", s)
    }
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

/// To add two snailfish numbers, form a pair from the left and right parameters of the addition
/// operator. For example, [1,2] + [[3,4],5] becomes [[1,2],[[3,4],5]].
impl std::ops::Add for TokenStream {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let s = ["[", &self.to_string(), ",", &rhs.to_string(), "]"].join("");
        reduce(TokenStream::from_str(&s).unwrap())
    }
}
