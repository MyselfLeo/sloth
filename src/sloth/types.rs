#[derive(Debug, Clone)]
pub enum Type {
    Boolean,
    Number,
    String,
    List(Box<Type>),     // type of the list elements
    Struct(String)   // name of the string
}