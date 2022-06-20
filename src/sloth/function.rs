use super::{value::Value, types::Type};
use super::scope::{Scope, ScopeID};

pub trait SlothFunction {
    /// Call the function with the given arguments
    /// TODO: This will need a scope input
    fn call(&self, args: Vec<Box<dyn Value>>, scope: &Scope) -> Result<Option<Box<dyn Value>>, String>;
}

/// Function defined in the code, written in Sloth
struct CustomFunction {
    name: String,
    input_types: Vec<Type>,
    output_type: Type,

    // instructions            TODO: sequence of statements here
}

impl SlothFunction for CustomFunction {
    fn call(&self, args: Vec<Box<dyn Value>>, scope: &Scope) -> Result<Option<Box<dyn Value>>, String> {

        // Check that the number of inputs given matches the number required
        if args.len() != self.input_types.len() {
            let err_msg = format!("Called function {} with {} argument(s), but the function requires {} argument(s)", self.name, args.len(), self.input_types.len());
            return Err(err_msg.to_string());
        }

        // Check that the given input types match the ones from the definition
        let mut i = 0;
        for (given, required) in std::iter::zip(args, &self.input_types) {
            let given_type = (*given).get_type();
            if given_type != *required {
                let err_msg = format!("Function {} was called with argument of type {} at position {}, where argument of type {} was required", self.name, given_type, i, required);
                return Err(err_msg.to_string());
            }
            i += 1;
        }

        // TODO: Implement function execution
        todo!();
    }
}