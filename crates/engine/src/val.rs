use super::EngineError;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Val {
    Missing, // filler name
    Unit,    // Unit tuple
    Number(i128),
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

    pub(crate) fn neg(&self) -> Result<Self, EngineError> {
        match self {
            Val::Number(n) => Ok(Val::Number(-n)),
            _ => Err(EngineError::InvalidNegOperation),
        }
    }
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Number(n) => f.write_str(format!("{}", n).as_str()),
            _ => Ok(()),
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
