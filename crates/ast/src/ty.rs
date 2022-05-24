use meta::Span;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Class(String),
    Instance(String),
    Null,
    Any
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeExpression {
    pub ty: Type,
    pub span: Span
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: String = match self {
            Self::Int => String::from("int"),
            Self::Float => String::from("float"), 
            Self::Bool => String::from("bool"),
            Self::String => String::from("string"),
            Self::Class(name) => name.clone(),
            Self::Instance(name) => format!("instanceof {name}"),
            Self::Null => String::from("null"),
            Self::Any => String::from("any")
        };

        f.write_str(format!("{}", msg).as_str())
    }
}
