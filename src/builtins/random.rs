use crate::errors::ErrMsg;
use crate::sloth::structure::ObjectBlueprint;
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::{BuiltInFunction, BuiltinTypes};
use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;



pub const BUILTINS: [&str; 1] = [
    "range"
];




/// Return whether each builtin is a function or a structure
pub fn get_type(builtin: &String) -> Result<BuiltinTypes, String> {
    match builtin.as_str() {
        "range" => Ok(BuiltinTypes::Function),
        _ => Err(format!("Builtin '{builtin}' not found in module 'random'"))
    }
}







/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "range" => Box::new(
            BuiltInFunction::new(
                "range",
                Some("random"),
                None,
                Type::Number,
                range
            )
        ),


        n => panic!("Requested unknown built-in '{}'", n)
    }
}







/// Return a StructDefinition along with the list of requirements this structure has
pub fn get_struct(s_name: String) -> (Box<dyn ObjectBlueprint>, Vec<String>) {
    match s_name.as_str() {
        s => panic!("Requested unknown built-in structure '{}'", s)
    }
}


















fn range(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let inputs = super::query_inputs(&scope, vec![Type::Number, Type::Number], "range")?;

    let (min, max) = match (&inputs[0], &inputs[1]) {
        (Value::Number(x), Value::Number(y)) => (x, y),
        _ => panic!()
    };

    if max - min < 0.0 {
        let err_msg = format!("The first value (min) should be lower than the second value (max). Given {} and {}", min, max);
        return Err(Error::new(ErrMsg::InvalidArguments(err_msg), None))
    }
    if min == max {
        return super::set_return(&scope, program, Value::Number(*min))
    } 

    let mut rng = rand::thread_rng();
    super::set_return(&scope, program, Value::Number(rng.gen_range(*min..*max)))
}