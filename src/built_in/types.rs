use crate::errors::ErrorMessage;
use crate::{errors::Error, sloth::function::FunctionID};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::types::Type;
use crate::sloth::value::Value;


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





pub struct BuiltinTypesToString {}

impl SlothFunction for BuiltinTypesToString {
    fn get_name(&self) -> String {"to_string".to_string()}
    fn get_owner_type(&self) -> Option<Type> {None}
    fn get_module(&self) -> Option<String> {Some("types".to_string())}

    fn get_function_id(&self) -> FunctionID {
        FunctionID::new(self.get_module(), self.get_name(), self.get_owner_type())
    }

    fn get_output_type(&self) -> Type {
        Type::String
    }
    unsafe fn call(&self, scope: &mut Scope, _: &mut SlothProgram) -> Result<(), Error> {
        let inputs = scope.get_inputs();
        
        if inputs.len() != 1 {
            let err_msg = format!("Called function 'to_string' with {} argument(s), but the function requires 1 argument", &inputs.len());
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None))
        }

        let value = inputs[0].clone();
        let result = match value {
            Value::Number(x) => Value::String(x.to_string()),
            Value::Boolean(v) => if v {Value::String("true".to_string())} else {Value::String("false".to_string())},
            Value::List(_, _) => unimplemented!("Converting a list to a string is not implemented yet"),
            Value::Struct(_, _) => return Err(Error::new(ErrorMessage::InvalidArguments("Can't convert an object into a string".to_string()), None)),
            string => string    // in case of Value::String
        };

        scope.set_variable("@return".to_string(), result);
        Ok(())
    }
}
