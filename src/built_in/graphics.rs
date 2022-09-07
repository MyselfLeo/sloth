use crate::errors::ErrorMessage;
use crate::sloth::structure::{ObjectBlueprint, SlothObject, StructSignature};
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::{BuiltInFunction, BuiltinTypes};
use std::cell::RefCell;
use std::rc::Rc;

use sdl2::render::Canvas as SDL2Canvas;
use sdl2::video::Window as SDL2Window;
use sdl2::Sdl;


static mut SDL_CONTEXT: Option<Sdl> = None;



pub const BUILTINS: [&str; 1] = [
    "Canvas"
];


/// Return whether each builtin is a function or a structure
pub fn get_type(builtin: &String) -> Result<BuiltinTypes, String> {
    match builtin.as_str() {
        

        "Canvas" => Ok(BuiltinTypes::Structure),

        _ => Err(format!("Builtin '{builtin}' not found in module 'graphics'"))
    }
}



/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        /*
        "save" => Box::new(
            BuiltInFunction::new(
                "save",
                Some("files"),
                None,
                Type::Number,
                save
            )
        ),
        */

        n => panic!("Requested unknown built-in '{}'", n)
    }
}











/// Return an ObjectBlueprint along with the list of requirements this structure has
pub fn get_struct(s_name: String) -> (Box<dyn ObjectBlueprint>, Vec<String>) {
    match s_name.as_str() {
        "Canvas" => (Box::new(CanvasBlueprint {}), Vec::new()),
        s => panic!("Requested unknown built-in structure '{}'", s)
    }
}








pub fn expect_positive_value(value: Value) -> Result<u32, String> {
    match value {
        Value::Number(x) => {
            if x < 0.0 {Err(format!("Cannot use a negative index ({}) to access a string", x as i64))}

            else {Ok(x as u32)}
        },
        v => Err(format!("Tried to index a string with an expression of type '{}'", v.get_type())),
    }
}










#[derive(Clone)]
pub struct CanvasBlueprint {}

impl ObjectBlueprint for CanvasBlueprint {
    fn box_clone(&self) -> Box<dyn ObjectBlueprint> {
        Box::new(self.clone())
    }

    fn get_signature(&self) -> StructSignature {
        StructSignature::new(Some("graphics".to_string()), "Canvas".to_string())
    }

    fn build(&self, given_values: Vec<Rc<RefCell<Value>>>) -> Result<Box<dyn crate::sloth::structure::SlothObject>, String> {
        // 3 inputs: window name, window x and window y

        if given_values.len() != 3 {return Err(format!("Structure 'Canvas' requires 3 inputs, got {}", given_values.len()))}

        let window_name = match given_values[0].borrow().to_owned() {
            Value::String(s) => s,
            v => return Err(format!("Argument 1 of 'Canvas' is of type String, given a value of type '{}'", v.get_type()))
        };
        let window_x = expect_positive_value(given_values[1].borrow().to_owned())?;
        let window_y = expect_positive_value(given_values[2].borrow().to_owned())?;

        // create SDL Canvas
        let canvas = unsafe {
            if SDL_CONTEXT.is_none() {SDL_CONTEXT = Some(sdl2::init()?)}
            let video_subsystem = SDL_CONTEXT.clone().unwrap().video()?;

            let window = video_subsystem.window(&window_name, window_x, window_y)
                                        .position_centered()
                                        .build()
                                        .unwrap();

            window.into_canvas().build().unwrap()
        };

        // return object
        Ok(Box::new(Canvas {inner: Rc::new(RefCell::new(canvas))}))
    }
}




#[derive(Clone)]
pub struct Canvas {
    inner: Rc<RefCell<SDL2Canvas<SDL2Window>>>,
}


impl std::fmt::Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl SlothObject for Canvas {
    fn box_clone(&self) -> Box<dyn SlothObject> {
        Box::new(self.clone())
    }

    fn get_signature(&self) -> crate::sloth::structure::StructSignature {
        StructSignature::new(Some("graphics".to_string()), "Canvas".to_string())
    }

    fn get_blueprint(&self) -> Box<dyn ObjectBlueprint> {
        Box::new(CanvasBlueprint {})
    }

    fn get_field(&self, field_name: &String) -> Result<Rc<RefCell<Value>>, String> {
        Err(format!("No fields in Canvas"))
    }

    fn get_fields(&self) -> (Vec<String>, Vec<Rc<RefCell<Value>>>) {
        (Vec::new(), Vec::new())
    }

    fn rereference(&self) -> Box<dyn SlothObject> {
        panic!("Cannot be rereferenced")
    }
}