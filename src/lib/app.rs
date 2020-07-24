use anyhow::Result;
use crate::lib::expr::Expr;
use crate::lib::scope::Scope;
use crate::lib::output;

#[derive(Debug, Clone, PartialEq)]
pub struct App {
    exprs: Vec<Expr>,
}

impl App {
    pub fn new() -> App {
        App {
            exprs: vec![],
        }
    }

    pub fn add(&self, expr: &str) -> Result<App> {
        let ok = Expr::from_str(expr)?;
        Ok(self.add_expr(ok))
    }

    pub fn add_expr(&self, e: Expr) -> App {
        let exprs = self.exprs.clone();
        let mut exprs: Vec<Expr> = exprs.into_iter().filter_map(|x| {
            if x == e {
                None
            } else {
                Some(x)
            }
        }).collect();
        exprs.push(e);
        App {
            exprs
        }
    }

    pub fn run(&self, s: Scope) -> Result<f64> {
        let rs: Vec<output::H> = self.exprs.iter().filter_map(
            |x| match x {
                Expr::Base1(e) => e.run(s),
                _ => None
            }
        ).collect();

        if rs.is_empty() {
            return Err(anyhow!("expr not found"))
        }
        let fst = rs[0];
        for i in &self.exprs {
            match i {
                Expr::Base2(e) => {
                    if e.output() == fst {
                        return e.run(s)
                    }
                },
                _ => continue,
            }
        }
        return Err(anyhow!("expr for {:?} not found", fst));
    }
}


#[cfg(test)]
mod test {
    use crate::lib::expr::Expr;
    use crate::lib::scope::Scope;
    use crate::lib::output;
    use super::App;

    #[test]
    fn test_app() {
        let app = App::new();
        let app = app.add("A && B && !C => H = M").unwrap();
        let app = app.add("A && B && C => H = P").unwrap();
        let app = app.add("!A && B && C => H = T").unwrap();
        let app = app.add("H = M => K = D + (D * E / 10)").unwrap();
        let app = app.add("H = P => K = D + (D * (E - F) / 25.5)").unwrap();
        let app = app.add("H = T => K = D - (D * F / 30)").unwrap();

        let rs = app.run(Scope::new(false, true, true, 5.0, 0, 30)).unwrap();
        assert_eq!(0.0, rs);


        let rs = app.run(Scope::new(true, true, true, 1.0, 52, 1)).unwrap();
        assert_eq!(3.0, rs);
    }
}