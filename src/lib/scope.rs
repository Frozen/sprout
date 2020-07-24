#[derive(Debug, Copy, Clone)]
pub struct Scope {
    pub a: bool,
    pub b: bool,
    pub c: bool,
}

impl Scope {
    pub fn abc(a: bool, b: bool, c: bool) -> Scope {
        return Scope {
            a, b, c
        }
    }
}

