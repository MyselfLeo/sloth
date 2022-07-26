use crate::errors::{Error, ErrorMessage};
use crate::sloth::function::Callable;
use crate::sloth::function::{SlothFunction, FunctionSignature};
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::types::Type;
use crate::sloth::value::Value;

use sloth_derive::SlothFunction;





pub const BUILTINS: [&str; 2] = [
    "to_num",
    "len"
];



/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "to_num" => Box::new(BuiltinStringToNum {}),
        "len" => Box::new(BuiltinStringLen {}),
        n => panic!("Requested unknown built-in '{}'", n)
    }
}




#[derive(SlothFunction)]
#[name = "to_num"] #[module = "strings"] #[output = "num"] #[owner = "string"]
pub struct BuiltinStringToNum {}
impl Callable for BuiltinStringToNum {
    unsafe fn call(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
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
}



#[derive(SlothFunction)]
#[name = "len"] #[module = "strings"] #[output = "num"] #[owner = "string"]
pub struct BuiltinStringLen {}
impl Callable for BuiltinStringLen {
    unsafe fn call(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
        let value = scope.get_variable("@self".to_string(), program).unwrap();

        let result = match value {
            Value::String(x) => Value::Number(x.len() as f64),
            _ => panic!("Implementation of method 'to_string' for type 'num' was called on a value of another type")
        };

        scope.set_variable("@return".to_string(), result);
        Ok(())
    }
}