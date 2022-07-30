use crate::errors::ErrorMessage;
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::BuiltInFunction;





pub const BUILTINS: [&str; 2] = [
    "to_num",
    "len"
];



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
                "to_num",
                Some("strings"),
                Some(Type::String),
                Type::Number,
                len
            )
        ),


        n => panic!("Requested unknown built-in '{}'", n)
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