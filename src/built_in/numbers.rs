use crate::errors::{Error, ErrorMessage};
use crate::sloth::function::Callable;
use crate::sloth::function::{SlothFunction, FunctionSignature};
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::types::Type;
use crate::sloth::value::Value;

use sloth_derive::SlothFunction;





pub const BUILTINS: [&str; 3] = [
    "to_string",
    "square",
    "sqrt"
];


/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "to_string" => Box::new(BuiltinTypesToString {}),
        "square" => Box::new(BuiltinTypesSquare {}),
        "sqrt" => Box::new(BuiltinTypesSqrt {}),
        n => panic!("Requested unknown built-in '{}'", n)
    }
}





#[derive(SlothFunction)]
#[name = "to_string"] #[module = "numbers"] #[output = "string"] #[owner = "num"]
pub struct BuiltinTypesToString {}
impl Callable for BuiltinTypesToString {
    unsafe fn call(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
        let value = scope.get_variable("@self".to_string(), program).unwrap();

        let result = match value {
            Value::Number(x) => Value::String(x.to_string()),
            _ => panic!("Implementation of method 'to_string' for type 'num' was called on a value of another type")
        };

        scope.set_variable("@return".to_string(), result);
        Ok(())
    }
}




#[derive(SlothFunction)]
#[name = "square"] #[module = "numbers"] #[output = "num"] #[owner = "num"]
pub struct BuiltinTypesSquare {}
impl Callable for BuiltinTypesSquare {
    unsafe fn call(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
        let value = scope.get_variable("@self".to_string(), program).unwrap();

        let result = match value {
            Value::Number(x) => Value::Number(x * x),
            _ => panic!("Implementation of method 'to_string' for type 'num' was called on a value of another type")
        };

        scope.set_variable("@return".to_string(), result);
        Ok(())
    }
}



#[derive(SlothFunction)]
#[name = "sqrt"] #[module = "numbers"] #[output = "num"] #[owner = "num"]
pub struct BuiltinTypesSqrt {}
impl Callable for BuiltinTypesSqrt {
    unsafe fn call(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
        let value = scope.get_variable("@self".to_string(), program).unwrap();


        let result = match value {
            Value::Number(x) => {
                if x < 0.0 {
                    return Err(Error::new(ErrorMessage::InvalidArguments(format!("Called sqrt on a negative number ({})", x)), None))}

                Value::Number(x.sqrt())
            },
            _ => panic!("Implementation of method 'to_string' for type 'num' was called on a value of another type")
        };

        scope.set_variable("@return".to_string(), result);
        Ok(())
    }
}