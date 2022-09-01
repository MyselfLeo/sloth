use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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






/// Trait stored in the Program. Used to build SlothObject
pub trait ObjectBlueprint {
    fn box_clone(&self) -> Box<dyn ObjectBlueprint>;
    fn get_signature(&self) -> StructSignature;
    fn build(&self, given_values: Vec<Rc<RefCell<Value>>>) -> Result<Box<dyn SlothObject>, String>;
}








/// Structure defined in Sloth
#[derive(Clone, Debug, PartialEq)]
pub struct CustomDefinition {
    pub signature: StructSignature,
    pub fields: Vec<(String, Type)>,
}

impl CustomDefinition {
    pub fn new(signature: StructSignature, fields: Vec<(String, Type)>) -> CustomDefinition {
        CustomDefinition { signature, fields }
    }

    /// should not be called without knowing the field exists first
    pub fn get_field_type(&self, field_name: &String) -> Result<Type, String> {
        for (n, t) in &self.fields {
            if n == field_name {return Ok(t.clone())}
        }
        panic!("get_field_type() called on a non-existant field")
    }
}

impl ObjectBlueprint for CustomDefinition {
    fn box_clone(&self) -> Box<dyn ObjectBlueprint> {
        Box::new(self.clone())
    }

    fn get_signature(&self) -> StructSignature {
        self.signature.clone()
    }

    fn build(&self, given_values: Vec<Rc<RefCell<Value>>>) -> Result<Box<dyn SlothObject>, String> {
        // Compare lenght of given fields to the struct def
        if self.fields.len() != given_values.len() {
            return Err(format!("Structure '{}' expects {} fields, but it has been given {} fields", self.signature.name, self.fields.len(), given_values.len()));
        }

        let mut result = HashMap::new();

        // Compare each given value to the fields of the Structure, checking their type
        for (given_value, (field_name, expected_type)) in std::iter::zip(given_values, self.fields.clone()) {
            let borrow = given_value.borrow();
            if borrow.get_type() != expected_type {
                return Err(format!("Field '{}' of structure '{}' is of type '{}', but it has been given a value of type '{}'", field_name, self.signature.name, expected_type, borrow.get_type()))
            }
            result.insert(field_name, given_value.clone());
        }

        return Ok(Box::new(StructureObject::new(self.clone(), result)))
    }
}









/// Trait used to allow for downcasting Trait Objects into their corresponding structs
pub trait ObjectToAny: 'static {
    fn as_any(&mut self) -> &mut dyn Any;
}


impl<T: 'static> ObjectToAny for T {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}











/// An object with custom behaviors that can be stored in a Value enum. From the point of view of the program, it
/// behaves like a structure, but it can have other features hidden from the user.
pub trait SlothObject: ObjectToAny {
    fn box_clone(&self) -> Box<dyn SlothObject>;
    fn get_signature(&self) -> StructSignature;
    fn get_blueprint(&self) -> Box<dyn ObjectBlueprint>;
    fn get_field(&self, field_name: &String) -> Result<Rc<RefCell<Value>>, String>;
    fn get_fields(&self) -> (Vec<String>, Vec<Rc<RefCell<Value>>>);
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
    definition: CustomDefinition,
    fields: HashMap<String, Rc<RefCell<Value>>>,
}

impl StructureObject {
    pub fn new(definition: CustomDefinition, fields: HashMap<String, Rc<RefCell<Value>>>) -> StructureObject {
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

    fn get_blueprint(&self) -> Box<dyn ObjectBlueprint> {
        self.definition.box_clone()
    }

    fn get_field(&self, field_name: &String) -> Result<Rc<RefCell<Value>>, String> {
        match self.fields.get(field_name) {
            Some(v) => Ok(v.clone()),
            None => Err(format!("Structure '{}' does not have a field named '{}'", self.get_signature().name, field_name))
        }
    }

    fn get_fields(&self) -> (Vec<String>, Vec<Rc<RefCell<Value>>>) {
        let mut res = (Vec::new(), Vec::new());

        for (k, v) in &self.fields {
            res.0.push(k.clone());
            res.1.push(v.clone());
        }

        res
    }
}




