use crate::reduction::reduce;
use crate::tokenparser::parse_magnitude;
use nom::{bytes::complete::take_while, combinator::map_res, multi::many0, IResult};
use std::str::FromStr;

/// A linear representation of snailfish numbers.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    pub fn pop(&mut self) -> Option<Token> {
        match self.tokens.get(self.pos) {
            None => None,
            Some(token) => {
                self.pos += 1;
                Some(*token)
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Token {
    Open,
    Close,
    Num(u16),
    Comma,
}

impl Token {
    pub fn same_variant(t0: &Token, t1: &Token) -> bool {
        matches!(
            (t0, t1),
            (Token::Open, Token::Open)
                | (Token::Close, Token::Close)
                | (Token::Num(_), Token::Num(_))
                | (Token::Comma, Token::Comma)
        )
    }
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
        map(many0(p_any_token), TokenStream::new)(input)
    }

    pub fn magnitude(self) -> u16 {
        parse_magnitude(self)
    }
}

// Parse a sequence of digits into a number.
pub fn parse_number(input: &str) -> IResult<&str, u16> {
    map_res(take_while(|c: char| c.is_digit(10)), |input: &str| {
        input.parse()
    })(input)
}

#[cfg(test)]
impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.tokens.iter().map(|token| token.to_string()).collect();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
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
        let mut tokens = Vec::with_capacity(self.tokens.len() + rhs.tokens.len() + 3);
        tokens.push(Token::Open);
        tokens.extend(self.tokens);
        tokens.push(Token::Comma);
        tokens.extend(rhs.tokens);
        tokens.push(Token::Close);
        let ts = TokenStream::new(tokens);
        reduce(ts)
    }
}

impl std::ops::Add<&Self> for TokenStream {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let mut combined = Vec::with_capacity(self.tokens.len() + rhs.tokens.len());
        combined.push(Token::Open);
        combined.extend(self.tokens);
        combined.push(Token::Comma);
        combined.extend(rhs.tokens.iter().copied());
        combined.push(Token::Close);
        reduce(TokenStream::new(combined))
    }
}
