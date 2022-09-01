use crate::errors::ErrorMessage;
use crate::sloth::structure::ObjectBlueprint;
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::{BuiltInFunction, BuiltinTypes};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;



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
pub fn get_struct(s_name: String) -> (Box<dyn ObjectBlueprint>, Vec<String>) {
    match s_name.as_str() {
        s => panic!("Requested unknown built-in structure '{}'", s)
    }
}








/// Check if the given value is Some and is a positive number (>= 0). Returns it as usize or an error string if it's not the case. Optional limit
pub fn expect_positive_index(value: Option<Value>, limit: Option<usize>) -> Result<usize, Error> {
    let res = match value {
        Some(Value::Number(x)) => {
            if (x as i128) < 0 {Err(format!("Cannot use a negative index ({}) to access a list", x as i128))}

            else {
                match limit {
                    Some(l) => {
                        if (x as usize) > l {Err(format!("Tried to set the {}th element of a list of only {} elements", x as usize, l))}
                        else {Ok(x as usize)}
                    },
                    None => Ok(x as usize)
                }
            }
        },
        Some(v) => Err(format!("Tried to index a list with an expression of type '{}'", v.get_type())),
        None => Err(format!("Expected an index"))
    };

    match res {
        Ok(u) => Ok(u),
        Err(e) => Err(Error::new(ErrorMessage::InvalidArguments(e), None))
    }
}











fn set(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let scope_borrow = scope.borrow();

    let list = scope_borrow.get_variable("@self".to_string(), program).unwrap();
    let inputs = scope_borrow.get_inputs();

    // get the list type
    let (list_type, list_vec) = match list.borrow().to_owned() {
        Value::List(t, v) => (t, v),
        _ => panic!("Called 'set' on a value which is not a list")
    };

    // first value is the index, second value is the value to place in the list
    let index = expect_positive_index(inputs.get(0).map(|v| v.borrow().to_owned()), Some(list_vec.len() - 1))?;

    // the new value must be the same type as list_type
    let new_value = match inputs.get(1) {
        Some(v) => {
            if v.borrow().get_type() == list_type {v}
            else {
                let err_msg = format!("Tried to set an element of type '{}' in a list of type '{}'", v.borrow().get_type(), list_type);
                return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
            }
        },

        None => {
            let err_msg = "The 'set' method requires two inputs: 'num', the index of the element to set, and its value (the same type as the list)".to_string();
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
        },
    };


    // Try to set the value
    let element_ref = list_vec[index].clone();
    match element_ref.try_borrow_mut() {
        Ok(mut borrow) => *borrow = new_value.borrow().to_owned(),
        Err(e) => return Err(Error::new(ErrorMessage::RuntimeError(e.to_string()), None))
    }

    Ok(())
}

















fn get(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let scope_borrow = scope.borrow();

    let list = scope_borrow.get_variable("@self".to_string(), program).unwrap();
    let inputs = scope_borrow.get_inputs();


    // get the list value
    let (_, list_vec) = match list.borrow().to_owned() {
        Value::List(t, v) => (t, v),
        _ => panic!("Called 'set' on a value which is not a list")
    };

    // first value is the index, second value is the value to place in the list
    let index = expect_positive_index(inputs.get(0).map(|v| v.borrow().to_owned()), Some(list_vec.len() - 1))?;


    // modify the value and set the self variable
    if index > list_vec.len() - 1 {
        let err_msg = format!("Tried to access the {}th element of a list with only {} elements", index, list_vec.len());
        return Err(Error::new(ErrorMessage::RuntimeError(err_msg), None));
    }

    // Return the value
    let element_value = list_vec[index].borrow().to_owned();
    super::set_return(scope.clone(), program, element_value);
    Ok(())
}

















fn push(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let scope_borrow = scope.borrow();

    let mut list = scope_borrow.get_variable("@self".to_string(), program).unwrap();
    let inputs = scope_borrow.get_inputs();

    if inputs.len() == 0 {
        let err_msg = "The 'push' method requires one input, the value to push to the list".to_string();
        return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
    }
    else if inputs.len() > 1 {
        let err_msg = "The 'push' method requires only one input, the value to push to the list".to_string();
        return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
    }

    // get the list type
    let (list_type, list_vec) = match list.borrow().to_owned() {
        Value::List(t, v) => (t, v),
        _ => panic!("Called 'push' on a value which is not a list")
    };

    // the new value must be the same type as list_type
    let new_value = inputs[0].clone();
    if new_value.borrow().get_type() != list_type {
        let err_msg = format!("Tried to push a value of type '{}' to a list of type '{}'", new_value.borrow().get_type(), list_type);
        return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
    }


    if let Value::List(t, l) = Rc::get_mut(&mut list).unwrap().get_mut() {
        // Set the list's type if it was 'Any' before
        if l.len() == 0 {*t = new_value.borrow().get_type();}
        // Push the new value
        (*l).push(new_value);
    }
    else {panic!()}

    Ok(())
}



















fn pull(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let scope_borrow = scope.borrow();

    let mut list = scope_borrow.get_variable("@self".to_string(), program).unwrap();
    let inputs = scope_borrow.get_inputs();


    // get the list value
    let (_, list_vec) = match list.borrow().to_owned() {
        Value::List(t, v) => (t, v),
        _ => panic!("Called 'pull' on a value which is not a list")
    };

    // first value is the index of the value to pull
    let index = expect_positive_index(inputs.get(0).map(|v| v.borrow().to_owned()), Some(list_vec.len() - 1))?;


    if let Value::List(t, l) = Rc::get_mut(&mut list).unwrap().get_mut() {
       let pulled_value = (*l).remove(index);
       super::set_return(scope.clone(), program, pulled_value.borrow().to_owned());
    }
    else {panic!()}

    Ok(())
}














fn len(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let list = scope.borrow().get_variable("@self".to_string(), program).unwrap();
    // get the list value
    let (_, list_vec) = match list.borrow().to_owned() {
        Value::List(t, v) => (t, v),
        _ => panic!("Called 'set' on a value which is not a list")
    };
    super::set_return(scope, program, Value::Number(list_vec.len() as f64));
    Ok(())
}