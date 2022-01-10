use crate::tokenstream::*;
use nom::{branch::alt, combinator::map, error::Error, sequence::tuple, IResult, Parser};

/// Matches the given token.
struct TokenParser(Token);

impl Parser<TokenStream, Token, Error<TokenStream>> for TokenParser {
    fn parse(&mut self, mut input: TokenStream) -> nom::IResult<TokenStream, Token> {
        match input.pop() {
            Some(token) if Token::same_variant(&self.0, &token) => Ok((input, token)),
            _ => Err(nom::Err::Error(Error {
                input,
                code: nom::error::ErrorKind::Char,
            })),
        }
    }
}

/// Matches Token::Number and returns the number.
struct TokenNumParser;

impl Parser<TokenStream, u16, Error<TokenStream>> for TokenNumParser {
    fn parse(&mut self, mut input: TokenStream) -> nom::IResult<TokenStream, u16> {
        match input.pop() {
            Some(Token::Num(n)) => Ok((input, n)),
            _ => Err(nom::Err::Error(Error {
                input,
                code: nom::error::ErrorKind::Digit,
            })),
        }
    }
}

// Parse a stringified Snailfish pair, adding up its magnitude.
fn magnitude_of_pair(input: TokenStream) -> IResult<TokenStream, u16> {
    let parser = tuple((
        TokenParser(Token::Open),
        magnitude,
        TokenParser(Token::Comma),
        magnitude,
        TokenParser(Token::Close),
    ));
    let mut discard_delimiter_parser = map(parser, |(_, l, _, r, _)| 3 * l + 2 * r);
    discard_delimiter_parser(input)
}

// Parse a stringified Snailfish number, adding up its magnitude.
fn magnitude(input: TokenStream) -> IResult<TokenStream, u16> {
    // An element can be either a pair of elements, or a number literal.
    alt((magnitude_of_pair, TokenNumParser))(input)
}

/// Iterate over a Snailfish number, adding up its magnitude as you go.
pub fn parse_magnitude(input: TokenStream) -> u16 {
    magnitude(input).unwrap().1
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

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
            let tokens = TokenStream::from_str(input).unwrap();
            let actual_magnitude = parse_magnitude(tokens);
            assert_eq!(actual_magnitude, expected_magnitude, "case {}", input);
        }
    }
}
