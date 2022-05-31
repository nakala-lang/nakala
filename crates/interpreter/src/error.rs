use meta::SourceId;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum RuntimeError {
    #[error("Undefined variable")]
    UndefinedVariable,

    #[error("Expected instance, got non instance value instead")]
    #[diagnostic(code(nak::unexpected_value))]
    ExpectedInstance(
        SourceId,
        #[label("This value is not an instance")] SourceSpan,
    ),
}
