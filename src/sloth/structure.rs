use crate::sloth::types::Type;
use crate::sloth::value::Value;




#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct StructSignature {
    pub module: Option<String>,
    pub name: String
}

impl StructSignature {
    pub fn new(module: Option<String>, name: String) -> StructSignature {
        StructSignature { module, name }
    }
}








pub trait SlothObject: std::fmt::Debug {
    fn get_signature(&self) -> StructSignature;
    fn get_field(&self, field_name: String) -> Result<Value, String>;
    fn set_field(&mut self, field_name: String, value: Value) -> Result<(), String>;
}

















#[derive(Clone, Debug, PartialEq)]
pub struct StructDefinition {
    pub name: String,
    pub fields_names: Vec<String>,
    pub fields_types: Vec<Box<Type>>,
    pub module: Option<String>,
}


impl StructDefinition {
    pub fn new(name: String, fields_names: Vec<String>, fields_types: Vec<Box<Type>>, module: Option<String>) -> StructDefinition {
        StructDefinition { name, fields_names, fields_types, module }
    }
}