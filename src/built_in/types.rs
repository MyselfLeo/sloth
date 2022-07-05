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
        
        
        Ok(())
    }
}
