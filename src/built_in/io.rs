use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::BuiltInFunction;
use std::io::{self, Write};
use text_io::read;





pub const BUILTINS: [&str; 2] = [
    "print",
    "read",
];



/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "print" => Box::new(
            BuiltInFunction::new(
                "print",
                Some("io"),
                None,
                Type::Number,
                print
            )
        ),

        "read" => Box::new(
            BuiltInFunction::new(
                "read",
                Some("io"),
                None,
                Type::String,
                read
            )
        ),

        n => panic!("Requested unknown built-in '{}'", n)
    }
}









fn print(scope: &mut Scope, _: &mut SlothProgram) -> Result<(), Error> {
    let inputs = scope.get_inputs();
    let mut text = String::new();

    for (_, v) in inputs.iter().enumerate() {
        text += &format!("{}", v).replace("\\n", "\n");
    }
    print!("{}", text);

    std::io::stdout().flush().unwrap();
    
    Ok(())
}






fn read(scope: &mut Scope, _: &mut SlothProgram) -> Result<(), Error> {
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