use crate::parser::parse_error::ParseError;
use syntax::SyntaxKind;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Event {
    StartNode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    AddToken,
    FinishNode,
    Error(ParseError),
    Placeholder,
}
