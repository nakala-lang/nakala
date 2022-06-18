use ast::ty::Type;
use meta::SourceId;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::value::Value;

#[derive(Error, Diagnostic, Debug)]
pub enum RuntimeError {
    // This should never bubble up all the way to the top, and the parser should prevent using
    // return in non function contexts
    #[error("Early Return")]
    EarlyReturn(Value),

    #[error("Arity mismatch")]
    #[diagnostic(code(nak_runtime::arity_mismatch))]
    ArityMismatch(
        SourceId,
        #[label("This function expects {2} arguments, but got {3}")] SourceSpan,
        usize,
        usize,
    ),

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

    #[error("Undefined class property")]
    #[diagnostic(code(nak_runtime::undefined_class_property))]
    UndefinedClassProperty(
        SourceId,
        #[label("This class doesn't have any property named {2}")] SourceSpan,
        String,
    ),

    #[error("Undefined static class property")]
    #[diagnostic(code(nak_runtime::undefined_static_property))]
    UndefinedStaticClassProperty(
        SourceId,
        #[label("This class doesn't have any static properties named {2}")] SourceSpan,
        String,
    ),

    #[error("Incompatible types")]
    #[diagnostic(
        code(nak::incompatible_types),
        help("This should have been caught by the parser")
    )]
    IncompatibleTypes(
        SourceId,
        #[label("Expects types compatible with {2}")] SourceSpan,
        Type,
        #[label("{4}")] SourceSpan,
        Type,
    ),
}
