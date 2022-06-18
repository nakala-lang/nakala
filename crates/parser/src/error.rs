use ast::ty::Type;
use lexer::TokenKind;
use meta::SourceId;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    #[error("Expected an expression")]
    #[diagnostic(
        code(nak::cant_parse_primary_expr),
        help("Change this into an expression")
    )]
    ExpectedExpression(SourceId, #[label] SourceSpan),

    #[error("Expected token '{2}', but found '{1}' instead")]
    #[diagnostic(code(nak::expected_token))]
    ExpectedToken(
        SourceId,
        String,
        TokenKind,
        #[label("Consider adding '{2}' here")] SourceSpan,
    ),

    #[error("Unexpected EOF")]
    #[diagnostic(
        code(nak::unexpected_eof),
        help("Expected more tokens, but none were found")
    )]
    UnexpectedEof(SourceId, #[label] SourceSpan),

    #[error("Invalid assignment target")]
    #[diagnostic(
        code(nak::invalid_assign_target),
        help("You can only assign to variables")
    )]
    InvalidAssignmentTarget(SourceId, #[label] SourceSpan),

    #[error("Unsupported operation")]
    #[diagnostic(code(nak::unsupported_operation))]
    UnsupportedOperation(
        SourceId,
        #[label("This operation doesn't support these types")] SourceSpan,
        #[label("{3}")] SourceSpan,
        Type,
        #[label("{5}")] SourceSpan,
        Type,
    ),

    #[error("Unsupported unary operation")]
    #[diagnostic(code(nak::unsupported_unary_operation))]
    UnsupportedUnaryOperation(
        SourceId,
        #[label("This operation doesn't support this type")] SourceSpan,
        #[label("{3}")] SourceSpan,
        Type,
    ),

    #[error("Undeclared variable")]
    #[diagnostic(
        code(nak::undeclared_variable),
        help("Consider adding 'let {2} = ...' before it's usage")
    )]
    UndeclaredVariable(
        SourceId,
        #[label("This variable has not been declared")] SourceSpan,
        String,
    ),

    #[error("Incompatible types")]
    #[diagnostic(code(nak::incompatible_types))]
    IncompatibleTypes(
        SourceId,
        #[label("Expects types compatible with {2}")] SourceSpan,
        Type,
        #[label("{4}")] SourceSpan,
        Type,
    ),

    #[error("Unknown type")]
    #[diagnostic(
        code(nak::unknown_type),
        help("Consider using 'int', 'float', 'bool', etc.")
    )]
    UnknownType(SourceId, #[label("This type is unknown")] SourceSpan),

    #[error("Uncallable expression")]
    #[diagnostic(
        code(nak::uncallable_expr),
        help("Only classes and functions are callable")
    )]
    UncallableExpression(SourceId, #[label("'{2}' is uncallable")] SourceSpan, Type),

    #[error("Only instances have properties")]
    #[diagnostic(code(nak::only_instances_have_properties))]
    OnlyInstancesAndClassesHaveProperties(
        SourceId,
        #[label("Expected instance type, but got {2} instead")] SourceSpan,
        Type,
    ),

    #[error("Function returns incompatible type")]
    #[diagnostic(code(nak::incompatible_types))]
    FunctionHasIncompatibleReturnType(
        SourceId,
        #[label("Expected a type that is compatible with {2}")] SourceSpan,
        Type,
        #[label("Found type {4} instead")] SourceSpan,
        Type,
    ),

    #[error("Can't return from global scope")]
    #[diagnostic(
        code(nak::cannot_return_from_global_scope),
        help("Can only return from within function scopes")
    )]
    CantReturnFromGlobalScope(
        SourceId,
        #[label("Cannot return from this scope")] SourceSpan,
    ),

    #[error("List shorthand count must be of type int")]
    #[diagnostic(code(nak::list_shorthand_count_must_be_int))]
    ListShorthandCountMustBeInt(
        SourceId,
        #[label("This must be compatible with int")] SourceSpan,
    ),
}
