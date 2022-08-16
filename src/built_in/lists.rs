use crate::errors::ErrorMessage;
use crate::sloth::structure::StructDefinition;
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::{BuiltInFunction, BuiltinTypes};



pub const BUILTINS: [&str; 5] = [
    "set",
    "get",
    "push",
    "pull",
    "len"
];


/// Return whether each builtin is a function or a structure
pub fn get_type(builtin: &String) -> Result<BuiltinTypes, String> {
    match builtin.as_str() {
        "set" => Ok(BuiltinTypes::Function),
        "get" => Ok(BuiltinTypes::Function),
        "push" => Ok(BuiltinTypes::Function),
        "pull" => Ok(BuiltinTypes::Function),
        "len" => Ok(BuiltinTypes::Function),

        _ => Err(format!("Builtin '{builtin}' not found in module 'lists'"))
    }
}



/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "set" => Box::new(
            BuiltInFunction::new(
                "set",
                Some("lists"),
                Some(Type::List(Box::new(Type::Any))),
                Type::Number,
                set
            )
        ),

        "get" => Box::new(
            BuiltInFunction::new(
                "get",
                Some("lists"),
                Some(Type::List(Box::new(Type::Any))),
                Type::Any,
                get
            )
        ),

        "push" => Box::new(
            BuiltInFunction::new(
                "push",
                Some("lists"),
                Some(Type::List(Box::new(Type::Any))),
                Type::Number,
                push
            )
        ),

        "pull" => Box::new(
            BuiltInFunction::new(
                "pull",
                Some("lists"),
                Some(Type::List(Box::new(Type::Any))),
                Type::Any,
                pull
            )
        ),

        "len" => Box::new(
            BuiltInFunction::new(
                "len",
                Some("lists"),
                Some(Type::List(Box::new(Type::Any))),
                Type::Number,
                len
            )
        ),




        n => panic!("Requested unknown built-in '{}'", n)
    }
}









/// Return a StructDefinition along with the list of requirements this structure has
pub fn get_struct(s_name: String) -> (StructDefinition, Vec<String>) {
    match s_name.as_str() {
        s => panic!("Requested unknown built-in structure '{}'", s)
    }
}














fn set(scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
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


















fn get(scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
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

















fn push(scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
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



















fn pull(scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
    let list = scope.get_variable("@self".to_string(), program).unwrap();
    let inputs = scope.get_inputs();


    // get the list value
    let (t, mut list_vec) = match list {
        Value::List(t, v) => (t, v),
        _ => panic!("Called 'pull' on a value which is not a list")
    };

    // first value is the index of the value to pull
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

    
    if index > list_vec.len() -1 {
        let err_msg = format!("Tried to access the {}th element of a list with only {} elements", index, list_vec.len());
        return Err(Error::new(ErrorMessage::RuntimeError(err_msg), None));
    }

    // Remove the requested value from the vec
    let pulled_value = list_vec.remove(index).clone();

    // modify the self variable and the return variable
    scope.set_variable("@self".to_string(), Value::List(t, list_vec));
    scope.set_variable("@return".to_string(), pulled_value);
    Ok(())
}














fn len(scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
    let list = scope.get_variable("@self".to_string(), program).unwrap();
    // get the list value
    let (_, list_vec) = match list {
        Value::List(t, v) => (t, v),
        _ => panic!("Called 'set' on a value which is not a list")
    };
    scope.set_variable("@return".to_string(), Value::Number(list_vec.len() as f64));
    Ok(())
}