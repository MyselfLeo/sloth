use crate::errors::ErrorMessage;
use crate::sloth::structure::{ObjectBlueprint, StructSignature, SlothObject};
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::{BuiltInFunction, BuiltinTypes};
use std::time;
use std::cell::RefCell;
use std::rc::Rc;





pub const BUILTINS: [&str; 4] = [
    "now",
    "since",

    "Date",
    "Duration"
];


/// Return whether each builtin is a function or a structure
pub fn get_type(builtin: &String) -> Result<BuiltinTypes, String> {
    match builtin.as_str() {
        "now" => Ok(BuiltinTypes::Function),
        "since" => Ok(BuiltinTypes::Function),

        "Date" => Ok(BuiltinTypes::Structure),
        "Duration" => Ok(BuiltinTypes::Structure),

        _ => Err(format!("Builtin '{builtin}' not found in module 'files'"))
    }
}



/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "now" => Box::new(
            BuiltInFunction::new(
                "now",
                Some("time"),
                None,
                Type::String,
                now
            )
        ),

        "since" => Box::new(
            BuiltInFunction::new(
                "since",
                Some("time"),
                Some(Type::Object("Date".to_string())),
                Type::Number,
                since
            )
        ),

        n => panic!("Requested unknown built-in '{}'", n)
    }
}











/// Return an ObjectBlueprint along with the list of requirements this structure has
pub fn get_struct(s_name: String) -> (Box<dyn ObjectBlueprint>, Vec<String>) {
    match s_name.as_str() {
        "Date" => (
            Box::new(DateBlueprint {}),
            Vec::new()
        ),
        "Duration" => (
            Box::new(DurationBlueprint {}),
            vec!["since".to_string()]
        ),
        s => panic!("Requested unknown built-in structure '{}'", s)
    }
}








#[derive(Clone)]
pub struct DurationBlueprint {}

impl ObjectBlueprint for DurationBlueprint {
    fn box_clone(&self) -> Box<dyn ObjectBlueprint> {
        Box::new(self.clone())
    }

    fn get_signature(&self) -> StructSignature {
        StructSignature::new(Some("file".to_string()), "Duration".to_string())
    }

    fn build(&self, given_values: Vec<Rc<RefCell<Value>>>) -> Result<Box<dyn SlothObject>, String> {
        todo!()
    }
}





#[derive(Clone)]
pub struct Duration {
    inner: time::Duration,
}

impl SlothObject for Duration {
    fn box_clone(&self) -> Box<dyn SlothObject> {
        Box::new(self.clone())
    }

    fn get_signature(&self) -> StructSignature {
        StructSignature {module: Some("time".to_string()), name: "Duration".to_string()}
    }

    fn get_blueprint(&self) -> Box<dyn ObjectBlueprint> {
        Box::new(DurationBlueprint {})
    }

    fn get_field(&self, field_name: &String) -> Result<Rc<RefCell<Value>>, String> {
        let value = match field_name.as_str() {
            "s" => Value::Number(self.inner.as_secs() as f64 + self.inner.subsec_micros() as f64 * 1e-9),
            "ms" => Value::Number(self.inner.as_millis() as f64),
            s => return Err(format!("Structure 'Duration' does not have a field named '{}'", s))
        };

        Ok(Rc::new(RefCell::new(value)))
    }

    fn get_fields(&self) -> (Vec<String>, Vec<Rc<RefCell<Value>>>) {
        (Vec::new(), Vec::new())
    }

    fn rereference(&self) -> Box<dyn SlothObject> {
        Box::new(self.clone())
    }
}






#[derive(Clone)]
pub struct DateBlueprint {}

impl ObjectBlueprint for DateBlueprint {
    fn box_clone(&self) -> Box<dyn ObjectBlueprint> {
        Box::new(self.clone())
    }

    fn get_signature(&self) -> StructSignature {
        StructSignature::new(Some("file".to_string()), "Date".to_string())
    }

    fn build(&self, given_values: Vec<Rc<RefCell<Value>>>) -> Result<Box<dyn SlothObject>, String> {
        todo!()
    }
}




#[derive(Clone)]
pub struct Date {
    inner: time::Instant,
}

impl SlothObject for Date {
    fn box_clone(&self) -> Box<dyn SlothObject> {
        Box::new(self.clone())
    }

    fn get_signature(&self) -> StructSignature {
        StructSignature::new(Some("file".to_string()), "Date".to_string())
    }

    fn get_blueprint(&self) -> Box<dyn ObjectBlueprint> {
        Box::new(DateBlueprint {})
    }

    fn get_field(&self, field_name: &String) -> Result<Rc<RefCell<Value>>, String> {
        let value = match field_name.as_str() {
            "epoch" => Value::Number(self.inner.),
            s => return Err(format!("Structure 'Duration' does not have a field named '{}'", s))
        };

        Ok(Rc::new(RefCell::new(value)))
    }

    fn get_fields(&self) -> (Vec<String>, Vec<Rc<RefCell<Value>>>) {
        (Vec::new(), Vec::new())
    }

    fn rereference(&self) -> Box<dyn SlothObject> {
        Box::new(self.clone())
    }
}








/*/// Return the content of a file as a string
fn load(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let inputs = scope.borrow().get_inputs();

    if inputs.len() != 1 {
        let err_msg = format!("Called function 'load' with {} argument(s), but the function requires 1 argument", inputs.len());
        return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
    }

    let path = match inputs[0].borrow().to_owned() {
        Value::String(x) => x,
        v => {
            let err_msg = format!("Argument 1 of function 'load' is of type string, given a value of type {}", v.get_type());
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
        }
    };

    let content = match fs::read_to_string(&path) {
        Ok(f) => f,
        Err(e) => {
            let err_msg = format!("Could not open file '{}': {}", path, e.to_string());
            return Err(Error::new(ErrorMessage::RuntimeError(err_msg), None))
        },
    };

    super::set_return(scope, program, Value::String(content))
}*/