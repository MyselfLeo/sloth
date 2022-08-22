use crate::errors::ErrorMessage;
use crate::sloth::structure::ObjectBlueprint;
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::{BuiltInFunction, BuiltinTypes};





pub const BUILTINS: [&str; 5] = [
    "to_num",
    "len",
    "insert",
    "push",
    "remove"
];




/// Return whether each builtin is a function or a structure
pub fn get_type(builtin: &String) -> Result<BuiltinTypes, String> {
    match builtin.as_str() {
        "to_num" => Ok(BuiltinTypes::Function),
        "len" => Ok(BuiltinTypes::Function),
        _ => Err(format!("Builtin '{builtin}' not found in module 'strings'"))
    }
}







/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "to_num" => Box::new(
            BuiltInFunction::new(
                "to_num",
                Some("strings"),
                Some(Type::String),
                Type::Number,
                to_num
            )
        ),

        "len" => Box::new(
            BuiltInFunction::new(
                "len",
                Some("strings"),
                Some(Type::String),
                Type::Number,
                len
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

















fn to_num(scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
    let value = scope.get_variable("@self".to_string(), program).unwrap();

    let result = match value {
        Value::String(x) => {
            match x.parse::<f64>() {
                Ok(v) => Value::Number(v),
                Err(_) => {
                    let err_msg = format!("Cannot parse string \"{}\" into a Number", x);
                    return Err(Error::new(ErrorMessage::RuntimeError(err_msg), None));
                }
            }
        },
        _ => panic!("Implementation of method 'to_num' for type 'string' was called on a value of another type")
    };

    scope.set_variable("@return".to_string(), result);
    Ok(())
}










fn len(scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
    let value = scope.get_variable("@self".to_string(), program).unwrap();

    let result = match value {
        Value::String(x) => Value::Number(x.len() as f64),
        _ => panic!("Implementation of method 'len' for type 'string' was called on a value of another type")
    };

    scope.set_variable("@return".to_string(), result);
    Ok(())
}