#[derive(Debug, Copy, Clone, Default)]
pub struct Scope {
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: f64,
    pub e: i64,
    pub f: i64,
}

impl Scope {
    pub fn new(a: bool, b: bool, c: bool, d: f64, e: i64, f: i64) -> Scope {
        Scope {
            a, b, c, d, e, f
        }
    }

    pub fn abc(a: bool, b: bool, c: bool) -> Scope {
        return Scope {
            a,
            b,
            c,
            ..Default::default()
        }
    }

    pub fn def(d: f64, e: i64, f: i64) -> Scope {
        return Scope {
            d,
            e,
            f,
            ..Default::default()
        }
    }
}

