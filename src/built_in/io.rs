use crate::errors::Error;
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use crate::sloth::types::Type;



const FUNCTIONS: [&str; 1] = [
    "print",
];


const STRUCTS: [&str; 0] = [
];





struct BuiltinIoPrint {}

impl SlothFunction for BuiltinIoPrint {
    fn get_name(&self) -> String {
        "print".to_string()
    }
    fn get_output_type(&self) -> Type {
        Type::Number
    }
    unsafe fn call(&self, scope: &mut Scope, _: &mut SlothProgram) -> Result<(), Error> {
        let args = scope.get_inputs();
        for v in args {
            print!("{}", v.to_string())
        };
        Ok(())
    }
}