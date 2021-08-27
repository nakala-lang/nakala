use crate::val::Val;
use nu_ansi_term::Color::{Green, Red, Yellow};

#[derive(Debug, Clone)]
pub enum EngineError {
    InvalidAddOperation {
        x: Val,
        y: Val,
    },
    InvalidSubOperation {
        x: Val,
        y: Val,
    },
    InvalidMulOperation {
        x: Val,
        y: Val,
    },
    InvalidDivOperation {
        x: Val,
        y: Val,
    },
    InvalidNegOperation {
        x: Val,
    },
    InvalidGreaterThanOperation {
        x: Val,
        y: Val,
    },
    InvalidGreaterThanOrEqOperation {
        x: Val,
        y: Val,
    },
    InvalidLessThanOperation {
        x: Val,
        y: Val,
    },
    InvalidLessThanOrEqOperation {
        x: Val,
        y: Val,
    },
    InvalidNotOperation {
        x: Val,
    },
    InvalidAndOperation {
        x: Val,
        y: Val,
    },
    InvalidOrOperation {
        x: Val,
        y: Val,
    },
    InvalidIndexOperation {
        x: Val,
    },
    VariableAlreadyExists {
        variable_name: String,
    },
    VariableUndefined {
        variable_name: String,
    },
    FunctionAlreadyExists {
        function_name: String,
    },
    FunctionUndefined {
        function_name: String,
    },
    MismatchedParameterCount {
        actual: usize,
        expected: usize,
    },
    ClassCreateMismatchedParameterCount {
        name: String,
        actual: usize,
        expected: usize,
    },
    MismatchedTypes {
        actual: Val,
        expected: Val,
    },
    EarlyReturn {
        value: Val,
    },
    ListIndicesMustBeIntegers,
    IndexOutOfBounds {
        index: usize,
        len: usize,
    },
    ClassAlreadyDefined {
        name: String,
    },
    ClassUndefined {
        name: String,
    },
    NotYetImplemented,
    Unknown,
}

fn missing_handler_msg(handler: &str, x: &Val, y: &Val) -> String {
    format!(
        "Could not find {} handler for the provided types {} and {}",
        handler,
        Yellow.paint(x.get_type()),
        Yellow.paint(y.get_type())
    )
}

fn missing_handler_msg_single(handler: &str, x: &Val) -> String {
    format!(
        "Could not find {} handler for the provided type {}",
        handler,
        Yellow.paint(x.get_type())
    )
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: String = match self {
            EngineError::InvalidAddOperation { x, y } => missing_handler_msg("ADD", x, y),
            EngineError::InvalidSubOperation { x, y } => missing_handler_msg("SUB", x, y),
            EngineError::InvalidMulOperation { x, y } => missing_handler_msg("MUL", x, y),
            EngineError::InvalidDivOperation { x, y } => missing_handler_msg("DIV", x, y),
            EngineError::InvalidNegOperation { x } => missing_handler_msg_single("NEG", x),
            EngineError::InvalidGreaterThanOperation { x, y } => {
                missing_handler_msg("GREATER_THAN", x, y)
            }
            EngineError::InvalidGreaterThanOrEqOperation { x, y } => {
                missing_handler_msg("GREATER_THAN_OR_EQ", x, y)
            }
            EngineError::InvalidLessThanOperation { x, y } => {
                missing_handler_msg("LESS_THAN", x, y)
            }
            EngineError::InvalidLessThanOrEqOperation { x, y } => {
                missing_handler_msg("LESS_THAN_OR_EQ", x, y)
            }
            EngineError::InvalidNotOperation { x } => missing_handler_msg_single("NOT", x),
            EngineError::InvalidAndOperation { x, y } => missing_handler_msg("AND", x, y),
            EngineError::InvalidOrOperation { x, y } => missing_handler_msg("OR", x, y),
            EngineError::InvalidIndexOperation { x } => missing_handler_msg_single("INDEX", x),
            EngineError::VariableAlreadyExists { variable_name } => format!(
                "The variable {} already exists in the scope",
                Yellow.paint(variable_name)
            ),
            EngineError::VariableUndefined { variable_name } => {
                format!(
                    "The variable {} is undefined in the scope",
                    Yellow.paint(variable_name)
                )
            }
            EngineError::FunctionAlreadyExists { function_name } => format!(
                "The function {} already exists in the scope",
                Yellow.paint(function_name)
            ),

            EngineError::FunctionUndefined { function_name } => {
                format!(
                    "The function {} is undefined in the scope",
                    Yellow.paint(function_name)
                )
            }
            EngineError::MismatchedParameterCount { actual, expected } => format!(
                "The function expected {} parameters, but received {}",
                Green.paint(format!("{}", expected)),
                Red.paint(format!("{}", actual))
            ),
            EngineError::ClassCreateMismatchedParameterCount {
                name,
                actual,
                expected,
            } => format!(
                "The class {} expected {} parameters, but received {}",
                Yellow.paint(format!("{}", name)),
                Green.paint(format!("{}", expected)),
                Red.paint(format!("{}", actual))
            ),
            EngineError::MismatchedTypes { actual, expected } => format!(
                "Expected type {}, but got {} instead",
                Green.paint(expected.get_type().to_string()),
                Yellow.paint(actual.get_type().to_string()),
            ),
            // Early returns are uncaught when return is not called from within a function
            EngineError::EarlyReturn { value: _ } => {
                "Can only return when inside the context of a function".into()
            }
            EngineError::ListIndicesMustBeIntegers => "List indices must be integers".into(),
            EngineError::IndexOutOfBounds { index, len } => format!(
                "The index {} is out of bounds for list of length {}",
                Yellow.paint(format!("{}", index)),
                Green.paint(format!("{}", len)),
            ),
            EngineError::ClassAlreadyDefined { name } => format!(
                "The class {} is already defined",
                Yellow.paint(format!("{}", name))
            ),
            EngineError::ClassUndefined { name } => format!(
                "The class {} is undefined in the scope",
                Yellow.paint(format!("{}", name))
            ),
            EngineError::NotYetImplemented => "This feature is not yet implemented".into(),
            EngineError::Unknown => "An unknown error occurred".into(),
        };

        f.write_str(format!("{}: {}", Red.paint("Engine Error"), msg).as_str())
    }
}

impl std::error::Error for EngineError {}
