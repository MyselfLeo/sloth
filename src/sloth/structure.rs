use std::collections::HashMap;

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






/// The fields throught which the user can interact with the Object
#[derive(Clone, Debug, PartialEq)]
pub struct StructDefinition {
    pub signature: StructSignature,
    pub fields: Vec<(String, Type)>,
}


impl StructDefinition {
    pub fn new(signature: StructSignature, fields: Vec<(String, Type)>) -> StructDefinition {
        StructDefinition {signature, fields}
    }

    /// should not be called without knowing the field exists first
    pub fn get_field_type(&self, field_name: &String) -> Result<Type, String> {
        for (n, t) in &self.fields {
            if n == field_name {return Ok(t.clone())}
        }
        panic!("get_field_type() called on a non-existant field")
    }
}








/// An object with custom behaviors that can be stored in a Value enum. From the point of view of the program, it
/// behaves like a structure, but it can have other features hidden from the user.
pub trait SlothObject: std::fmt::Debug {
    fn box_clone(&self) -> Box<dyn SlothObject>;
    fn get_signature(&self) -> StructSignature;
    fn get_definition(&self) -> StructDefinition;
    fn get_field(&self, field_name: &String) -> Result<Value, String>;
    fn set_field(&mut self, field_name: &String, value: Value) -> Result<(), String>;
    fn get_fields(&self) -> (Vec<String>, Vec<Value>);
}



impl Clone for Box<dyn SlothObject> {
    fn clone(&self) -> Box<dyn SlothObject> {
        self.box_clone()
    }
}


impl PartialEq for Box<dyn SlothObject> {
    fn eq(&self, other: &Self) -> bool {
        self.get_signature() == other.get_signature()
    }
}









#[derive(Debug, Clone)]
/// Object created from a structure defined in Sloth.
pub struct StructureObject {
    definition: StructDefinition,
    fields: HashMap<String, Value>,
}

impl StructureObject {
    pub fn new(definition: StructDefinition, fields: HashMap<String, Value>) -> StructureObject {
        StructureObject {definition, fields }
    }
}

impl SlothObject for StructureObject {
    fn box_clone(&self) -> Box<dyn SlothObject> {
        Box::new(self.clone())
    }

    fn get_signature(&self) -> StructSignature {
        self.definition.signature.clone()
    }

    fn get_definition(&self) -> StructDefinition {
        self.definition.clone()
    }

    fn get_field(&self, field_name: &String) -> Result<Value, String> {
        match self.fields.get(field_name) {
            Some(v) => Ok(v.clone()),
            None => Err(format!("Structure '{}' does not have a field named '{}'", self.get_signature().name, field_name))
        }
    }

    fn set_field(&mut self, field_name: &String, value: Value) -> Result<(), String> {
        let t = self.definition.get_field_type(field_name)?;
        if t != value.get_type() {
            Err(format!("Field '{}' expects a value of type '{}', got a value of type '{}' instead", field_name, t, value.get_type()))
        }
        else {
            self.fields.insert(field_name.clone(), value);
            Ok(())
        }
    }

    fn get_fields(&self) -> (Vec<String>, Vec<Value>) {
        let mut res = (Vec::new(), Vec::new());

        for (k, v) in &self.fields {
            res.0.push(k.clone());
            res.1.push(v.clone());
        }

        res
    }
}




