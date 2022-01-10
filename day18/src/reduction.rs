//! Snailfish reduction rules
use crate::tokenstream::*;

const EXPLODE_DEPTH: u8 = 4;
const SPLIT_SIZE: u16 = 10;

pub fn reduce(mut ts: TokenStream) -> TokenStream {
    while apply_explode(&mut ts) || apply_split(&mut ts) {}
    ts
}

/// Returns true if a number was split.
fn apply_split(ts: &mut TokenStream) -> bool {
    let (new_tokens, split_done) = ts.tokens.iter().fold(
        (Vec::with_capacity(ts.tokens.len()), false),
        |(mut new_tokens, mut split_done), token| {
            match token {
                Token::Num(n) if n >= &SPLIT_SIZE && !split_done => {
                    let (l, r) = split(*n);
                    new_tokens.extend([
                        Token::Open,
                        Token::Num(l),
                        Token::Comma,
                        Token::Num(r),
                        Token::Close,
                    ]);
                    split_done = true;
                }
                other => new_tokens.push(*other),
            }
            (new_tokens, split_done)
        },
    );
    ts.tokens = new_tokens;
    split_done
}

fn split(n: u16) -> (u16, u16) {
    // the left element of the pair should be the regular number divided by two
    // and rounded down
    let l = n / 2;
    // the right element of the pair should be the regular number divided by two
    // and rounded up.
    let r = (n + 2 - 1) / 2;
    (l, r)
}

/// Returns true if a pair was exploded.
fn apply_explode(ts: &mut TokenStream) -> bool {
    enum Explode {
        None,
        Carry(u16),
        Done,
    }

    let mut new_tokens = Vec::with_capacity(ts.tokens.len());
    let mut explode = Explode::None;
    let mut depth = 0u8;
    let mut i = 0;

    while i < ts.tokens.len() {
        let token = ts.tokens[i];
        match token {
            Token::Comma => {}
            Token::Open => depth += 1,
            Token::Close => depth -= 1,
            Token::Num(n) => match explode {
                Explode::Done => {}
                Explode::None => {
                    if depth > EXPLODE_DEPTH && ts.tokens[i + 1] == Token::Comma {
                        if let Token::Num(n_right) = ts.tokens[i + 2] {
                            explode = Explode::Carry(n_right);
                            add_to(&mut new_tokens, n);
                            let len = new_tokens.len();
                            new_tokens[len - 1] = Token::Num(0);
                            i += 4;
                            continue;
                        }
                    }
                }
                Explode::Carry(n_right) => {
                    new_tokens.push(Token::Num(n + n_right));
                    explode = Explode::Done;
                    i += 1;
                    continue;
                }
            },
        }
        // Advance the loop
        new_tokens.push(token);
        i += 1;
    }
    ts.tokens = new_tokens;
    !matches!(explode, Explode::None)
}

fn add_to(new_tokens: &mut [Token], n: u16) {
    // Go backwards through the new tokens until you find a number
    for j in (0..new_tokens.len()).rev() {
        if let Token::Num(m) = new_tokens[j] {
            // Add the left elem of exploding pair
            new_tokens[j] = Token::Num(m + n);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_apply_split() {
        let input_str = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
        let mut stream = TokenStream::from_str(input_str).unwrap();
        let expected = TokenStream::from_str("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]").unwrap();
        assert!(apply_split(&mut stream));
        assert_eq!(
            stream.to_string(),
            expected.to_string(),
            "input: {}",
            input_str
        )
    }

    #[test]
    fn test_split() {
        assert_eq!(split(10), (5, 5));
        assert_eq!(split(11), (5, 6));
        assert_eq!(split(12), (6, 6));
    }

    #[test]
    fn test_apply_explode() {
        let tests = [
            (
                "[[[[[9,8],1],2],3],4]",
                "[[[[0,9],2],3],4]",
                "(the 9 has no regular number to its left, so it is not added to any regular number)",
            ),
            (
                "[7,[6,[5,[4,[3,2]]]]]",
                "[7,[6,[5,[7,0]]]]",
                "the 2 has no regular number to its right, and so it is not added to any regular number"
            ),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]", ""),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "(the pair [3,2] is unaffected because the pair [7,3] is further to the left; [3,2] would explode on the next action)."
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
                "",
            ),
        ];
        for (input, expected, why) in tests {
            let mut ts = TokenStream::from_str(input).unwrap();
            let exploded = apply_explode(&mut ts);
            assert!(exploded);
            assert_eq!(ts.to_string(), expected, "input {}. {}", input, why);
        }
    }

    #[test]
    fn test_example() {
        let input = TokenStream::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let actual = reduce(input);
        assert_eq!(actual.to_string(), expected);
    }
}
