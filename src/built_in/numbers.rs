use crate::errors::ErrorMessage;
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::BuiltInFunction;




pub const BUILTINS: [&str; 3] = [
    "to_string",
    "pow",
    "sqrt"
];


/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "to_string" => Box::new(
            BuiltInFunction::new(
                "to_string",
                Some("numbers"),
                Some(Type::Number),
                Type::String,
                to_string
            )
        ),

        "pow" => Box::new(
            BuiltInFunction::new(
                "pow",
                Some("numbers"),
                Some(Type::Number),
                Type::Number,
                pow
            )
        ),

        "sqrt" => Box::new(
            BuiltInFunction::new(
                "sqrt",
                Some("numbers"),
                Some(Type::Number),
                Type::Number,
                sqrt
            )
        ),


        n => panic!("Requested unknown built-in '{}'", n)
    }
}












fn to_string(scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
    let value = scope.get_variable("@self".to_string(), program).unwrap();

    let result = match value {
        Value::Number(x) => Value::String(x.to_string()),
        _ => panic!("Implementation of method 'to_string' for type 'num' was called on a value of another type")
    };

    scope.set_variable("@return".to_string(), result);
    Ok(())
}











fn pow(scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
    let value = scope.get_variable("@self".to_string(), program).unwrap();
    let inputs = scope.get_inputs();


    let power = match inputs.get(0) {
        Some(Value::Number(x)) => *x,
        _ => {
            let err_msg = "The 'pow' method requires a 'num' input".to_string();
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
        }
    };

    let result = match value {
        Value::Number(x) => Value::Number(x.powf(power)),
        _ => panic!("Implementation of method 'pow' for type 'num' was called on a value of another type")
    };

    scope.set_variable("@return".to_string(), result);
    Ok(())
}











fn sqrt(scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
    let value = scope.get_variable("@self".to_string(), program).unwrap();

    let result = match value {
        Value::Number(x) => {
            if x < 0.0 {
                return Err(Error::new(ErrorMessage::InvalidArguments(format!("Called sqrt on a negative number ({})", x)), None))}

            Value::Number(x.sqrt())
        },
        _ => panic!("Implementation of method 'sqrt' for type 'num' was called on a value of another type")
    };

    scope.set_variable("@return".to_string(), result);
    Ok(())
}