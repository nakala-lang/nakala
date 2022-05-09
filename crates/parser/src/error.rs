use ast::ty::Type;
use lexer::TokenKind;
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    #[error("Expected an expression")]
    #[diagnostic(
        code(nak::cant_parse_primary_expr),
        help("Change this into an expression")
    )]
    ExpectedExpression(#[source_code] NamedSource, #[label] SourceSpan),

    #[error("Expected token {1}")]
    #[diagnostic(code(nak::expected_token))]
    ExpectedToken(
        #[source_code] NamedSource,
        String,
        TokenKind,
        #[label("Consider adding '{2}' here")] SourceSpan,
    ),

    #[error("Unexpected EOF")]
    #[diagnostic(
        code(nak::unexpected_eof),
        help("Expected more tokens, but none were found")
    )]
    UnexpectedEof(#[source_code] NamedSource, #[label] SourceSpan),

    #[error("Invalid assignment target")]
    #[diagnostic(
        code(nak::invalid_assign_target),
        help("You can only assign to variables")
    )]
    InvalidAssignmentTarget(#[source_code] NamedSource, #[label] SourceSpan),

    #[error("Unsupported operation")]
    #[diagnostic(
        code(nak::unsupported_operation)
    )]
    UnsupportedOperation(
        #[source_code] NamedSource,
        #[label("This operation doesn't support these types")] SourceSpan,
        #[label("{3}")] SourceSpan,
        Type,
        #[label("{5}")] SourceSpan,
        Type
    ),

    #[error("Unsupported unary operation")]
    #[diagnostic(
        code(nak::unsupported_unary_operation)
    )]
    UnsupportedUnaryOperation(
        #[source_code] NamedSource,
        #[label("This operation doesn't support this type")] SourceSpan,
        #[label("{3}")] SourceSpan,
        Type
    ),

    #[error("Undeclared variable")]
    #[diagnostic(
        code(nak::undeclared_variable),
        help("Consider adding 'let {2} = ...' before it's usage")
    )]
    UndeclaredVariable(
        #[source_code] NamedSource,
        #[label("This variable has not been declared")] SourceSpan,
        String,
    ),

    #[error("Incompatible types")]
    #[diagnostic(
        code(nak::incompatible_types)
    )]
    IncompatibleTypes(
        #[source_code] NamedSource,
        #[label("Expects types compatible with {2}")] SourceSpan,
        Type,
        #[label("{4}")] SourceSpan,
        Type
    )
}
