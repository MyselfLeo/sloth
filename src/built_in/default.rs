/// default builtins, included in every program.
/// It contains the functions needed by the lists



use crate::errors::{Error, ErrorMessage};
use crate::sloth::function::Callable;
use crate::sloth::function::{SlothFunction, FunctionSignature};
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::types::Type;
use crate::sloth::value::Value;

use sloth_derive::SlothFunction;





pub const BUILTINS: [&str; 3] = [
    "set",
    "get",
    "push"
];


/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "set" => Box::new(BuiltinDefaultListSet {}),
        "get" => Box::new(BuiltinDefaultListGet {}),
        "push" => Box::new(BuiltinDefaultListPush {}),
        n => panic!("Requested unknown built-in '{}'", n)
    }
}





#[derive(SlothFunction)]
#[name = "set"] #[module = "default"] #[output = "num"] #[owner = "list"]
pub struct BuiltinDefaultListSet {}
impl Callable for BuiltinDefaultListSet {
    unsafe fn call(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
        let list = scope.get_variable("@self".to_string(), program).unwrap();
        let inputs = scope.get_inputs();


        // get the list type
        let (mut list_type, mut list_vec) = match list {
            Value::List(t, v) => (t, v),
            _ => panic!("Called 'set' on a value which is not a list")
        };

        
        // first value is the index, second value is the value to place in the list
        let index = match inputs.get(0) {
            Some(Value::Number(x)) => {
                if (*x as i128) < 0 {
                    let err_msg = format!("Cannot use a negative index ({}) to access a list", *x as i128);
                    return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
                }
                else {*x as usize}
            },

            Some(v) => {
                let err_msg = format!("Tried to index a list with an expression of type '{}'", v.get_type());
                return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
            },

            None => {
                let err_msg = "The 'set' method requires two inputs: 'num', the index of the element to set, and its value (the same type as the list)".to_string();
                return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
            },
        };

        // the new value must be the same type as list_type
        let new_value = match inputs.get(1) {
            Some(v) => {
                if v.get_type() == list_type {v.clone()}
                else {
                    let err_msg = format!("Tried to set an element of type '{}' in a list of type '{}'", v.get_type(), list_type);
                    return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
                }
            },

            None => {
                let err_msg = "The 'set' method requires two inputs: 'num', the index of the element to set, and its value (the same type as the list)".to_string();
                return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
            },
        };


        // modify the value and set the self variable
        if list_vec.len() == 0 {
            list_type = new_value.get_type();   // the list was empty before so it was of type Any. Now that there is an element, we change its type*
            list_vec.push(new_value);
        }
        else if index > list_vec.len() - 1 {list_vec.push(new_value)}
        else {list_vec[index] = new_value}
        scope.set_variable("@self".to_string(), Value::List(list_type, list_vec));

        scope.set_variable("@return".to_string(), Value::Number(0.0));
        Ok(())
    }
}









#[derive(SlothFunction)]
#[name = "push"] #[module = "default"] #[output = "num"] #[owner = "list"]
pub struct BuiltinDefaultListPush {}
impl Callable for BuiltinDefaultListPush {
    unsafe fn call(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
        let list = scope.get_variable("@self".to_string(), program).unwrap();
        let inputs = scope.get_inputs();

        if inputs.len() == 0 {
            let err_msg = "The 'push' method requires one input, the value to push to the list".to_string();
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
        }
        else if inputs.len() > 1 {
            let err_msg = "The 'push' method requires only one input, the value to push to the list".to_string();
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
        }

        // get the list type
        let (mut list_type, mut list_vec) = match list {
            Value::List(t, v) => (t, v),
            _ => panic!("Called 'push' on a value which is not a list")
        };


        // the new value must be the same type as list_type
        let new_value = inputs[0].clone();

        // modify the value and set the self variable
        if list_vec.len() == 0 {
            // the list was empty before so it was of type Any. Now that there is an element, we change its type
            list_type = new_value.get_type();
        }
        list_vec.push(new_value);
        scope.set_variable("@self".to_string(), Value::List(list_type, list_vec));

        scope.set_variable("@return".to_string(), Value::Number(0.0));
        Ok(())
    }
}












#[derive(SlothFunction)]
#[name = "get"] #[module = "default"] #[output = "any"] #[owner = "list"]
pub struct BuiltinDefaultListGet {}
impl Callable for BuiltinDefaultListGet {
    unsafe fn call(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
        let list = scope.get_variable("@self".to_string(), program).unwrap();
        let inputs = scope.get_inputs();


        // get the list value
        let (_, list_vec) = match list {
            Value::List(t, v) => (t, v),
            _ => panic!("Called 'set' on a value which is not a list")
        };

        
        // first value is the index, second value is the value to place in the list
        let index = match inputs.get(0) {
            Some(Value::Number(x)) => {
                if (*x as i128) < 0 {
                    let err_msg = format!("Cannot use a negative index ({}) to access a list", *x as i128);
                    return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
                }
                else {*x as usize}
            },

            Some(v) => {
                let err_msg = format!("Tried to index a list with an expression of type '{}'", v.get_type());
                return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
            },

            None => {
                let err_msg = "The 'get' method requires the index of the element to get".to_string();
                return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
            },
        };


        // modify the value and set the self variable
        if index > list_vec.len() - 1 {
            let err_msg = format!("Tried to access the {}th element of a list with only {} elements", index, list_vec.len());
            return Err(Error::new(ErrorMessage::RuntimeError(err_msg), None));
        }
        scope.set_variable("@return".to_string(), list_vec[index].clone());
        Ok(())
    }
}