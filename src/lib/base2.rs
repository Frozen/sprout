use crate::lib::tokenize::Token;

struct Base2Expr {

}

struct Executable {
    inner: Vec<Token>,
}

impl Executable {
    pub fn build(tokens: &[Token]) -> Result<Executable, String> {
        let mut stack = vec![];
        let mut out = vec![];

        for t in tokens {
            match t {
                Token::D => {
                    out.push(Token::D)
                },
                Token::E => out.push(Token::E),
                Token::F => out.push(Token::F),
                Token::Plus => {
                    stack.push(Token::Plus)
                },
                Token::BracketOpen => {
                    stack.push(Token::BracketOpen)
                },
                Token::BracketClose => {
                    out.push(stack.pop().unwrap());
                    stack.pop();
                }
                t => return Err(format!("unexpected token {:?}", t))
            }
        }

        while stack.len() > 0 {
            out.push(stack.pop().unwrap());
        }


        return Ok(Executable {
            inner: out,
        })
    }
}


#[cfg(test)]
mod test {
    use super::{Base2Expr, Executable};
    use crate::lib::output;


    #[test]
    fn test_build_executable() {
        use super::super::tokenize::Token as T;
        let v = vec![T::Not, T::A, T::And, T::B, T::And, T::C];
        let out = Executable::build(&v).unwrap();

        assert_eq!(vec![T::A, T::Not, T::B, T::And, T::C, T::And], out.inner)
    }

    // #[test]
    // fn test_base2_expr() {
    //     use crate::lib::tokenize::Token::*;
    //     let v = vec![A, And, B, And, Not, C, Arrow, H, Eq, M];
    //     let rs1 = Base2Expr::build(v).unwrap();
    //
    //     let v = vec![Not, A, And, B, And, C, Arrow, H, Eq, T];
    //     let rs2 = Base2Expr::build(v).unwrap();
    //
    //     assert_ne!(rs1, rs2);
    //     assert_eq!(rs1, rs1);
    //
    //     assert_eq!(None, rs1.run(Scope::abc(true, true, true)));
    //     assert_eq!(Some(output::H::M), rs1.run(Scope::abc(true, true, false)));
    //
    //     assert_eq!(Some(output::H::T), rs2.run(Scope::abc(false, true, true)));
    //     assert_eq!(None, rs2.run(Scope::abc(true, true, true)));
    // }
}