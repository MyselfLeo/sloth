use crate::sloth::structure::ObjectBlueprint;
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::{BuiltInFunction, BuiltinTypes};
use std::cell::RefCell;
use std::rc::Rc;



pub const BUILTINS: [&str; 1] = [
    "to_string"
];




/// Return whether each builtin is a function or a structure
pub fn get_type(builtin: &String) -> Result<BuiltinTypes, String> {
    match builtin.as_str() {
        "to_string" => Ok(BuiltinTypes::Function),
        _ => Err(format!("Builtin '{builtin}' not found in module 'numbers'"))
    }
}







/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "to_string" => Box::new(
            BuiltInFunction::new(
                "to_string",
                Some("numbers"),
                Some(Type::Number),
                Type::String,
                to_string
            )
        ),


        n => panic!("Requested unknown built-in '{}'", n)
    }
}







/// Return a StructDefinition along with the list of requirements this structure has
pub fn get_struct(s_name: String) -> (Box<dyn ObjectBlueprint>, Vec<String>) {
    match s_name.as_str() {
        s => panic!("Requested unknown built-in structure '{}'", s)
    }
}


















fn to_string(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let value = super::get_self(&scope, program)?;

    let result = match value {
        Value::Number(x) => Value::String(x.to_string()),
        _ => panic!("Implementation of method 'to_string' for type 'num' was called on a value of another type")
    };

    super::set_return(&scope, program, result)?;
    Ok(())
}