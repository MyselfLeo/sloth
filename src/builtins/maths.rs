use crate::errors::ErrMsg;
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::{BuiltInFunction, BuiltinTypes};
use crate::sloth::structure::ObjectBlueprint;
use std::cell::RefCell;
use std::rc::Rc;




pub const BUILTINS: [&str; 2] = [
    "pow",
    "sqrt"
];




/// Return whether each builtin is a function or a structure
pub fn get_type(builtin: &String) -> Result<BuiltinTypes, String> {
    match builtin.as_str() {
        "pow" => Ok(BuiltinTypes::Function),
        "sqrt" => Ok(BuiltinTypes::Function),

        _ => Err(format!("Builtin '{builtin}' not found in module 'maths'"))
    }
}





/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "pow" => Box::new(
            BuiltInFunction::new(
                "pow",
                Some("maths"),
                Some(Type::Number),
                Type::Number,
                pow
            )
        ),

        "sqrt" => Box::new(
            BuiltInFunction::new(
                "sqrt",
                Some("maths"),
                Some(Type::Number),
                Type::Number,
                sqrt
            )
        ),


        n => panic!("Requested unknown built-in function '{}'", n)
    }
}





/// Return a StructDefinition along with the list of requirements this structure has
pub fn get_struct(s_name: String) -> (Box<dyn ObjectBlueprint>, Vec<String>) {
    match s_name.as_str() {



        s => panic!("Requested unknown built-in structure '{}'", s)
    }
}






























fn pow(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let value_self = super::get_self(&scope, program)?;
    let inputs = super::query_inputs(&scope, vec![Type::Number], "pow")?;

    let power = match inputs[0] {
        Value::Number(x) => x,
        _ => panic!("'query_inputs' failed to meet its little small goal")
    };

    let result = match value_self {
        Value::Number(x) => Value::Number(x.powf(power)),
        _ => panic!("Implementation of method 'pow' for type 'num' was called on a value of another type")
    };

    super::set_return(&scope, program, result)
}











fn sqrt(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let value_self = super::get_self(&scope, program)?;

    let result = match value_self {
        Value::Number(x) => {
            if x < 0.0 {
                let err_msg = format!("Called sqrt on a negative number ({})", x);
                return Err(Error::new(ErrMsg::InvalidArguments(err_msg), None))
            }
            Value::Number(x.sqrt())
        },
        _ => panic!("Implementation of method 'sqrt' for type 'num' was called on a value of another type")
    };

    super::set_return(&scope, program, result)
}