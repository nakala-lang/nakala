use super::EngineError;

#[derive(Debug, PartialEq, Clone)]
pub enum Val {
    Missing, // filler name
    Unit,    // Unit tuple
    Number(i128),
    String(String),
    Boolean(bool),
}

impl Val {
    pub(crate) fn add(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(x) => match other {
                Val::Number(y) => Ok(Val::Number(*x + y)),
                _ => Err(EngineError::InvalidAddOperation),
            },
            _ => Err(EngineError::InvalidAddOperation),
        }
    }

    pub(crate) fn sub(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(x) => match other {
                Val::Number(y) => Ok(Val::Number(*x - y)),
                _ => Err(EngineError::InvalidSubOperation),
            },
            _ => Err(EngineError::InvalidSubOperation),
        }
    }

    pub(crate) fn mul(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(x) => match other {
                Val::Number(y) => Ok(Val::Number(*x * y)),
                _ => Err(EngineError::InvalidMulOperation),
            },
            _ => Err(EngineError::InvalidMulOperation),
        }
    }

    pub(crate) fn div(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(x) => match other {
                Val::Number(y) => Ok(Val::Number(*x / y)),
                _ => Err(EngineError::InvalidDivOperation),
            },
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
            Val::Missing | Val::Unit => Ok(()),
            Val::Number(n) => f.write_str(format!("{}", n).as_str()),
            Val::String(s) => f.write_str(s),
            Val::Boolean(b) => f.write_str(format!("{}", b).as_str()),
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
