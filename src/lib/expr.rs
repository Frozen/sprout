use anyhow::Result;
use crate::lib::tokenize::{tokenize, Token};
use crate::lib::expr::Expr::{Base2, Base1};
use crate::lib::base1::Base1Expr;
use crate::lib::base2::Base2Expr;
use crate::lib::output;
use crate::lib::scope::Scope;

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum BaseOutput {
    F64(f64),
    Output(Option<output::H>)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Base1(Base1Expr),
    Base2(Base2Expr)
}

impl Expr {
    pub fn from_str(s: &str) -> Result<Expr> {
        let tokens = tokenize(s)?;
        match tokens.get(0) {
            Some(Token::H) => Ok(Base2(Base2Expr::build(&tokens)?)),
            Some(_) => Ok(Base1(Base1Expr::build(&tokens)?)),
            _ => return Err(anyhow!("invalid token string"))
        }
    }

    fn run(&self, s: Scope) -> Result<BaseOutput> {
        match self{
            Base1(exp) => {
                return Ok(BaseOutput::Output(exp.run(s)));
            },
            Base2(exp) => {
                Ok(BaseOutput::F64(exp.run(s)?))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lib::expr::Expr;
    use crate::lib::scope::Scope;
    use crate::lib::output;
    use super::BaseOutput;

    #[test]
    fn test_expr1() {
        let rs = Expr::from_str("A && B && !C => H = M").unwrap();
        assert_eq!(
            rs.run(Scope::abc(true, true, false)).unwrap(),
            BaseOutput::Output(Some(output::H::M))
        );
    }

    #[test]
    fn test_expr2() {
        let rs = Expr::from_str("H = P => K = D + (D * (E - F) / 25.5)").unwrap();
        assert_eq!(
            rs.run(Scope::def(1.0, 1, 1)).unwrap(),
            BaseOutput::F64(1.0)
        )
    }
}