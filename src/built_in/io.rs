use crate::errors::Error;
use crate::sloth::function::{SlothFunction, FunctionSignature, Callable};
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::types::Type;
use crate::sloth::value::Value;
use std::io::{self, Write};
use text_io::read;

use sloth_derive::SlothFunction;









pub const BUILTINS: [&str; 2] = [
    "print",
    "read",
];


/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "print" => Box::new(BuiltinIoPrint {}),
        "read" => Box::new(BuiltinIoRead {}),
        n => panic!("Requested unknown built-in '{}'", n)
    }
}












#[derive(SlothFunction)]
#[name = "print"] #[module = "io"] #[output = "num"]
pub struct BuiltinIoPrint {}
impl Callable for BuiltinIoPrint {
    unsafe fn call(&self, scope: &mut Scope, _: &mut SlothProgram) -> Result<(), Error> {
        let inputs = scope.get_inputs();
        let mut text = String::new();

        for (_, v) in inputs.iter().enumerate() {
            text += &format!("{}", v).replace("\\n", "\n");
        }
        print!("{}", text);

        std::io::stdout().flush().unwrap();
        
        Ok(())
    }
}





#[derive(SlothFunction)]
#[name = "read"] #[module = "io"] #[output = "string"]
pub struct BuiltinIoRead {}
impl Callable for BuiltinIoRead {
    unsafe fn call(&self, scope: &mut Scope, _: &mut SlothProgram) -> Result<(), Error> {
        let inputs = scope.get_inputs();
        let mut text = String::new();

        for (_, v) in inputs.iter().enumerate() {
            text += &format!("{}", v).replace("\\n", "\n");
        }
        print!("{}", text);

        io::stdout().flush().unwrap();

        let console_input: String = read!("{}\n");
        let return_value = Value::String(console_input);
        scope.set_variable("@return".to_string(), return_value);
        Ok(())
    }
}