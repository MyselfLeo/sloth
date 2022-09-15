use crate::errors::ErrorMessage;
use crate::sloth::structure::{ObjectBlueprint};
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::{BuiltInFunction, BuiltinTypes};
use std::cell::RefCell;
use std::rc::Rc;
use std::fs;





pub const BUILTINS: [&str; 2] = [
    "load",
    "save"
];


/// Return whether each builtin is a function or a structure
pub fn get_type(builtin: &String) -> Result<BuiltinTypes, String> {
    match builtin.as_str() {
        "load" => Ok(BuiltinTypes::Function),
        "save" => Ok(BuiltinTypes::Function),

        _ => Err(format!("Builtin '{builtin}' not found in module 'files'"))
    }
}



/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "load" => Box::new(
            BuiltInFunction::new(
                "load",
                Some("files"),
                None,
                Type::String,
                load
            )
        ),

        "save" => Box::new(
            BuiltInFunction::new(
                "save",
                Some("files"),
                None,
                Type::Number,
                save
            )
        ),

        n => panic!("Requested unknown built-in '{}'", n)
    }
}











/// Return an ObjectBlueprint along with the list of requirements this structure has
pub fn get_struct(s_name: String) -> (Box<dyn ObjectBlueprint>, Vec<String>) {
    match s_name.as_str() {
        s => panic!("Requested unknown built-in structure '{}'", s)
    }
}














/// Return the content of a file as a string
fn load(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let inputs = super::query_inputs(&scope, vec![Type::String], "load")?;

    let path = match &inputs[0] {
        Value::String(x) => x,
        _ => panic!("'query_inputs' failed")
    };

    let content = match fs::read_to_string(&path) {
        Ok(f) => f,
        Err(e) => {
            let err_msg = format!("Could not open file '{}': {}", path, e.to_string());
            return Err(Error::new(ErrorMessage::RuntimeError(err_msg), None))
        },
    };

    super::set_return(&scope, program, Value::String(content))
}





/// Save the content of the string to a file with the given path
fn save(scope: Rc<RefCell<Scope>>, _: &mut SlothProgram) -> Result<(), Error> {
    let inputs = super::query_inputs(&scope, vec![Type::String, Type::String], "save")?;

    let path = match &inputs[0] {
        Value::String(x) => x,
        _ => panic!("'query_inputs' failed")
    };

    let string = match &inputs[1] {
        Value::String(x) => x,
        _ => panic!("'query_inputs' failed")
    };

    match fs::write(&path, string) {
        Ok(()) => Ok(()),
        Err(e) => {
            let err_msg = format!("Could not save to file '{}': {}", path, e.to_string());
            return Err(Error::new(ErrorMessage::RuntimeError(err_msg), None))
        },
    }
}