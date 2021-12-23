use crate::sailfish_number::*;

pub fn reduce(ts: TokenStream) -> TokenStream {
    let mut curr = ts;
    loop {
        // println!("{}", curr);
        let exploded = apply_explode(&curr);
        if exploded != curr {
            curr = exploded;
            continue;
        }
        let split = apply_split(&curr);
        if split != curr {
            curr = split;
            continue;
        }
        break;
    }
    curr
}

fn apply_split(ts: &TokenStream) -> TokenStream {
    let mut new_tokens = Vec::new();
    let mut split_done = false;
    for token in &ts.tokens {
        match token {
            Token::Num(n) if n >= &10 && !split_done => {
                let (l, r) = split(*n);
                new_tokens.push(Token::Open);
                // the left element of the pair should be the regular number divided by two
                // and rounded down
                new_tokens.push(Token::Num(l));
                new_tokens.push(Token::Comma);
                // the right element of the pair should be the regular number divided by two
                // and rounded up.
                new_tokens.push(Token::Num(r));
                new_tokens.push(Token::Close);
                split_done = true;
            }
            other => new_tokens.push(*other),
        }
    }
    TokenStream { tokens: new_tokens }
}

fn split(n: u8) -> (u8, u8) {
    let l = n / 2;
    let r = (n + 2 - 1) / 2;
    (l, r)
}

enum Explode {
    None,
    Carry(u8),
    Done,
}

fn apply_explode(ts: &TokenStream) -> TokenStream {
    let mut new_tokens = Vec::new();
    let mut explode = Explode::None;
    let mut depth = 0u16;
    let mut i = 0;

    while i < ts.tokens.len() {
        let token = ts.tokens[i];
        match token {
            t @ Token::Open => {
                depth += 1;
                new_tokens.push(t);
                i += 1;
            }
            t @ Token::Close => {
                depth -= 1;
                new_tokens.push(t);
                i += 1;
            }
            t @ Token::Num(n) => {
                match explode {
                    Explode::None => {
                        if depth >= 5 && ts.tokens[i + 1] == Token::Comma {
                            if let Token::Num(n_right) = &ts.tokens[i + 2] {
                                explode = Explode::Carry(*n_right);
                                // Go backwards through the new tokens until you find a number
                                for j in (0..new_tokens.len()).rev() {
                                    if let Token::Num(m) = new_tokens[j] {
                                        // Add the left elem of exploding pair
                                        new_tokens[j] = Token::Num(m + n);
                                        break;
                                    }
                                }
                                i += 4;
                                new_tokens.pop();
                                new_tokens.push(Token::Num(0));
                                continue;
                            }
                        }
                        i += 1;
                        new_tokens.push(t);
                    }
                    Explode::Carry(n_right) => {
                        new_tokens.push(Token::Num(n + n_right));
                        explode = Explode::Done;
                        i += 1;
                    }
                    Explode::Done => {
                        i += 1;
                        new_tokens.push(t);
                    }
                }
            }
            t @ Token::Comma => {
                new_tokens.push(t);
                i += 1;
            }
        }
    }
    TokenStream { tokens: new_tokens }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_apply_split() {
        let input_str = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
        let input = TokenStream::from_str(input_str).unwrap();
        let expected = TokenStream::from_str("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]").unwrap();
        let actual = apply_split(&input);
        assert_eq!(
            actual.to_string(),
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
        for (input, expected_exploded, why) in tests {
            let ts = TokenStream::from_str(input).unwrap();
            let actual_exploded = apply_explode(&ts);
            assert_eq!(
                actual_exploded.to_string(),
                expected_exploded,
                "input {}. {}",
                input,
                why
            );
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
