use std::cell::RefCell;
use std::rc::Rc;

use crate::errors::{Error, ErrorMessage};
use super::program::SlothProgram;
use super::statement::Statement;
use super::{types::Type};
use super::scope::{Scope};


#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct FunctionSignature {
    pub module: Option<String>,                     // In case of a function imported (from builtin for example)
    pub name: String,                               // name of the function
    pub owner_type: Option<Type>,                   // in case the function is a method
    pub input_types: Option<Vec<(Type, bool)>>,     // Can be an option as some functions don't have a specific input types pattern (like the main function, or builtins). The bool is whether to pass by reference or not
    pub output_type: Option<Type>,
}

impl FunctionSignature {
    pub fn new(module: Option<String>, name: String, owner_type: Option<Type>, input_types: Option<Vec<(Type, bool)>>, output_type: Option<Type>) -> FunctionSignature {
        FunctionSignature {module, name, owner_type, input_types, output_type}
    }
}





pub trait SlothFunction {
    /// Return the type owning this function, or None if this is not a method
    fn get_owner_type(&self) -> Option<Type>;

    /// Return a FunctionID representing this function
    fn get_signature(&self) -> FunctionSignature;

    /// Return the module from which the function comes
    fn get_module(&self) -> Option<String>;
    
    /// Return the name of the function
    fn get_name(&self) -> String;

    /// Return the output type of the function
    fn get_output_type(&self) -> Type;

    /// Execute the function
    unsafe fn call(&self,  scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error>;
}



/// Function defined in the code, written in Sloth
/// The input_types and output_type can't be None because Sloth code can't permit it
pub struct CustomFunction {
    pub signature: FunctionSignature,
    pub instructions: Vec<Statement>,
}


impl SlothFunction for CustomFunction {
    fn get_owner_type(&self) -> Option<Type> {self.signature.owner_type.clone()}
    fn get_signature(&self) -> FunctionSignature {self.signature.clone()}
    fn get_module(&self) -> Option<String> {self.signature.module.clone()}
    fn get_name(&self) -> String {self.signature.name.clone()}
    fn get_output_type(&self) -> Type {self.signature.output_type.as_ref().unwrap().clone()}

    unsafe fn call(&self, scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
        // get the given arguments
        let args = scope.borrow().get_inputs();

        let self_inputs = match &self.signature.input_types {
            Some(v) => v.clone(),
            None => panic!("The SlothFunction {:?} doesn't have defined input types", self.signature)
        };

        // Check that the number of inputs given matches the number required
        if args.len() != self_inputs.len() {
            let err_msg = format!("Called function {} with {} argument(s), but the function requires {} argument(s)", self.get_name(), args.len(), self_inputs.len());
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
        }

        // Check that the given input types match the ones from the definition
        let mut i = 0;
        for (given, (required, _)) in std::iter::zip(args, &self_inputs) {
            let given_type = given.borrow().get_type();
            if given_type != *required {
                let err_msg = format!("Function {} was called with argument of type {} at position {}, where argument of type {} was required", self.get_name(), given_type, i, required);
                return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
            }
            i += 1;
        }

        // Call each statement of the function
        for statement in &self.instructions {
            statement.apply(scope.clone(), program)?;
        };

        return Ok(())
    }
}