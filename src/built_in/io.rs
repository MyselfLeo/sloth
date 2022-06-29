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





pub struct BuiltinIoPrint {}

impl SlothFunction for BuiltinIoPrint {
    fn get_name(&self) -> String {
        "print".to_string()
    }
    fn get_output_type(&self) -> Type {
        Type::Number
    }
    unsafe fn call(&self, scope: &mut Scope, _: &mut SlothProgram) -> Result<(), Error> {
        let inputs = scope.get_inputs();
        let mut text = String::new();

        for (i, v) in inputs.iter().enumerate() {
            text += &format!("{}", v).replace("\\n", "\n");
            if i < inputs.len() - 1 {text += " "}
        }
        print!("{}", text);

        Ok(())
    }
}