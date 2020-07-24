use anyhow::Result;
use crate::lib::tokenize::Token;
use std::fmt::Debug;
use crate::lib::output;
use crate::lib::scope::Scope;


#[derive(Debug, Clone)]
pub struct Base1Expr {
    output: output::H,
    original: Vec<Token>,
    executable: Executable,
}

impl PartialEq for Base1Expr {
    fn eq(&self, other: &Self) -> bool {
        self.executable == other.executable
    }
}

impl Base1Expr {
    pub fn build(tokens: &[Token]) -> Result<Base1Expr> {
        let pos = tokens.iter().position(|&x| x == Token::Arrow);
        return match pos {
            Some(p) => {
                let (left, _right) = tokens.split_at(p);
                let ex = Executable::build(left)?;
                let out = match tokens.get(p + 3) {
                    Some(Token::M) => output::H::M,
                    Some(Token::P) => output::H::P,
                    Some(Token::T) => output::H::T,
                    _ => return Err(anyhow!("no output found")),
                };
                Ok(Base1Expr {
                    output: out,
                    original: tokens.to_vec(),
                    executable: ex,
                })
            },
            None => Err(anyhow!("invalid expression"))
        }
    }

    pub fn run(&self, s: Scope) -> Option<output::H> {
        if self.executable.run(s) {
            return Some(self.output);
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Executable {
    inner: Vec<Token>,
}

impl Executable {
    fn run(&self, s: Scope) -> bool {
        let mut stack = vec![];
        for i in &self.inner {
            match i {
                Token::A => stack.push(s.a),
                Token::B => stack.push(s.b),
                Token::C => stack.push(s.c),
                Token::Not => {
                    let b = !stack.pop().unwrap();
                    stack.push(b);
                },
                Token::And => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a && b);
                },
                _ => unreachable!()
            }
        }
        return stack.pop().unwrap()
    }

    fn build(input: &[Token]) -> Result<Executable> {
        let mut stack = vec![];
        let mut out = vec![];

        for i in input {
            match i {
                Token::Not => stack.push(Token::Not),
                Token::A => {
                    out.push(Token::A);
                    match stack.pop() {
                        Some(v) => out.push(v),
                        None => continue
                    }
                },
                Token::B => {
                    out.push(Token::B);
                    match stack.pop() {
                        Some(v) => out.push(v),
                        None => continue
                    }
                },
                Token::C => {
                    out.push(Token::C);
                    match stack.pop() {
                        Some(v) => out.push(v),
                        None => continue
                    }
                },
                Token::And => {
                    stack.push(Token::And);
                },
                t => return Err(anyhow!("unexpected token {:?}", t))
            }
        }
        for i in stack {
            out.push(i)
        }
        return Ok(Executable {
            inner: out
        });
    }
}


#[cfg(test)]
mod test {
    use crate::lib::base1::{Base1Expr, Executable};
    use crate::lib::output;
    use crate::lib::scope::Scope;
    use crate::lib::tokenize::Token::Const;


    #[test]
    fn test_build_executable() {
        use super::super::tokenize::Token::*;
        let v = vec![A, And, B, And, Not, C];
        let out = Executable::build(&v).unwrap();

        assert_eq!(vec![A, B, And, C, Not, And], out.inner)
    }

    #[test]
    fn test_base1_expr() {
        use crate::lib::tokenize::Token::*;
        let v = vec![A, And, B, And, Not, C, Arrow, H, Eq, M];
        let rs1 = Base1Expr::build(&v).unwrap();

        let v = vec![Not, A, And, B, And, C, Arrow, H, Eq, T];
        let rs2 = Base1Expr::build(&v).unwrap();

        assert_ne!(rs1, rs2);
        assert_eq!(rs1, rs1);

        assert_eq!(None, rs1.run(Scope::abc(true, true, true)));
        assert_eq!(Some(output::H::M), rs1.run(Scope::abc(true, true, false)));

        assert_eq!(Some(output::H::T), rs2.run(Scope::abc(false, true, true)));
        assert_eq!(None, rs2.run(Scope::abc(true, true, true)));
    }

    #[test]
    fn test_invalid() {
        assert!(Base1Expr::build(&vec![Const(1.0)]).is_err());
    }
}
