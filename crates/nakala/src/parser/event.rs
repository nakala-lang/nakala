use crate::lexer::SyntaxKind;
use rowan::SmolStr;

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Event {
    StartNode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    StartNodeAt {
        kind: SyntaxKind,
        checkpoint: usize,
    },
    AddToken {
        kind: SyntaxKind,
        text: SmolStr,
    },
    FinishNode,
    Placeholder,
}
