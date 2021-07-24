use hir::Expr;

#[derive(Debug)]
pub enum EngineError {
    InvalidExpression(Expr),
    InvalidAddOperation,
    InvalidSubOperation,
    InvalidMulOperation,
    InvalidDivOperation,
    InvalidNegOperation,
    InvalidGreaterThanOperation,
    InvalidGreaterThanOrEqOperation,
    InvalidLessThanOperation,
    InvalidLessThanOrEqOperation,
    InvalidNotOperation,
    InvalidAndOperation,
    InvalidOrOperation,
    VariableAlreadyExists { variable_name: String },
    VariableUndefined { variable_name: String },
    FunctionAlreadyExists { function_name: String },
    FunctionUndefined { function_name: String },
    MismatchedParameterCount { actual: usize, expected: usize },
    NotYetImplemented,
    Unknown,
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::InvalidExpression(_) => {
                f.write_str("Unable to parse expression because it's invalid")
            }
            EngineError::InvalidAddOperation => {
                f.write_str("Could not find ADD handler for the provided types")
            }
            EngineError::InvalidSubOperation => {
                f.write_str("Could not find SUB handler for the provided types")
            }
            EngineError::InvalidMulOperation => {
                f.write_str("Could not find MUL handler for the provided types")
            }
            EngineError::InvalidDivOperation => {
                f.write_str("Could not find DIV handler for the provided types")
            }
            EngineError::InvalidNegOperation => {
                f.write_str("Could not find NEG handler for the provided type")
            }
            EngineError::InvalidGreaterThanOperation => {
                f.write_str("Could not find GREATER_THAN handler for the provided type")
            }
            EngineError::InvalidGreaterThanOrEqOperation => {
                f.write_str("Could not find GREATER_THAN_OR_EQ handler for the provided type")
            }
            EngineError::InvalidLessThanOperation => {
                f.write_str("Could not find LESS_THAN handler for the provided type")
            }
            EngineError::InvalidLessThanOrEqOperation => {
                f.write_str("Could not find LESS_THAN_OR_EQ handler for the provided type")
            }
            EngineError::InvalidNotOperation => {
                f.write_str("Could not find NOT handler for the provided type")
            }
            EngineError::InvalidAndOperation => {
                f.write_str("Could not find AND handler for the provided type")
            }
            EngineError::InvalidOrOperation => {
                f.write_str("Could not find OR handler for the provided type")
            }
            EngineError::VariableAlreadyExists { variable_name } => f.write_str(
                format!(
                    "The variable `{}` already exists in the scope",
                    variable_name
                )
                .as_str(),
            ),
            EngineError::VariableUndefined { variable_name } => f.write_str(
                format!("The variable `{}` is undefined in the scope", variable_name).as_str(),
            ),
            EngineError::FunctionAlreadyExists { function_name } => f.write_str(
                format!(
                    "The function `{}` already exists in the scope",
                    function_name
                )
                .as_str(),
            ),
            EngineError::FunctionUndefined { function_name } => f.write_str(
                format!("The function `{}` is undefined in the scope", function_name).as_str(),
            ),
            EngineError::MismatchedParameterCount { actual, expected } => f.write_str(
                format!(
                    "The function expected {} parameters, but received {}",
                    expected, actual
                )
                .as_str(),
            ),
            EngineError::NotYetImplemented => f.write_str("This feature is not yet implemented"),
            EngineError::Unknown => f.write_str("An unknown error occurred"),
        }
    }
}

impl std::error::Error for EngineError {}
