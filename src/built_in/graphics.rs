use crate::errors::ErrorMessage;
use crate::sloth::structure::{ObjectBlueprint, SlothObject};
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::{BuiltInFunction, BuiltinTypes};
use std::cell::RefCell;
use std::rc::Rc;





pub const BUILTINS: [&str; 0] = [
];


/// Return whether each builtin is a function or a structure
pub fn get_type(builtin: &String) -> Result<BuiltinTypes, String> {
    match builtin.as_str() {

        _ => Err(format!("Builtin '{builtin}' not found in module 'graphics'"))
    }
}



/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        /*
        "save" => Box::new(
            BuiltInFunction::new(
                "save",
                Some("files"),
                None,
                Type::Number,
                save
            )
        ),
        */

        n => panic!("Requested unknown built-in '{}'", n)
    }
}











/// Return an ObjectBlueprint along with the list of requirements this structure has
pub fn get_struct(s_name: String) -> (Box<dyn ObjectBlueprint>, Vec<String>) {
    match s_name.as_str() {
        s => panic!("Requested unknown built-in structure '{}'", s)
    }
}






pub struct CanvasBlueprint {}

impl ObjectBlueprint for CanvasBlueprint {
    fn box_clone(&self) -> Box<dyn ObjectBlueprint> {
        todo!()
    }

    fn get_signature(&self) -> crate::sloth::structure::StructSignature {
        todo!()
    }

    fn build(&self, given_values: Vec<Rc<RefCell<Value>>>) -> Result<Box<dyn crate::sloth::structure::SlothObject>, String> {
        todo!()
    }
}



pub struct Canvas {

}


impl std::fmt::Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl SlothObject for Canvas {
    fn box_clone(&self) -> Box<dyn SlothObject> {
        todo!()
    }

    fn get_signature(&self) -> crate::sloth::structure::StructSignature {
        todo!()
    }

    fn get_blueprint(&self) -> Box<dyn ObjectBlueprint> {
        todo!()
    }

    fn get_field(&self, field_name: &String) -> Result<Rc<RefCell<Value>>, String> {
        todo!()
    }

    fn get_fields(&self) -> (Vec<String>, Vec<Rc<RefCell<Value>>>) {
        todo!()
    }

    fn rereference(&self) -> Box<dyn SlothObject> {
        todo!()
    }
}