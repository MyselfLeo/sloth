use crate::errors::ErrorMessage;
use crate::{errors::Error, sloth::types::Type};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;
use super::{BuiltInFunction, BuiltinTypes};
use crate::sloth::structure::{ObjectBlueprint, StructSignature, SlothObject};

use sdl2::render::{Canvas, CanvasBuilder};
use sdl2::video::Window;



const WINDOW_STRUCT_NAME: &str = "Window";






pub const BUILTINS: [&str; 2] = [
    WINDOW_STRUCT_NAME,
    "draw"
];


/// Return whether each builtin is a function or a structure
pub fn get_type(builtin: &String) -> Result<BuiltinTypes, String> {
    match builtin.as_str() {
        WINDOW_STRUCT_NAME => Ok(BuiltinTypes::Structure),
        "draw" => Ok(BuiltinTypes::Function),

        _ => Err(format!("Builtin '{builtin}' not found in module 'io'"))
    }
}







/// Return an ObjectBlueprint along with the list of requirements this structure has
pub fn get_struct(s_name: String) -> (Box<dyn ObjectBlueprint>, Vec<String>) {
    match s_name.as_str() {
        WINDOW_STRUCT_NAME => (Box::new(SDL2WrapperBlueprint {}), (vec!["draw".to_string()])),
        s => panic!("Requested unknown built-in structure '{}'", s)
    }
}






/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "draw" => Box::new(
            BuiltInFunction::new(
                "draw",
                Some("graphics"),
                Some(Type::Object("Window".to_string())),
                Type::Number,
                draw
            )
        ),


        n => panic!("Requested unknown built-in function '{}'", n)
    }
}









#[derive(Clone)]
/// Blueprint of the SDL2 wrapper
pub struct SDL2WrapperBlueprint {}

impl ObjectBlueprint for SDL2WrapperBlueprint {
    fn box_clone(&self) -> Box<dyn ObjectBlueprint> {
        Box::new(self.clone())
    }

    fn get_signature(&self) -> StructSignature {
        StructSignature::new(Some("graphics".to_string()), WINDOW_STRUCT_NAME.to_string())
    }

    /// Arguments are:
    /// - window name
    /// - window x size (pixels)
    /// - window y size (pixels)
    fn build(&self, given_values: Vec<Value>) -> Result<Box<dyn crate::sloth::structure::SlothObject>, String> {
        if given_values.len() != 3 {return Err(format!("Structure '{}' expects {} fields, but it has been given {} fields", WINDOW_STRUCT_NAME, 3, given_values.len()))}

        let window_name = match &given_values[0] {
            Value::String(x) => x,
            v => {return Err(format!("Field 'name' of structure '{}' is of type '{}', but it has been given a value of type '{}'", WINDOW_STRUCT_NAME, Type::String, v.get_type()))}
        };

        let x_size = match &given_values[1] {
            Value::Number(x) => {
                if *x < 0.0 {return Err(format!("Cannot give a negative x size to the window ({})", x))}
                *x as u32
            },
            v => {return Err(format!("Field 'x' of structure '{}' is of type '{}', but it has been given a value of type '{}'", WINDOW_STRUCT_NAME, Type::Number, v.get_type()))}
        };

        let y_size = match &given_values[2] {
            Value::Number(y) => {
                if *y < 0.0 {return Err(format!("Cannot give a negative x size to the window ({})", y))}
                *y as u32
            },
            v => {return Err(format!("Field 'y' of structure '{}' is of type '{}', but it has been given a value of type '{}'", WINDOW_STRUCT_NAME, Type::Number, v.get_type()))}
        };

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(&window_name, x_size, y_size)
            .position_centered()
            .build()
            .unwrap();

        let canvas = &mut window.into_canvas().build().unwrap();

        unsafe {
            Ok(Box::new(SDL2Wrapper::new(self.clone(), canvas)))
        }
    }
}



#[derive(Clone)]
pub struct SDL2Wrapper {
    blueprint: SDL2WrapperBlueprint,
    canvas: *mut Canvas<Window>,
}


impl SlothObject for SDL2Wrapper {

    fn box_clone(&self) -> Box<dyn SlothObject> {
        Box::new(self.clone())
    }

    fn get_signature(&self) -> StructSignature {
        StructSignature::new(Some("graphics".to_string()), WINDOW_STRUCT_NAME.to_string())
    }

    fn get_blueprint(&self) -> Box<dyn ObjectBlueprint> {
        Box::new(self.blueprint.clone())
    }

    fn get_field(&self, field_name: &String) -> Result<Value, String> {
        todo!()
    }

    fn set_field(&mut self, field_name: &String, value: Value) -> Result<(), String> {
        todo!()
    }

    fn get_fields(&self) -> (Vec<String>, Vec<Value>) {
        todo!()
    }

    fn execute(&mut self, instruction_name: &str) -> Result<(), String> {
        match instruction_name {
            "draw" => draw_sys(self),
            _ => panic!("Called execute on SDL2Wrapper but the instruction does not exists")
        }
    }
}


impl SDL2Wrapper {
    unsafe fn new(blueprint: SDL2WrapperBlueprint, canvas: *mut Canvas<Window>) -> SDL2Wrapper {
        SDL2Wrapper { blueprint, canvas }
    }
}






/// Update the wrapper on the screen
fn draw_sys(wrapper: &mut SDL2Wrapper) -> Result<(), String> {
    unsafe {
        wrapper.canvas.as_mut().unwrap().present();
        Ok(())
    }
}








/// Draw the given Window to the screen
fn draw(scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
    let value = scope.get_variable("@self".to_string(), program).unwrap();

    let result = match value {
        Value::Object(mut x) => {x.execute("draw_sys")},
        _ => panic!("Implementation of method 'draw' for type 'Window' was called on a value of another type")
    };

    match result {
        Ok(()) => (),
        Err(e) => return Err(Error::new(ErrorMessage::RuntimeError(e), None))
    }

    scope.set_variable("@return".to_string(), Value::Number(0.0));
    Ok(())
}