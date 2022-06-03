use ast::ty::Type;
use meta::SourceId;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::val::Value;

#[derive(Error, Diagnostic, Debug)]
pub enum RuntimeError {
    // This should never bubble up all the way to the top, and the parser should prevent using
    // return in non function contexts
    #[error("Early Return")]
    EarlyReturn(Value),

    #[error("Undefined variable")]
    #[diagnostic(code(nak_runtime::unknown_variable))]
    UndefinedVariable(SourceId, #[label("Undefined variable")] SourceSpan),

    #[error("Expected {1}, got {2} value instead")]
    #[diagnostic(code(nak_runtime::unexpected_value))]
    UnexpectedValueType(
        SourceId,
        Type,
        String,
        #[label("This value is not of type {1}")] SourceSpan,
    ),

    #[error("Unsupported operation")]
    #[diagnostic(code(nak_runtime::unexpected_operation))]
    UnsupportedOperation(
        SourceId,
        #[label("This operation doesn't support these types")] SourceSpan,
        #[label("{3}")] SourceSpan,
        Type,
        #[label("{5}")] SourceSpan,
        Type,
    ),
}
