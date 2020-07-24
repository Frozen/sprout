#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
    BracketOpen,
    BracketClose,
}

pub fn tokenize(str: &str) -> Result<Vec<Token>, String> {
    let mut out: Vec<Token> = vec![];
    let mut iter = str.chars().into_iter();
    let mut pos = -1;
    loop {
        pos += 1;
        let v = iter.next();
        if v.is_none() {
            return Ok(out);
        }
        match v.unwrap() {
            ' ' => continue,
            'A' => out.push(Token::A),
            'B' => out.push(Token::B),
            'C' => out.push(Token::C),
            'H' => out.push(Token::H),
            'M' => out.push(Token::M),
            '!' => out.push(Token::Not),
            '=' => out.push(Token::Eq),
            '&' => {
                match iter.next() {
                    Some('&') => out.push(Token::And),
                    Some(t) => return Err(format!("unexpected char '{}' at pos {}", t, pos)),
                    None => return Err(format!("unexpected eof at pos {}", pos)),
                }
            },
            '>' => {
                    match out.pop() {
                        Some(Token::Eq) => out.push(Token::Arrow),
                        _ => return Err(format!("invalid value '{}' at pos {}", v.unwrap(), pos)),
                    }
                },

            _   => return Err(format!("invalid value '{}' at pos {}", v.unwrap(), pos)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lib::tokenize::tokenize;

    #[test]
    fn check_tokenize() {
        use super::Token::*;

        assert_eq!(Ok(vec![And]), tokenize("&&"));
        assert_eq!(Ok(vec![A, And, B]), tokenize("A && B "));
        assert_eq!(Ok(vec![Not, C, Arrow, H, Eq, M]), tokenize(" !C => H = M"));
        assert_eq!(Ok(vec![A, And, B, And, Not, C, Arrow, H, Eq, M]), tokenize("A && B && !C => H = M"));
    }
}