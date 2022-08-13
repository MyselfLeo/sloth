use crate::sloth::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct StructDefinition {
    pub name: String,
    pub fields_names: Vec<String>,
    pub fields_types: Vec<Box<Type>>,
}


impl StructDefinition {
    pub fn new(name: String, fields_names: Vec<String>, fields_types: Vec<Box<Type>>) -> StructDefinition {
        StructDefinition { name, fields_names, fields_types }
    }
}