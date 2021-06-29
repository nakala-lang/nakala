use hir::Expr;

#[derive(Debug)]
pub enum EngineError {
    InvalidExpression(Expr),
    InvalidAddOperation,
    InvalidSubOperation,
    InvalidMulOperation,
    InvalidDivOperation,
    InvalidNegOperation,
    BindingAlreadyExists { binding_name: String },
    BindingUndefined { binding_name: String },
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
            EngineError::BindingAlreadyExists { binding_name } => f.write_str(
                format!("The binding `{}` already exists in the scope", binding_name).as_str(),
            ),
            EngineError::BindingUndefined { binding_name } => f.write_str(
                format!("The binding `{}` is undefined in the scope", binding_name).as_str(),
            ),
            EngineError::Unknown => f.write_str("An unknown error occurred"),
        }
    }
}

impl std::error::Error for EngineError {}
