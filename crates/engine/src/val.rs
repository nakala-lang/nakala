use super::EngineError;
use crate::class::Class;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Val {
    Missing, // filler name
    Unit,    // Unit tuple
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Self>),
    Class(Class),
}

impl Val {
    pub(crate) fn get_type(&self) -> &str {
        match self {
            Val::Missing => "missing",
            Val::Unit => "unit",
            Val::Number(_) => "number",
            Val::String(_) => "string",
            Val::Boolean(_) => "boolean",
            Val::List(_) => "list",
            Val::Class(_) => "class",
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

    pub(crate) fn index(&self, index: Self) -> Result<Self, EngineError> {
        if let Self::List(l) = self {
            if let Self::Number(n) = index {
                let index_as_int = n as usize;
                // We only support int indices for now. This checks if the
                // cast to usize was lossless
                #[allow(clippy::float_cmp)]
                if n == (n as usize) as f64 {
                    l.get(index_as_int)
                        .cloned()
                        .ok_or(EngineError::IndexOutOfBounds {
                            index: index_as_int,
                            len: l.len(),
                        })
                } else {
                    Err(EngineError::ListIndicesMustBeIntegers)
                }
            } else {
                Err(EngineError::MismatchedTypes {
                    expected: Val::Number(0.0),
                    actual: index,
                })
            }
        } else {
            Err(EngineError::InvalidIndexOperation { x: self.clone() })
        }
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Missing | Val::Unit => Ok(()),
            Val::Number(n) => write!(f, "{}", n),
            Val::String(s) => write!(f, "{}", s),
            Val::Boolean(b) => write!(f, "{}", b),
            Val::List(l) => {
                write!(f, "[")?;
                for (index, val) in l.iter().enumerate() {
                    write!(f, "{}", val)?;
                    if index != l.len() - 1 {
                        write!(f, ",")?;
                    } else {
                        write!(f, "]")?;
                    }
                }
                Ok(())
            }
            Val::Class(_) => write!(f, "Class Object"),
        }
    }
}
