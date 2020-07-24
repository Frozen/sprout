use anyhow::Result;
use crate::lib::tokenize::Token::Const;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    M,
    P,
    T,
    K,
    Not,
    Arrow,
    Eq,
    And,
    Plus,
    Minus,
    Open,
    Close,
    Multiple,
    Divide,
    Const(f64),
}

pub fn tokenize(str: &str) -> Result<Vec<Token>> {
    let mut out: Vec<Token> = vec![];
    let iter: Vec<char> = str.chars().collect();
    let mut pos: usize = 0;
    loop {
        let v = iter.get(pos);
        if v.is_none() {
            return Ok(out);
        }
        match v.unwrap() {
            ' ' => { pos += 1; continue},
            'A' => out.push(Token::A),
            'B' => out.push(Token::B),
            'C' => out.push(Token::C),
            'D' => out.push(Token::D),
            'F' => out.push(Token::F),
            'E' => out.push(Token::E),
            'H' => out.push(Token::H),
            'M' => out.push(Token::M),
            'K' => out.push(Token::K),
            'P' => out.push(Token::P),
            'T' => out.push(Token::T),
            '!' => out.push(Token::Not),
            '=' => out.push(Token::Eq),
            '+' => out.push(Token::Plus),
            '-' => out.push(Token::Minus),
            '*' => out.push(Token::Multiple),
            '/' => out.push(Token::Divide),
            '(' => out.push(Token::Open),
            ')' => out.push(Token::Close),
            '0'..='9'  => {
                let mut str = String::new();
                str.push(iter[pos]);
                loop {
                    if let None = iter.get(pos + 1) {
                        break
                    }
                    match iter[pos + 1] {
                        '0'..='9' | '.' => {
                            str.push(iter[pos + 1])
                        },
                        _ => break,
                    }
                    pos += 1;
                }
                out.push(Const(str.parse::<f64>()?))
            },
            '&' => {
                pos += 1;
                match iter.get(pos) {
                    Some('&') => out.push(Token::And),
                    Some(t) => return Err(anyhow!("unexpected char '{}' at pos {}", t, pos)),
                    None => return Err(anyhow!("unexpected eof at pos {}", pos)),
                }
            },
            '>' => {
                    match out.pop() {
                        Some(Token::Eq) => out.push(Token::Arrow),
                        _ => return Err(anyhow!("invalid value '{}' at pos {}", v.unwrap(), pos)),
                    }
                },

            _   => return Err(anyhow!("tokenize: invalid value '{}' at pos {}", v.unwrap(), pos)),
        }
        pos += 1;
    }
}

#[cfg(test)]
mod test {
    use crate::lib::tokenize::tokenize;

    #[test]
    fn check_tokenize() {
        use super::Token::*;

        assert_eq!(vec![And], tokenize("&&").unwrap());
        assert_eq!(vec![A, And, B], tokenize("A && B ").unwrap());
        assert_eq!(vec![Not, C, Arrow, H, Eq, M], tokenize(" !C => H = M").unwrap());
        assert_eq!(vec![A, And, B, And, Not, C, Arrow, H, Eq, M], tokenize("A && B && !C => H = M").unwrap());
    }

    #[test]
    fn check_tokenize2() {
        use super::Token::*;
        assert_eq!(vec![H, Eq, P, Arrow, K, Eq, D, Plus, Open, D, Multiple,
                        Open, E, Minus, F, Close, Divide, Const(25.5), Close],
                   tokenize("H = P => K = D + (D * (E - F) / 25.5)").unwrap());
    }
}