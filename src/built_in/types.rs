use crate::errors::ErrorMessage;
use crate::errors::Error;
use crate::sloth::function::Callable;
use crate::sloth::function::{SlothFunction, FunctionSignature};
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::types::Type;
use crate::sloth::value::Value;

use sloth_derive::SlothFunction;







pub const BUILTINS: [&str; 1] = [
    "to_string"
];


/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "to_string" => Box::new(BuiltinTypesToString {}),
        n => panic!("Requested unknown built-in '{}'", n)
    }
}





#[derive(SlothFunction)]
#[name = "to_string"] #[module = "types"] #[output = "string"] #[owner = "num"]
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