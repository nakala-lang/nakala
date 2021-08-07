use super::EngineError;

#[derive(Debug, PartialEq, Clone)]
pub enum Val {
    Missing, // filler name
    Unit,    // Unit tuple
    Number(f64),
    String(String),
    Boolean(bool),
}

impl Val {
    pub(crate) fn get_type(&self) -> &str {
        match self {
            Val::Missing => "missing",
            Val::Unit => "unit",
            Val::Number(_) => "number",
            Val::String(_) => "string",
            Val::Boolean(_) => "boolean",
        }
    }

    pub(crate) fn add(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(x) => match other {
                Val::Number(y) => Ok(Val::Number(*x + y)),
                _ => Err(EngineError::InvalidAddOperation {
                    x: self.clone(),
                    y: other,
                }),
            },
            _ => Err(EngineError::InvalidAddOperation {
                x: self.clone(),
                y: other,
            }),
        }
    }

    pub(crate) fn sub(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(x) => match other {
                Val::Number(y) => Ok(Val::Number(*x - y)),
                _ => Err(EngineError::InvalidSubOperation {
                    x: self.clone(),
                    y: other,
                }),
            },
            _ => Err(EngineError::InvalidSubOperation {
                x: self.clone(),
                y: other,
            }),
        }
    }

    pub(crate) fn mul(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(x) => match other {
                Val::Number(y) => Ok(Val::Number(*x * y)),
                _ => Err(EngineError::InvalidMulOperation {
                    x: self.clone(),
                    y: other,
                }),
            },
            _ => Err(EngineError::InvalidMulOperation {
                x: self.clone(),
                y: other,
            }),
        }
    }

    pub(crate) fn div(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(x) => match other {
                Val::Number(y) => Ok(Val::Number(*x / y)),
                _ => Err(EngineError::InvalidDivOperation {
                    x: self.clone(),
                    y: other,
                }),
            },
            _ => Err(EngineError::InvalidDivOperation {
                x: self.clone(),
                y: other,
            }),
        }
    }

    pub(crate) fn neg(&self) -> Result<Self, EngineError> {
        match self {
            Val::Number(n) => Ok(Val::Number(-n)),
            _ => Err(EngineError::InvalidNegOperation { x: self.clone() }),
        }
    }

    pub(crate) fn equals(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(x) => match other {
                Val::Number(y) => Ok(Val::Boolean(x.eq(&y))),
                _ => Ok(Val::Boolean(false)),
            },
            Val::Boolean(x) => match other {
                Val::Boolean(y) => Ok(Val::Boolean(x.eq(&y))),
                _ => Ok(Val::Boolean(false)),
            },
            Val::String(x) => match other {
                Val::String(y) => Ok(Val::Boolean(x.eq(&y))),
                _ => Ok(Val::Boolean(false)),
            },
            Val::Unit => match other {
                Val::Unit => Ok(Val::Boolean(true)),
                _ => Ok(Val::Boolean(false)),
            },
            _ => unreachable!("Cannot equals Missing type"),
        }
    }

    pub(crate) fn greater_than(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(x) => match other {
                Val::Number(y) => Ok(Val::Boolean(*x > y)),
                _ => Err(EngineError::InvalidGreaterThanOperation {
                    x: self.clone(),
                    y: other,
                }),
            },
            _ => Err(EngineError::InvalidGreaterThanOperation {
                x: self.clone(),
                y: other,
            }),
        }
    }

    pub(crate) fn greater_than_or_eq(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(x) => match other {
                Val::Number(y) => Ok(Val::Boolean(*x >= y)),
                _ => Err(EngineError::InvalidGreaterThanOrEqOperation {
                    x: self.clone(),
                    y: other,
                }),
            },
            _ => Err(EngineError::InvalidGreaterThanOrEqOperation {
                x: self.clone(),
                y: other,
            }),
        }
    }

    pub(crate) fn less_than(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(x) => match other {
                Val::Number(y) => Ok(Val::Boolean(*x < y)),
                _ => Err(EngineError::InvalidLessThanOperation {
                    x: self.clone(),
                    y: other,
                }),
            },
            _ => Err(EngineError::InvalidLessThanOperation {
                x: self.clone(),
                y: other,
            }),
        }
    }

    pub(crate) fn less_than_or_eq(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Number(x) => match other {
                Val::Number(y) => Ok(Val::Boolean(*x <= y)),
                _ => Err(EngineError::InvalidLessThanOrEqOperation {
                    x: self.clone(),
                    y: other,
                }),
            },
            _ => Err(EngineError::InvalidLessThanOrEqOperation {
                x: self.clone(),
                y: other,
            }),
        }
    }

    pub(crate) fn not(&self) -> Result<Self, EngineError> {
        match self {
            Val::Boolean(b) => Ok(Val::Boolean(!b)),
            _ => Err(EngineError::InvalidNotOperation { x: self.clone() }),
        }
    }

    pub(crate) fn or(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Boolean(x) => match other {
                Val::Boolean(y) => Ok(Val::Boolean(*x || y)),
                _ => Err(EngineError::InvalidOrOperation {
                    x: self.clone(),
                    y: other,
                }),
            },
            _ => Err(EngineError::InvalidOrOperation {
                x: self.clone(),
                y: other,
            }),
        }
    }

    pub(crate) fn and(&self, other: Val) -> Result<Self, EngineError> {
        match self {
            Val::Boolean(x) => match other {
                Val::Boolean(y) => Ok(Val::Boolean(*x && y)),
                _ => Err(EngineError::InvalidAndOperation {
                    x: self.clone(),
                    y: other,
                }),
            },
            _ => Err(EngineError::InvalidAndOperation {
                x: self.clone(),
                y: other,
            }),
        }
    }

    pub(crate) fn is_true(&self) -> Result<bool, EngineError> {
        match self {
            Val::Boolean(x) => Ok(*x),
            _ => Err(EngineError::MismatchedTypes {
                actual: self.clone(),
                expected: Val::Boolean(true),
            }),
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

//impl From<Val> for i128 {
//    fn from(v: Val) -> i128 {
//        match v {
//            Val::Number(n) => n,
//            _ => panic!("Cannot convert {:?} to i128", v),
//        }
//    }
//}
