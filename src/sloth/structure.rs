use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::iter::zip;

use crate::sloth::types::Type;
use crate::sloth::value::Value;

use super::value::RecursiveRereference;




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
        for (mut given_value, (field_name, expected_type)) in std::iter::zip(given_values, self.fields.clone()) {

            // special case for lists: if the given list is EMPTY (so it's a list of type Any), make its type the same as the type of the required LIST
            let res = {
                if let Type::List(t_r) = &expected_type {
                    if let Type::List(t_g) = given_value.borrow().get_type() {
                        if (*t_g).strict_eq(&Type::Any) {
                            Rc::new(RefCell::new(Value::List((**t_r).clone(), Vec::new())))
                        }
                        else {given_value.clone()}
                    }
                    else {given_value.clone()}
                }
                else {given_value.clone()}
            };

            given_value = res;

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
pub trait SlothObject: ObjectToAny + std::fmt::Display {
    fn box_clone(&self) -> Box<dyn SlothObject>;
    fn get_signature(&self) -> StructSignature;
    fn get_blueprint(&self) -> Box<dyn ObjectBlueprint>;
    fn get_field(&self, field_name: &String) -> Result<Rc<RefCell<Value>>, String>;
    fn get_fields(&self) -> (Vec<String>, Vec<Rc<RefCell<Value>>>);

    /// Return a clone of the object, with all its inner values re-allocated
    fn rereference(&self) -> Box<dyn SlothObject>;
}



impl Clone for Box<dyn SlothObject> {
    fn clone(&self) -> Box<dyn SlothObject> {
        self.box_clone()
    }
}


impl PartialEq for Box<dyn SlothObject> {
    fn eq(&self, other: &Self) -> bool {
        if self.get_signature() != other.get_signature() {return false}

        let self_fields = self.get_fields().1;
        let other_fields = other.get_fields().1;

        for i in 0..self_fields.len() {
            if self_fields[i].borrow().to_owned() != other_fields[i].borrow().to_owned() {return false}
        }
        true
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

        // Required so the fields are given in the correct order (as the Hashmap is not sorted)
        let definition_order = &self.definition.fields;

        for (field_name, _) in definition_order {
            res.0.push(field_name.clone());
            res.1.push(self.fields.get(field_name).unwrap().clone());
        }

        res
    }

    fn rereference(&self) -> Box<dyn SlothObject> {
        let mut new_fields = HashMap::new();
        for (k,v) in &self.fields {
            new_fields.insert(k.clone(), v.borrow().to_owned().rereference());
        };

        Box::new(StructureObject::new(self.definition.clone(), new_fields))
    }
}


impl std::fmt::Display for StructureObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fields = self.get_fields();
        let fields_str = zip(fields.0, fields.1)
                                .map(|(s, v)| format!("{s}: {}", v.borrow().to_owned()))
                                .collect::<Vec<String>>()
                                .join(", ");
        write!(f, "{} ({})", self.get_signature().name, fields_str)
    }
}



