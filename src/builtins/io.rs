use crate::sloth::structure::{ObjectBlueprint};
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::{BuiltInFunction, BuiltinTypes};
use std::cell::RefCell;
use std::rc::Rc;
use std::io::{self, Write};
use text_io::read;





pub const BUILTINS: [&str; 2] = [
    "print",
    "read",
];


/// Return whether each builtin is a function or a structure
pub fn get_type(builtin: &String) -> Result<BuiltinTypes, String> {
    match builtin.as_str() {
        "print" => Ok(BuiltinTypes::Function),
        "read" => Ok(BuiltinTypes::Function),

        _ => Err(format!("Builtin '{builtin}' not found in module 'io'"))
    }
}



/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    let res = match f_name.as_str() {
        "print" => BuiltInFunction::new(
            "print",
            Some("io"),
            None,
            Type::Number,
            print
        ),

        "read" => BuiltInFunction::new(
            "read",
            Some("io"),
            None,
            Type::String,
            read
        ),

        n => panic!("Requested unknown built-in '{}'", n)
    };

    Box::new(res)
}











/// Return an ObjectBlueprint along with the list of requirements this structure has
pub fn get_struct(s_name: String) -> (Box<dyn ObjectBlueprint>, Vec<String>) {
    match s_name.as_str() {
        s => panic!("Requested unknown built-in structure '{}'", s)
    }
}























fn print(scope: Rc<RefCell<Scope>>, _: &mut SlothProgram) -> Result<(), Error> {
    let inputs = scope.borrow().get_inputs();
    let mut text = String::new();

    for (_, v) in inputs.iter().enumerate() {
        text += &format!("{}", v.borrow()).replace("\\n", "\n");
    }
    print!("{}", text);
    std::io::stdout().flush().unwrap();
    Ok(())
}






fn read(scope: Rc<RefCell<Scope>>, p: &mut SlothProgram) -> Result<(), Error> {
    let inputs = scope.borrow().get_inputs();
    let mut text = String::new();

    for (_, v) in inputs.iter().enumerate() {
        text += &format!("{}", v.borrow()).replace("\\n", "\n");
    }
    print!("{}", text);

    io::stdout().flush().unwrap();

    let console_input: String = read!("{}\n");
    let return_value = Value::String(console_input);

    super::set_return(&scope, p, return_value)
}