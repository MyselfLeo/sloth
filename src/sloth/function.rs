use crate::errors::{Error, ErrorMessage};
use super::program::SlothProgram;
use super::statement::Statement;
use super::{types::Type};
use super::scope::{Scope};


#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct FunctionID {
    pub module: Option<String>,     // In case of a function imported (from builtin for example)
    pub name: String,               // name of the function
    pub owner_type: Option<Type>    // in case the function is a method
}

impl FunctionID {
    pub fn new(module: Option<String>, name: String, owner_type: Option<Type>) -> FunctionID {
        FunctionID {module, name, owner_type}
    }
}





pub trait SlothFunction {
    /// Return the type owning this function, or None if this is not a method
    fn get_owner_type(&self) -> Option<Type>;

    /// Return a FunctionID representing this function
    fn get_function_id(&self) -> FunctionID;

    /// Return the module from which the function comes
    fn get_module(&self) -> Option<String>;
    
    /// Return the name of the function
    fn get_name(&self) -> String;

    /// Call the function, like a procedure, in the given scope.
    /// The FunctionCall statement must create a new scope for this function. The 'scope' given to this method
    /// IS NOT the Scope in which the function is called, but the scope INSIDE of the function
    unsafe fn call(&self,  scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error>;

    fn get_output_type(&self) -> Type;
}

/// Function defined in the code, written in Sloth
pub struct CustomFunction {
    pub name: String,

    pub owner_type: Option<Type>,

    pub input_types: Vec<Type>,
    pub output_type: Type,

    pub instructions: Vec<Statement>,
}


impl SlothFunction for CustomFunction {
    fn get_owner_type(&self) -> Option<Type> {
        Some(self.output_type.clone())
    }

    fn get_function_id(&self) -> FunctionID {
        FunctionID { module: None, name: self.name.clone(), owner_type: self.owner_type.clone() }
    }

    fn get_module(&self) -> Option<String> {
        None
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_output_type(&self) -> Type {
        return self.output_type.clone()
    }
    

    unsafe fn call(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
        // get the given arguments
        let args = scope.get_inputs();

        // Check that the number of inputs given matches the number required
        if args.len() != self.input_types.len() {
            let err_msg = format!("Called function {} with {} argument(s), but the function requires {} argument(s)", self.name, args.len(), self.input_types.len());
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
        }

        // Check that the given input types match the ones from the definition
        let mut i = 0;
        for (given, required) in std::iter::zip(args, &self.input_types) {
            let given_type = given.get_type();
            if given_type != *required {
                let err_msg = format!("Function {} was called with argument of type {} at position {}, where argument of type {} was required", self.name, given_type, i, required);
                return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None));
            }
            i += 1;
        }

        // Call each statement of the function
        for statement in &self.instructions {
            statement.apply(scope, program)?;
        };

        return Ok(())
    }
}



