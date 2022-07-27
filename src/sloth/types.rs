use super::value::Value;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Unknown,             // used in lists of size 0
    Boolean,
    Number,
    String,
    List(Box<Type>),     // type of the list elements
    Struct(String)       // name of the string
}

impl Type {
    /// Return the default value for this type
    pub fn default(&self) -> Value {
        match &self {
            Type::Unknown => panic!("Cannot generate a value from type Unknown"), // May use a real Value::Number after
            Type::Boolean => Value::Boolean(false),
            Type::Number => Value::Number(0.0),
            Type::String => Value::String("".to_string()),
            Type::List(t) => Value::List(*t.clone(), Vec::new()),
            Type::Struct(_s) => unimplemented!()
        }
    }
}


impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Type::Unknown => write!(f, "unknown"),
            Type::Boolean => write!(f, "bool"),
            Type::Number => write!(f, "num"),
            Type::String => write!(f, "string"),
            Type::List(t) => write!(f, "list[{}]", t),
            Type::Struct(n) => write!(f, "{}", n),
        }
    }
}