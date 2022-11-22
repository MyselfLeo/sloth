use std::{rc::Rc, cell::RefCell};
use crate::sloth::function::{FunctionSignature, SlothFunction};
use crate::sloth::scope::Scope;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;
use crate::sloth::types::Type;








/// Builtin operations. Is a function to allow overloading
pub struct OperatorFunction {
    signature: FunctionSignature,
    call_function: fn(Rc<RefCell<Scope>>, &mut SlothProgram) -> Result<(), Error>,
}

impl std::fmt::Debug for OperatorFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OperatorFunction").field("signature", &self.signature).finish()
    }
}

impl SlothFunction for OperatorFunction {
    fn get_owner_type(&self) -> Option<Type> {self.signature.owner_type.clone()}
    fn get_signature(&self) -> FunctionSignature {self.signature.clone()}
    fn get_module(&self) -> Option<String> {None}
    fn get_name(&self) -> String {self.signature.name.clone()}
    fn get_output_type(&self) -> Type {self.signature.output_type.as_ref().unwrap().clone()}
    fn get_input_types(&self) -> Option<Vec<Type>> {
        match &self.signature.input_types {
            None => None,
            Some(v) => {
                Some(
                    v.iter()
                     .map(|(t, _)| t.clone())
                     .collect::<Vec<Type>>()
                )
            }
        }
    }

    unsafe fn call(&self, scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
        (self.call_function)(scope, program)
    }
}


impl OperatorFunction {
    pub fn new(name: &str, module: Option<&str>, owner_type: Option<Type>, output_type: Type, call_function: fn(Rc<RefCell<Scope>>, &mut SlothProgram) -> Result<(), Error>) -> BuiltInFunction {
        let new_module = match module {
            Some(s) => Some(s.to_string()),
            None => None
        };

        OperatorFunction {
            signature: FunctionSignature{module: new_module, name: name.to_string(), owner_type, input_types: None, output_type: Some(output_type)},
            call_function
        }
    }
}