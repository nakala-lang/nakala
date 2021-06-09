#[derive(Debug, PartialEq)]
pub enum Val {
    Number(i128),
    Missing, // filler name
}

impl Val {
    pub(crate) fn add(&self, other: Val) -> Self {
        match self {
            Val::Number(n) => Val::Number(n + &other.into()),
            _ => todo!(),
        }
    }

    pub(crate) fn sub(&self, other: Val) -> Self {
        match self {
            Val::Number(n) => Val::Number(n + &other.into()),
            _ => todo!(),
        }
    }

    pub(crate) fn mul(&self, other: Val) -> Self {
        match self {
            Val::Number(n) => Val::Number(n + &other.into()),
            _ => todo!(),
        }
    }

    pub(crate) fn div(&self, other: Val) -> Self {
        match self {
            Val::Number(n) => Val::Number(n + &other.into()),
            _ => todo!(),
        }
    }
}

impl From<Val> for i128 {
    fn from(v: Val) -> i128 {
        match v {
            Val::Number(n) => n,
            _ => panic!("Cannot convert {:?} to i128", v),
        }
    }
}
