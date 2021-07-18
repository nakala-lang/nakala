use hir::Expr;

#[derive(Debug)]
pub enum EngineError {
    InvalidExpression(Expr),
    InvalidAddOperation,
    InvalidSubOperation,
    InvalidMulOperation,
    InvalidDivOperation,
    InvalidNegOperation,
    VariableAlreadyExists { variable_name: String },
    VariableUndefined { variable_name: String },
    FunctionAlreadyExists { function_name: String },
    FunctionUndefined { function_name: String },
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
                f.write_str("Could not find add handler for the provided types")
            }
            EngineError::InvalidSubOperation => {
                f.write_str("Could not find sub handler for the provided types")
            }
            EngineError::InvalidMulOperation => {
                f.write_str("Could not find mul handler for the provided types")
            }
            EngineError::InvalidDivOperation => {
                f.write_str("Could not find div handler for the provided types")
            }
            EngineError::InvalidNegOperation => {
                f.write_str("Could not find neg handler for the provided type")
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
            EngineError::NotYetImplemented => f.write_str("This feature is not yet implemented"),
            EngineError::Unknown => f.write_str("An unknown error occurred"),
        }
    }
}

impl std::error::Error for EngineError {}
