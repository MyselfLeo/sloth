use super::value::Value;

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub enum Type {
    Any,             // used in lists of size 0
    Boolean,
    Number,
    String,
    List(Box<Type>),     // type of the list elements
    Object(String)       // name of the string
}


impl Type {
    /// Return the default value for this type
    pub fn default(&self) -> Value {
        match &self {
            Type::Any => Value::Number(0.0), // Default value is a Number, should not cause problems as 'Any' type is only used in builtins, which should be ok with it
            Type::Boolean => Value::Boolean(false),
            Type::Number => Value::Number(0.0),
            Type::String => Value::String("".to_string()),
            Type::List(t) => Value::List(*t.clone(), Vec::new()),
            Type::Object(_s) => Value::Number(0.0), // TEMPORARY TODO
        }
    }

    /// Strict comparaison of types: the Type::Any won't match with any other type except Type::Any
    pub fn strict_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0, // List[Any] is 'equal' to every other lists
            (Self::Object(l0), Self::Object(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}


impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Type::Any => write!(f, "any"),
            Type::Boolean => write!(f, "bool"),
            Type::Number => write!(f, "num"),
            Type::String => write!(f, "string"),
            Type::List(t) => write!(f, "list[{}]", t),
            Type::Object(n) => write!(f, "{}", n),
        }
    }
}




impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0.strict_eq(&Type::Any) || r0.strict_eq(&Type::Any) || l0 == r0, // List[Any] is 'equal' to every other lists
            (Self::Object(l0), Self::Object(r0)) => l0 == r0,
            (Type::Any, _) => true,
            (_, Type::Any) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}