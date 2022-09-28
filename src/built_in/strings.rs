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




pub const BUILTINS: [&str; 7] = [
    "to_num",
    "len",
    "insert",
    "push",
    "remove",
    "split",
    "get"
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
        "get" => Ok(BuiltinTypes::Function),
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

        "get" => Box::new(
            BuiltInFunction::new(
                "get",
                Some("strings"),
                Some(Type::String),
                Type::String,
                get
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
                        if (x as usize) > l {Err(format!("Tried to set the {}th element of a string of only {} elements", x as usize, l + 1))}
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
    let value = super::get_self(&scope, program)?;

    let result = match value {
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

    super::set_return(&scope, program, result)?;
    Ok(())
}










fn len(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let value = super::get_self(&scope, program)?;

    let result = match value {
        Value::String(x) => Value::Number(x.len() as f64),
        _ => panic!("Implementation of method 'len' for type 'string' was called on a value of another type")
    };

    super::set_return(&scope, program, result)?;
    Ok(())
}









fn insert(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let inputs = super::query_inputs(&scope, vec![Type::Number, Type::String], "insert")?;

    let self_value = super::get_self(&scope, program)?;
    let mut string = match self_value {
        Value::String(x) => x,
        _ => panic!("Implementation of method 'insert' for type 'string' was called on a value of another type")
    };


    let idx = super::expect_natural(&inputs[0], Some((string.len(), "length of string")), 1)?;
    
    let insertion = match &inputs[1] {
        Value::String(v) => v,
        _ => panic!()
    };
    string.insert_str(idx, insertion);
    

    super::set_self(&scope, program, Value::String(string))
}





fn push(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let inputs = super::query_inputs(&scope, vec![Type::String], "push")?;

    let self_value = super::get_self(&scope, program)?;
    let mut string = match self_value {
        Value::String(x) => x,
        _ => panic!("Implementation of method 'push' for type 'string' was called on a value of another type")
    };


    let insert_value = match &inputs[0] {
        Value::String(x) => x,
        _ => panic!()
    };
    

    string.push_str(&insert_value);
    
    // try to edit owner value
    super::set_self(&scope, program, Value::String(string))
}







fn remove(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let owner_v = super::get_self(&scope, program)?;
    let inputs = super::query_inputs(&scope, vec![Type::Number], "remove")?;


    let mut string = match owner_v {
        Value::String(x) => x,
        _ => panic!("Implementation of method 'remove' for type 'string' was called on a value of another type")
    };

    let idx = super::expect_natural(&inputs[0], Some((string.len(), "string length")), 0)?;
    string.remove(idx);
    
    super::set_self(&scope, program, Value::String(string))
}






fn split(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let (string, sep) = {
        let scope_borrow = scope.borrow();

        let owner_v = super::get_self(&scope, program)?;
        let inputs = scope_borrow.get_inputs();

        if inputs.len() != 1 {
            let err_msg = format!("Called function 'split' with {} argument(s), but the function requires 1 arguments", inputs.len());
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
        }


        let string = match owner_v {
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

    super::set_return(&scope, program, Value::List(Type::String, vec))?;

    Ok(())
}













fn get(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let (string, idx) = {
        let scope_borrow = scope.borrow();

        let owner_v = super::get_self(&scope, program)?;
        let inputs = scope_borrow.get_inputs();

        if inputs.len() != 1 {
            let err_msg = format!("Called function 'split' with {} argument(s), but the function requires 1 arguments", inputs.len());
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
        }

        let string = match owner_v {
            Value::String(x) => x,
            _ => panic!("Implementation of method 'insert' for type 'string' was called on a value of another type")
        };

        let idx = expect_positive_index(inputs.get(0).map(|v| v.borrow().to_owned()), Some(string.len() - 1))?;

        (string, idx)
    };
    
    super::set_return(&scope, program, Value::String(string.char_indices().nth(idx).unwrap().1.to_string()))?;

    Ok(())
}