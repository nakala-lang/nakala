use hir::Expr;

#[derive(Debug)]
pub enum EngineError {
    InvalidExpression(Expr),
    InvalidAddOperation,
    InvalidSubOperation,
    InvalidMulOperation,
    InvalidDivOperation,
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
            EngineError::Unknown => f.write_str("An unknown error occurred"),
        }
    }
}

impl std::error::Error for EngineError {}
