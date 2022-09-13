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
use std::thread;





pub const BUILTINS: [&str; 5] = [
    "now",
    "since",
    "sleep",

    "Instant",
    "Duration"
];


/// Return whether each builtin is a function or a structure
pub fn get_type(builtin: &String) -> Result<BuiltinTypes, String> {
    match builtin.as_str() {
        "now" => Ok(BuiltinTypes::Function),
        "since" => Ok(BuiltinTypes::Function),
        "sleep" => Ok(BuiltinTypes::Function),

        "Instant" => Ok(BuiltinTypes::Structure),
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
                Some("clock"),
                None,
                Type::Object("Instant".to_string()),
                now
            )
        ),

        "since" => Box::new(
            BuiltInFunction::new(
                "since",
                Some("clock"),
                Some(Type::Object("Instant".to_string())),
                Type::Object("Duration".to_string()),
                since
            )
        ),

        "sleep" => Box::new(
            BuiltInFunction::new(
                "sleep",
                Some("clock"),
                None,
                Type::Number,
                sleep
            )
        ),

        n => panic!("Requested unknown built-in '{}'", n)
    }
}











/// Return an ObjectBlueprint along with the list of requirements this structure has
pub fn get_struct(s_name: String) -> (Box<dyn ObjectBlueprint>, Vec<String>) {
    match s_name.as_str() {
        "Instant" => (
            Box::new(InstantBlueprint {}),
            Vec::new()
        ),
        "Duration" => (
            Box::new(DurationBlueprint {}),
            vec!["since".to_string()]
        ),
        s => panic!("Requested unknown built-in structure '{}'", s)
    }
}







fn duration_from_sec_f64(x: f64) -> time::Duration {
    let secs = x.floor() as u64;
    let nanos = (x.fract() * 1e10) as u32;

    time::Duration::new(secs, nanos)
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

    /// Duration as seconds (in f64)
    fn build(&self, given_values: Vec<Rc<RefCell<Value>>>) -> Result<Box<dyn SlothObject>, String> {
        if given_values.len() > 1 {return Err(format!("Structure 'Duration' requires 0 or 1 input, got {}", given_values.len()))}

        let duration = {
            if given_values.len() == 0 {time::Duration::new(0, 0)}
            else {
                let x = match given_values[0].borrow().to_owned() {
                    Value::Number(x) => x,
                    v => {return Err(format!("Structure 'Duration' expected a Number, got {}", v.get_type()))}
                };

                duration_from_sec_f64(x)
            }
        };

        Ok(Box::new(Duration {inner: duration}))
    }
}





#[derive(Clone)]
pub struct Duration {
    inner: time::Duration,
}

impl SlothObject for Duration {
    fn get_signature(&self) -> StructSignature {
        StructSignature {module: Some("clock".to_string()), name: "Duration".to_string()}
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

    fn rereference(&self) -> Result<Box<dyn SlothObject>, String> {
        Ok(Box::new(self.clone()))
    }
}



impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Duration({}s)", self.inner.as_secs() as f64 + self.inner.subsec_micros() as f64 * 1e-9)
    }
}






#[derive(Clone)]
pub struct InstantBlueprint {}

impl ObjectBlueprint for InstantBlueprint {
    fn box_clone(&self) -> Box<dyn ObjectBlueprint> {
        Box::new(self.clone())
    }

    fn get_signature(&self) -> StructSignature {
        StructSignature::new(Some("file".to_string()), "Instant".to_string())
    }

    fn build(&self, _: Vec<Rc<RefCell<Value>>>) -> Result<Box<dyn SlothObject>, String> {
        // TODO: maybe allow to build ?
        Err("The structure 'Instant' cannot be built".to_string())
    }
}




#[derive(Clone)]
pub struct Date {
    inner: time::Instant,
}

impl SlothObject for Date {
    fn get_signature(&self) -> StructSignature {
        StructSignature::new(Some("file".to_string()), "Instant".to_string())
    }

    fn get_blueprint(&self) -> Box<dyn ObjectBlueprint> {
        Box::new(InstantBlueprint {})
    }

    fn get_field(&self, field_name: &String) -> Result<Rc<RefCell<Value>>, String> {
        #[allow(unused_variables)]
        let value = match field_name.as_str() {
            s => return Err(format!("Structure 'Instant' does not have a field named '{}'", s))
        };

        #[allow(unreachable_code)]
        Ok(Rc::new(RefCell::new(value)))
    }

    fn get_fields(&self) -> (Vec<String>, Vec<Rc<RefCell<Value>>>) {
        (Vec::new(), Vec::new())
    }

    fn rereference(&self) -> Result<Box<dyn SlothObject>, String> {
        Ok(Box::new(self.clone()))
    }
}



impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instant()")
    }
}






/// Return an Instant representing now
fn now(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    super::query_inputs(&scope, vec![], "now")?;

    let instant = Box::new(Date {inner: time::Instant::now()});
    super::set_return(&scope, program, Value::Object(instant))
}



/// Return a Duration between the given Date and now
fn since(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    super::query_inputs(&scope, vec![], "now")?;

    // This should be an Instant object
    let value_self = super::get_self(&scope, program)?;

    let duration = match value_self {
        Value::Object(reference) => {
            let mut any = reference.to_owned();
            let object = match any.as_any().downcast_ref::<Date>() {
                Some(v) => v,
                None => return Err(Error::new(ErrorMessage::RustError("Called function 'since' on an object which is not a Instant".to_string()), None))
            };

            object.inner.elapsed()
        },
        _ => {
            return Err(Error::new(ErrorMessage::RustError("Called function 'since' on an object which is not a Instant".to_string()), None))
        }
    };

    let res = Box::new(Duration {inner: duration});
    super::set_return(&scope, program, Value::Object(res))
}





/// Pauses the execution for the given duration in seconds
fn sleep(scope: Rc<RefCell<Scope>>, _: &mut SlothProgram) -> Result<(), Error> {
    let inputs = super::query_inputs(&scope, vec![Type::Number], "sleep")?;

    match inputs[0] {
        Value::Number(x) => {
            if x < 0.0 {
                let err_msg = format!("Cannot wait for a negative duration ({})", x);
                Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None))
            }
            else {
                // The sleep occurs here
                thread::sleep(duration_from_sec_f64(x));
                Ok(())
            }
        },
        _ => panic!("query_inputs failed")
    }
}