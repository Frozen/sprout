use crate::lib::tokenize::Token;
use crate::lib::scope::Scope;
use anyhow::Result;
use crate::lib::output;

#[derive(Debug, Clone)]
pub struct Base2Expr {
    output: output::H,
    original: Vec<Token>,
    executable: Executable,
}

impl PartialEq for Base2Expr {
    fn eq(&self, other: &Self) -> bool {
        self.executable == other.executable
    }
}

impl Base2Expr {
    pub fn run(&self, s: Scope) -> Result<f64> {
        self.executable.run(s)
    }

    pub fn build(tokens: &[Token]) -> Result<Base2Expr> {
        let pos = tokens.iter().position(|&x| x == Token::Arrow);
        if pos.is_none() {
            return Err(anyhow!("Invalid tokens, no `Arrow` token found"));
        }
        let pos = pos.unwrap();
        let (left, _right) = tokens.split_at(pos);
        let output = match left.get(2) {
            Some(Token::M) => output::H::M,
            Some(Token::P) => output::H::P,
            Some(Token::T) => output::H::T,
            Some(t) => return Err(anyhow!("Invalid tokens, expected one on [M, P, T], found {:?} found", t)),
            None => return Err(anyhow!("Invalid tokens, expected eof"))
        };

        match (tokens.get(pos + 1), tokens.get(pos + 2)) {
            (Some(Token::K), Some(Token::Eq)) => {
                let exec = Executable::build(&tokens[pos+3..])?;
                Ok(Base2Expr {
                    original: tokens.to_vec(),
                    executable: exec,
                    output,
                })
            },
            _ => return Err(anyhow!("Invalid tokens"))
        }
    }

    pub fn output(&self) -> output::H {
        self.output
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Executable {
    inner: Vec<Token>,
}

impl Executable {
    pub fn build(tokens: &[Token]) -> Result<Executable> {
        let mut stack = vec![];
        let mut out = vec![];

        for t in tokens {
            match t {
                Token::D => {
                    out.push(Token::D)
                },
                Token::E => out.push(Token::E),
                Token::F => out.push(Token::F),
                Token::Plus => stack.push(Token::Plus),
                Token::Minus => stack.push(Token::Minus),
                Token::Open => stack.push(Token::Open),
                Token::Close => {
                    while let Some(t) = stack.pop() {
                        if t == Token::Open {
                            break;
                        }
                        out.push(t);
                    }
                }
                Token::Multiple => {
                    stack.push(Token::Multiple)
                }
                Token::Divide => {
                    stack.push(Token::Divide)
                },
                Token::Const(v) => {
                    out.push(Token::Const(*v))
                },
                t => return Err(anyhow!("unexpected token {:?}", t))
            }
        }

        while stack.len() > 0 {
            out.push(stack.pop().unwrap());
        }

        return Ok(Executable {
            inner: out,
        })
    }

    pub fn run(&self, s: Scope) -> Result<f64> {
        let mut stack = vec![];
        for (i, t) in self.inner.iter().enumerate() {
            match t {
                Token::D => stack.push(s.d),
                Token::E => stack.push(s.e as f64),
                Token::F => stack.push(s.f as f64),
                Token::Const(v) => stack.push(*v),
                Token::Plus => {
                    let first = stack.pop().unwrap();
                    let second = stack.pop().unwrap();
                    stack.push(first + second);
                },
                Token::Minus => {
                    let second = stack.pop().unwrap();
                    let first = stack.pop().unwrap();
                    stack.push(first - second);
                },
                Token::Multiple => {
                    let second = stack.pop().unwrap();
                    let first = stack.pop().unwrap();
                    stack.push(first * second);
                },
                Token::Divide => {
                    let second = stack.pop().unwrap();
                    let first = stack.pop().unwrap();
                    if second == 0.0 {
                        return Err(anyhow!("Zero division"));
                    }
                    stack.push(first / second);
                },
                _ => return Err(anyhow!("InvalidToken {:?} at pos {}", *t, i))
            }
        }
        return Ok(stack.pop().unwrap());
    }
}


#[cfg(test)]
mod test {
    use super::{Executable};
    use crate::lib::scope::Scope;
    use crate::lib::base2::Base2Expr;
    use crate::lib::output;

    #[test]
    fn test_build_executable() {
        use super::super::tokenize::Token::*;;
        let values = vec![
            (vec![D, Plus, F],
             vec![D, F, Plus]),
            (vec![D, Multiple, F, Divide, Const(25.0)],
             vec![D, F, Const(25.0), Divide, Multiple]),
            (vec![Open, D, Plus, E, Close, Multiple, E],
             vec![D, E, Plus, E, Multiple]),
            (vec![D, Plus, Open, D, Multiple, E , Divide, Const(10.0), Close],
             vec![D, D, E, Const(10.0), Divide, Multiple, Plus]),
            (vec![D, Plus, Open, D, Multiple, Open,  E, Minus, F, Close,  Divide, Const( 25.5), Close],
             vec![D, D, E, F, Minus, Const(25.5), Divide, Multiple, Plus])
        ];

        for v in values {
            let out = Executable::build(&v.0).unwrap();
            assert_eq!(v.1, out.inner)
        }
    }

    #[test]
    fn test_base2_expr() {
        use crate::lib::tokenize::Token::*;
        let v = vec![
            H, Eq, M, Arrow, K, Eq,
            D, Plus, Open, D, Multiple, Open,  E, Minus,
            F, Close,  Divide, Const(25.5), Close];
        let rs1 = Base2Expr::build(&v).unwrap();

        let v = vec![H, Eq, P, Arrow, K, Eq, D];
        let rs2 = Base2Expr::build(&v).unwrap();

        assert_ne!(rs1, rs2);
        assert_eq!(rs1, rs1);

        assert_eq!(3.0, rs1.run(Scope::def(1.0, 80, 29)).unwrap());
        assert_eq!(5.0, rs2.run(Scope::def(5.0, 80, 29)).unwrap());
        assert_eq!(rs2.output, output::H::P);
    }
}