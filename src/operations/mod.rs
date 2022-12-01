use std::{rc::Rc, cell::RefCell};
use crate::lexer::Operator;
use crate::sloth::function::{FunctionSignature, SlothFunction};
use crate::sloth::scope::Scope;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;
use crate::sloth::types::Type;
use crate::sloth::value::Value;
use crate::builtins::set_return;

pub mod add;







/// Builtin operations. Is a function to allow overloading
pub struct OperatorFunction {
    signature: FunctionSignature,
    call_function: Box<dyn Fn(Rc<RefCell<Scope>>, &mut SlothProgram) -> Result<(), Error>>,
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
    /// Implemented for 2 values, for 1 operands op (like '!'), just use Value::Any
    pub fn new(op: Operator, input_types: Vec<Type>, output_type: Type, op_func: &'static dyn Fn(Value, Value) -> Value) -> OperatorFunction {
        let nb_inputs = input_types.len();
        let false_vec = vec![false; nb_inputs];

        let signature = FunctionSignature::new(
            None,
            format!("@{}", op.get_name()),
            None,
            Some(std::iter::zip(input_types.clone(), false_vec).collect()),
            Some(output_type)
        );


        let function = |s: Rc<RefCell<Scope>>, p: &mut SlothProgram| {
            // evaluate given values in the scope
            let values = s.borrow().get_inputs();
            
            let first_v = values.get(0).unwrap_or(&Rc::new(RefCell::new(Value::Any))).borrow().to_owned();
            let second_v = values.get(0).unwrap_or(&Rc::new(RefCell::new(Value::Any))).borrow().to_owned();

            set_return(&s, p, op_func(first_v, second_v))
        };


        OperatorFunction {
            signature: signature,
            call_function: Box::new(function)
        }
    }
}









pub fn get_all() -> Vec<OperatorFunction> {
    todo!();
    //add::get_all()
}