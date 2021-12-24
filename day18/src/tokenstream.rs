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
            Token::Open => '[',
            Token::Close => ']',
            Token::Num(n) => return write!(f, "{}", n),
            Token::Comma => ',',
        };
        write!(f, "{}", s)
    }
}

/// To add two snailfish numbers, form a pair from the left and right parameters of the addition
/// operator. For example, [1,2] + [[3,4],5] becomes [[1,2],[[3,4],5]].
impl std::ops::Add for TokenStream {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut tokens = Vec::with_capacity(self.0.len() + rhs.0.len() + 3);
        tokens.push(Token::Open);
        tokens.extend(self.0);
        tokens.push(Token::Comma);
        tokens.extend(rhs.0);
        tokens.push(Token::Close);
        let ts = TokenStream(tokens);
        reduce(ts)
    }
}

impl std::ops::Add<&Self> for TokenStream {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let mut combined = Vec::with_capacity(self.0.len() + rhs.0.len());
        combined.push(Token::Open);
        combined.extend(self.0);
        combined.push(Token::Comma);
        combined.extend(rhs.0.iter().copied());
        combined.push(Token::Close);
        reduce(TokenStream(combined))
    }
}
