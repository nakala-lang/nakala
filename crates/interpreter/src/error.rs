use ast::ty::Type;
use meta::SourceId;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum RuntimeError {
    #[error("Undefined variable")]
    UndefinedVariable,

    #[error("Expected {1}, got non {1} value instead")]
    #[diagnostic(code(nak::unexpected_value))]
    UnexpectedValueType(
        SourceId,
        Type,
        #[label("This value is not of type {1}")] SourceSpan,
    ),
}
