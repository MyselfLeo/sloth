#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Boolean,
    Number,
    String,
    List(Box<Type>),     // type of the list elements
    Struct(String)   // name of the string
}


impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Type::Boolean => write!(f, "bool"),
            Type::Number => write!(f, "num"),
            Type::String => write!(f, "string"),
            Type::List(t) => write!(f, "list[{}]", t),
            Type::Struct(n) => write!(f, "{}", n),
        }
    }
}