use meta::Span;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Null,
    Any
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeExpression {
    pub ty: Type,
    pub span: Span
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Int => "int",
            Self::Float => "float", 
            Self::Bool => "bool",
            Self::String => "string",
            Self::Null => "null",
            Self::Any => "any"
        })
    }
}
