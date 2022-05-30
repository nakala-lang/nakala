use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum RuntimeError {
    #[error("Undefined variable")]
    UndefinedVariable
}
