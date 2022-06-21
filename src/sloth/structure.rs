use crate::sloth::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct StructDefinition {
    pub name: String,
    pub fields_names: Vec<String>,
    pub fields_types: Vec<Box<Type>>,
}