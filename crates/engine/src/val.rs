use super::EngineError;

#[derive(Debug, PartialEq)]
pub enum Val {
    Number(i128),
    Missing, // filler name
}

impl Val {
    pub(crate) fn add(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(n) => Ok(Val::Number(n + &other.into())),
            _ => Err(EngineError::InvalidAddOperation),
        }
    }

    pub(crate) fn sub(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(n) => Ok(Val::Number(n - &other.into())),
            _ => Err(EngineError::InvalidSubOperation),
        }
    }

    pub(crate) fn mul(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(n) => Ok(Val::Number(n * &other.into())),
            _ => Err(EngineError::InvalidMulOperation),
        }
    }

    pub(crate) fn div(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(n) => Ok(Val::Number(n / &other.into())),
            _ => Err(EngineError::InvalidDivOperation),
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
