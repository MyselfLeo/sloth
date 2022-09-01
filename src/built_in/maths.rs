use crate::errors::ErrorMessage;
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
    let scope_borrow = scope.borrow();

    let value = scope_borrow.get_variable("@self".to_string(), program).unwrap();
    let inputs = scope_borrow.get_inputs();


    let power = match inputs.get(0).map(|v| v.borrow().to_owned()) {
        Some(Value::Number(x)) => x,
        _ => {
            let err_msg = "The 'pow' method requires a 'num' input".to_string();
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
        }
    };

    let result = match value.borrow().to_owned() {
        Value::Number(x) => Value::Number(x.powf(power)),
        _ => panic!("Implementation of method 'pow' for type 'num' was called on a value of another type")
    };

    super::set_return(scope.clone(), program, result);
    Ok(())
}











fn sqrt(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let value = scope.borrow().get_variable("@self".to_string(), program).unwrap();

    let result = match value.borrow().to_owned() {
        Value::Number(x) => {
            if x < 0.0 {
                return Err(Error::new(ErrorMessage::InvalidArguments(format!("Called sqrt on a negative number ({})", x)), None))
            }

            Value::Number(x.sqrt())
        },
        _ => panic!("Implementation of method 'sqrt' for type 'num' was called on a value of another type")
    };

    super::set_return(scope, program, result);
    Ok(())
}