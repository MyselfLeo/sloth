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

use sdl2::rect::Rect;
use sdl2::render::Canvas as SDL2Canvas;
use sdl2::video::Window as SDL2Window;
use sdl2::{Sdl, event::Event};
use sdl2::event::EventType;


static mut SDL_CONTEXT: Option<Sdl> = None;



pub const BUILTINS: [&str; 5] = [
    "Canvas",

    "check_event",
    "update",
    "set_pixel",
    "set_rect"
];


/// Return whether each builtin is a function or a structure
pub fn get_type(builtin: &String) -> Result<BuiltinTypes, String> {
    match builtin.as_str() {
        "Canvas" => Ok(BuiltinTypes::Structure),

        "check_event" => Ok(BuiltinTypes::Function),
        "update" => Ok(BuiltinTypes::Function),
        "set_pixel" => Ok(BuiltinTypes::Function),
        "set_rect" => Ok(BuiltinTypes::Function),

        _ => Err(format!("Builtin '{builtin}' not found in module 'media'"))
    }
}



/// Return a reference to a new SlothFunction. Panics if the function does not exists
pub fn get_function(f_name: String) -> Box<dyn SlothFunction> {
    match f_name.as_str() {
        "check_event" => Box::new(
            BuiltInFunction::new(
                "check_event",
                Some("media"),
                None,
                Type::Boolean,
                check_event
            )
        ),

        "update" => Box::new(
            BuiltInFunction::new(
                "update",
                Some("media"),
                Some(Type::Object("Canvas".to_string())),
                Type::Number,
                update
            )
        ),

        "set_pixel" => Box::new(
            BuiltInFunction::new(
                "set_pixel",
                Some("media"),
                Some(Type::Object("Canvas".to_string())),
                Type::Number,
                set_pixel
            )
        ),

        "set_rect" => Box::new(
            BuiltInFunction::new(
                "set_rect",
                Some("media"),
                Some(Type::Object("Canvas".to_string())),
                Type::Number,
                set_rect
            )
        ),

        n => panic!("Requested unknown built-in '{}'", n)
    }
}











/// Return an ObjectBlueprint along with the list of requirements this structure has
pub fn get_struct(s_name: String) -> (Box<dyn ObjectBlueprint>, Vec<String>) {
    match s_name.as_str() {
        "Canvas" => (Box::new(CanvasBlueprint {}), vec!["update".to_string(), "set_pixel".to_string()]),
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
        StructSignature::new(Some("media".to_string()), "Canvas".to_string())
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
        write!(f, "Canvas Object")
    }
}


impl SlothObject for Canvas {
    fn get_signature(&self) -> crate::sloth::structure::StructSignature {
        StructSignature::new(Some("media".to_string()), "Canvas".to_string())
    }

    fn get_blueprint(&self) -> Box<dyn ObjectBlueprint> {
        Box::new(CanvasBlueprint {})
    }

    fn get_field(&self, _: &String) -> Result<Rc<RefCell<Value>>, String> {
        Err(format!("No fields in Canvas"))
    }

    fn get_fields(&self) -> (Vec<String>, Vec<Rc<RefCell<Value>>>) {
        (Vec::new(), Vec::new())
    }

    fn shallow_clone(&self) -> Box<dyn SlothObject> {
        Box::new(Canvas {inner: self.inner.clone()})
    }

    fn deep_clone(&self) -> Result<Box<dyn SlothObject>, String> {
        Err("Canvas cannot be copied: you must use it as a reference (define myfunc: ~Canvas -> ...)".to_string())
    }
}







fn update(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let value_self = super::get_self(&scope, program)?;

    let mut obj = match value_self {
        Value::Object(obj) => {obj},
        _ => panic!()
    };

    let canvas = match obj.as_any().downcast_ref::<Canvas>() {
        Some(v) => v,
        None => return Err(Error::new(ErrorMessage::RustError("here".to_string()), None))
    };

    let res = match canvas.inner.try_borrow_mut() {
        Ok(mut reference) => {
            reference.present();
            Ok(())
        },
        Err(e) => Err(Error::new(ErrorMessage::RustError(e.to_string()), None)),
    };
    
    res
}




fn set_pixel(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let value_self = super::get_self(&scope, program)?;
    let inputs = super::query_inputs(&scope, vec![Type::Number, Type::Number, Type::Number, Type::Number, Type::Number], "set_pixel")?;

    let mut obj = match value_self {
        Value::Object(obj) => {obj},
        _ => panic!()
    };
    let canvas = match obj.as_any().downcast_ref::<Canvas>() {
        Some(v) => v,
        None => return Err(Error::new(ErrorMessage::RustError("here".to_string()), None))
    };


    let window_size = canvas.inner.borrow().window().size();


    let x = super::expect_natural(&inputs[0], Some((window_size.0 as usize, "Window width")), 0)? as i32;
    let y = super::expect_natural(&inputs[1], Some((window_size.1 as usize, "Window height")), 1)? as i32;

    let r = super::expect_natural(&inputs[2], None, 2)? as u8;
    let g = super::expect_natural(&inputs[3], None, 3)? as u8;
    let b = super::expect_natural(&inputs[4], None, 4)? as u8;


    let res = match canvas.inner.try_borrow_mut() {
        Ok(mut reference) => {
            reference.set_draw_color((r, g, b));
            match reference.draw_point((x, y)) {
                Ok(()) => Ok(()),
                Err(e) => Err(Error::new(ErrorMessage::RustError(e.to_string()), None))
            }
        },
        Err(e) => Err(Error::new(ErrorMessage::RustError(e.to_string()), None)),
    };
    
    res
}











fn set_rect(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let value_self = super::get_self(&scope, program)?;
    let inputs = super::query_inputs(&scope, vec![Type::Number, Type::Number, Type::Number, Type::Number, Type::Number, Type::Number, Type::Number], "set_pixel")?;

    let mut obj = match value_self {
        Value::Object(obj) => {obj},
        _ => panic!()
    };
    let canvas = match obj.as_any().downcast_ref::<Canvas>() {
        Some(v) => v,
        None => return Err(Error::new(ErrorMessage::RustError("here".to_string()), None))
    };


    let window_size = canvas.inner.borrow().window().size();


    let x = super::expect_natural(&inputs[0], Some((window_size.0 as usize, "Window width")), 0)? as i32;
    let y = super::expect_natural(&inputs[1], Some((window_size.1 as usize, "Window height")), 1)? as i32;
    
    let width = super::expect_natural(&inputs[2], None, 2)? as u32;
    let height = super::expect_natural(&inputs[3], None, 3)? as u32;

    let r = super::expect_natural(&inputs[4], None, 4)? as u8;
    let g = super::expect_natural(&inputs[5], None,5)? as u8;
    let b = super::expect_natural(&inputs[6], None, 6)? as u8;


    let res = match canvas.inner.try_borrow_mut() {
        Ok(mut reference) => {
            reference.set_draw_color((r, g, b));
            match reference.draw_rect(Rect::new(x, y, width, height)) {
                Ok(()) => Ok(()),
                Err(e) => Err(Error::new(ErrorMessage::RustError(e.to_string()), None))
            }
        },
        Err(e) => Err(Error::new(ErrorMessage::RustError(e.to_string()), None)),
    };
    
    res
}












fn check_event(scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
    let inputs = super::query_inputs(&scope, vec![Type::String], "check_event")?;

    let event_id = match &inputs[0] {
        Value::String(x) => x.as_str(),
        _ => panic!()
    };

    let requested_event = match event_id {
        "EVENT_QUIT" => EventType::Quit,
        _ => {
            let err_msg = format!("Event '{}' does not exists", event_id);
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None))
        }
    };


    let called = unsafe {
        if SDL_CONTEXT.is_none() {
            SDL_CONTEXT = match sdl2::init() {
                Ok(v) => Some(v),
                Err(e) => return Err(Error::new(ErrorMessage::RustError(e.to_string()), None))
            }
        };

        let ep = SDL_CONTEXT.as_ref().unwrap().event_pump();
        let mut event_pump = match ep {
            Ok(v) => v,
            Err(e) => return Err(Error::new(ErrorMessage::RustError(e.to_string()), None))
        };


        // find requested event
        let mut res = false;
        for event in event_pump.poll_iter() {
            // skip unknown events
            if event.is_unknown() {continue}

            let event_type = match get_event_type(&event) {
                Ok(e) => e,
                Err(e) => return Err(Error::new(ErrorMessage::RustError(e), None))
            };

            if event_type == requested_event {
                res = true;
                break
            }
        }

        res
    };


    super::set_return(&scope, program, Value::Boolean(called))
}






















/// Return the type of the given event. Return error if the given event is Unknow
fn get_event_type(event: &Event) -> Result<EventType, String> {
    match event {
        Event::Quit {..} => Ok(EventType::Quit),
        Event::AppTerminating {..} => Ok(EventType::AppTerminating),
        Event::AppLowMemory {..} => Ok(EventType::AppLowMemory),
        Event::AppWillEnterBackground {..} => Ok(EventType::AppWillEnterBackground),
        Event::AppDidEnterBackground {..} => Ok(EventType::AppDidEnterBackground),
        Event::AppWillEnterForeground {..} => Ok(EventType::AppWillEnterForeground),
        Event::AppDidEnterForeground {..} => Ok(EventType::AppDidEnterForeground),
        Event::Display {..} => Ok(EventType::Display),
        Event::Window {..} => Ok(EventType::Window),
        Event::KeyDown {..} => Ok(EventType::KeyDown),
        Event::KeyUp {..} => Ok(EventType::KeyUp),
        Event::TextEditing {..} => Ok(EventType::TextEditing),
        Event::TextInput {..} => Ok(EventType::TextInput),
        Event::MouseMotion {..} => Ok(EventType::MouseMotion),
        Event::MouseButtonDown {..} => Ok(EventType::MouseButtonDown),
        Event::MouseButtonUp {..} => Ok(EventType::MouseButtonUp),
        Event::MouseWheel {..} => Ok(EventType::MouseWheel),
        Event::JoyAxisMotion {..} => Ok(EventType::JoyAxisMotion),
        Event::JoyBallMotion {..} => Ok(EventType::JoyBallMotion),
        Event::JoyHatMotion {..} => Ok(EventType::JoyHatMotion),
        Event::JoyButtonDown {..} => Ok(EventType::JoyButtonDown),
        Event::JoyButtonUp {..} => Ok(EventType::JoyButtonUp),
        Event::JoyDeviceAdded {..} => Ok(EventType::JoyDeviceAdded),
        Event::JoyDeviceRemoved {..} => Ok(EventType::JoyDeviceRemoved),
        Event::ControllerAxisMotion {..} => Ok(EventType::ControllerAxisMotion),
        Event::ControllerButtonDown {..} => Ok(EventType::ControllerButtonDown),
        Event::ControllerButtonUp {..} => Ok(EventType::ControllerButtonUp),
        Event::ControllerDeviceAdded {..} => Ok(EventType::ControllerDeviceAdded),
        Event::ControllerDeviceRemoved {..} => Ok(EventType::ControllerDeviceRemoved),
        Event::ControllerDeviceRemapped {..} => Ok(EventType::ControllerDeviceRemapped),
        Event::FingerDown {..} => Ok(EventType::FingerDown),
        Event::FingerUp {..} => Ok(EventType::FingerUp),
        Event::FingerMotion {..} => Ok(EventType::FingerMotion),
        Event::DollarGesture {..} => Ok(EventType::DollarGesture),
        Event::DollarRecord {..} => Ok(EventType::DollarRecord),
        Event::MultiGesture {..} => Ok(EventType::MultiGesture),
        Event::ClipboardUpdate {..} => Ok(EventType::ClipboardUpdate),
        Event::DropFile {..} => Ok(EventType::DropFile),
        Event::DropText {..} => Ok(EventType::DropText),
        Event::DropBegin {..} => Ok(EventType::DropBegin),
        Event::DropComplete {..} => Ok(EventType::DropComplete),
        Event::AudioDeviceAdded {..} => Ok(EventType::AudioDeviceAdded),
        Event::AudioDeviceRemoved {..} => Ok(EventType::AudioDeviceRemoved),
        Event::RenderTargetsReset {..} => Ok(EventType::RenderTargetsReset),
        Event::RenderDeviceReset {..} => Ok(EventType::RenderDeviceReset),
        Event::User {..} => Ok(EventType::User),
        Event::Unknown {..} => Err("Unknown event".to_string()),
    }
}