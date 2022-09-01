use crate::errors::ErrorMessage;
use crate::sloth::structure::ObjectBlueprint;
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::{BuiltInFunction, BuiltinTypes};
use std::cell::RefCell;
use std::rc::Rc;




pub const BUILTINS: [&str; 6] = [
    "to_num",
    "len",
    "insert",
    "push",
    "remove",
    "split"
];




/// Return whether each builtin is a function or a structure
pub fn get_type(builtin: &String) -> Result<BuiltinTypes, String> {
    match builtin.as_str() {
        "to_num" => Ok(BuiltinTypes::Function),
        "len" => Ok(BuiltinTypes::Function),
        "insert" => Ok(BuiltinTypes::Function),
        "push" => Ok(BuiltinTypes::Function),
        "remove" => Ok(BuiltinTypes::Function),
        "split" => Ok(BuiltinTypes::Function),
        _ => Err(format!("Builtin '{builtin}' not found in module 'strings'"))
    }
}







/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "to_num" => Box::new(
            BuiltInFunction::new(
                "to_num",
                Some("strings"),
                Some(Type::String),
                Type::Number,
                to_num
            )
        ),

        "len" => Box::new(
            BuiltInFunction::new(
                "len",
                Some("strings"),
                Some(Type::String),
                Type::Number,
                len
            )
        ),

        "insert" => Box::new(
            BuiltInFunction::new(
                "insert",
                Some("strings"),
                Some(Type::String),
                Type::Number,
                insert
            )
        ),

        "push" => Box::new(
            BuiltInFunction::new(
                "push",
                Some("strings"),
                Some(Type::String),
                Type::Number,
                push
            )
        ),

        "remove" => Box::new(
            BuiltInFunction::new(
                "remove",
                Some("strings"),
                Some(Type::String),
                Type::Number,
                remove
            )
        ),

        "split" => Box::new(
            BuiltInFunction::new(
                "split",
                Some("strings"),
                Some(Type::String),
                Type::List(Box::new(Type::String)),
                split
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
            if (x as i128) < 0 {Err(format!("Cannot use a negative index ({}) to access a string", x as i128))}

            else {
                match limit {
                    Some(l) => {
                        if (x as usize) > l {Err(format!("Tried to set the {}th element of a string of only {} elements", x as usize, l))}
                        else {Ok(x as usize)}
                    },
                    None => Ok(x as usize)
                }
            }
        },
        Some(v) => Err(format!("Tried to index a string with an expression of type '{}'", v.get_type())),
        None => Err(format!("Expected an index"))
    };

    match res {
        Ok(u) => Ok(u),
        Err(e) => Err(Error::new(ErrorMessage::InvalidArguments(e), None))
    }
}















fn to_num(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let value = scope.borrow().get_variable("@self".to_string(), program).unwrap();

    let result = match value.borrow().to_owned() {
        Value::String(x) => {
            match x.parse::<f64>() {
                Ok(v) => Value::Number(v),
                Err(_) => {
                    let err_msg = format!("Cannot parse string \"{}\" into a Number", x);
                    return Err(Error::new(ErrorMessage::RuntimeError(err_msg), None));
                }
            }
        },
        _ => panic!("Implementation of method 'to_num' for type 'string' was called on a value of another type")
    };

    super::set_return(scope, program, result);
    Ok(())
}










fn len(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let value = scope.borrow().get_variable("@self".to_string(), program).unwrap();

    let result = match value.borrow().to_owned() {
        Value::String(x) => Value::Number(x.len() as f64),
        _ => panic!("Implementation of method 'len' for type 'string' was called on a value of another type")
    };

    super::set_return(scope, program, result);
    Ok(())
}









fn insert(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let scope_borrow = scope.borrow();

    let owner_v = scope_borrow.get_variable("@self".to_string(), program).unwrap();
    let inputs = scope_borrow.get_inputs();

    if inputs.len() != 2 {
        let err_msg = format!("Called function 'insert' with {} argument(s), but the function requires 2 arguments", inputs.len());
        return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
    }


    let mut string = match owner_v.borrow().to_owned() {
        Value::String(x) => x,
        _ => panic!("Implementation of method 'insert' for type 'string' was called on a value of another type")
    };


    let idx = expect_positive_index(inputs.get(0).map(|v| v.borrow().to_owned()), Some(string.len() - 1))?;


    let insert_value = match inputs[1].borrow().to_owned() {
        Value::String(x) => x,
        v => {
            let err_msg = format!("Argument 2 of function 'insert' is of type string, given a value of type {}", v.get_type());
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
        }
    };

    string.insert_str(idx, &insert_value);
    
    // try to edit owner value
    let res = match owner_v.try_borrow_mut() {
        Ok(mut borrow) => {
            *borrow = Value::String(string);
            Ok(())
        },
        Err(e) => return Err(Error::new(ErrorMessage::RuntimeError(e.to_string()), None))
    };

    res
}





fn push(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let scope_borrow = scope.borrow();

    let owner_v = scope_borrow.get_variable("@self".to_string(), program).unwrap();
    let inputs = scope_borrow.get_inputs();

    if inputs.len() != 1 {
        let err_msg = format!("Called function 'push' with {} argument(s), but the function requires 1 arguments", inputs.len());
        return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
    }


    let mut string = match owner_v.borrow().to_owned() {
        Value::String(x) => x,
        _ => panic!("Implementation of method 'insert' for type 'string' was called on a value of another type")
    };


    let insert_value = match &inputs[0].borrow().to_owned() {
        Value::String(x) => x,
        v => {
            let err_msg = format!("Argument 1 of function 'push' is of type string, given a value of type {}", v.get_type());
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
        }
    };
    
    string.push_str(&insert_value);
    
    // try to edit owner value
    match owner_v.try_borrow_mut() {
        Ok(borrow) => {
            *borrow = Value::String(string);
            Ok(())
        },
        Err(e) => return Err(Error::new(ErrorMessage::RuntimeError(e.to_string()), None))
    }
}







fn remove(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let scope_borrow = scope.borrow();

    let owner_v = scope_borrow.get_variable("@self".to_string(), program).unwrap();
    let inputs = scope_borrow.get_inputs();

    if inputs.len() != 1 {
        let err_msg = format!("Called function 'remove' with {} argument(s), but the function requires 1 arguments", inputs.len());
        return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
    }


    let mut string = match owner_v.borrow().to_owned() {
        Value::String(x) => x,
        _ => panic!("Implementation of method 'insert' for type 'string' was called on a value of another type")
    };

    let idx = expect_positive_index(inputs.get(0).map(|v| v.borrow().to_owned()), Some(string.len() - 1))?;

    string.remove(idx);
    
    // try to edit owner value
    match owner_v.try_borrow_mut() {
        Ok(borrow) => {
            *borrow = Value::String(string);
            Ok(())
        },
        Err(e) => return Err(Error::new(ErrorMessage::RuntimeError(e.to_string()), None))
    }
}






fn split(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let (string, sep) = {
        let scope_borrow = scope.borrow();

        let owner_v = scope_borrow.get_variable("@self".to_string(), program).unwrap();
        let inputs = scope_borrow.get_inputs();

        if inputs.len() != 1 {
            let err_msg = format!("Called function 'split' with {} argument(s), but the function requires 1 arguments", inputs.len());
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
        }


        let string = match owner_v.borrow().to_owned() {
            Value::String(x) => x,
            _ => panic!("Implementation of method 'insert' for type 'string' was called on a value of another type")
        };

        let sep = match inputs[0].borrow().to_owned() {
            Value::String(x) => x,
            v => {
                let err_msg = format!("Argument 1 of function 'split' is of type string, given a value of type {}", v.get_type());
                return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
            }
        };

        (string, sep)
    };
    

    let vec = string.split(&sep).map(|x| Rc::new(RefCell::new(Value::String(x.to_string())))).collect();

    super::set_return(scope, program, Value::List(Type::String, vec))?;

    Ok(())
}