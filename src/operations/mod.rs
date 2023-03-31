//! This module contains the logic of builtin operations (+, -, etc.) in the form of functions ([OperatorFunction] implementing [SlothFunction]).
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
pub mod cmp;
pub mod div;
pub mod modulo;
pub mod mul;
pub mod sub;
pub mod len;
pub mod inv;






/// Builtin operations. Is a function to allow overloading
#[derive(Clone)]
pub struct OperatorFunction {
    signature: FunctionSignature,
    op_function: fn(Value, Value) -> Value,
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
        // evaluate given values in the scope
        let values = scope.borrow().get_inputs();

        let first_v = match values.get(0) {
            Some(v) => v.borrow().to_owned(),
            None => Value::Any,
        };

        let second_v = match values.get(1) {
            Some(v) => v.borrow().to_owned(),
            None => Value::Any,
        };

        let res = (self.op_function)(first_v, second_v);

        set_return(&scope, program, res)
    }
}




impl OperatorFunction {
    /// Implemented for 2 values, for 1 operands op (like '!'), just use Value::Any
    pub fn new(op: Operator, input_types: Vec<Type>, output_type: Type, op_func: fn(Value, Value) -> Value) -> OperatorFunction {
        let nb_inputs = input_types.len();
        let false_vec = vec![false; nb_inputs];

        let signature = FunctionSignature::new(
            None,
            format!("@{}", op.get_name()),
            None,
            Some(std::iter::zip(input_types.clone(), false_vec).collect()),
            Some(output_type)
        );

        OperatorFunction {
            signature: signature,
            op_function: op_func
        }
    }
}









pub fn get_all() -> Vec<OperatorFunction> {
    let vecs = vec![
        add::get_all(),
        cmp::get_all(),
        modulo::get_all(),
        sub::get_all(),
        mul::get_all(),
        div::get_all(),
        len::get_all(),
        inv::get_all(),
    ];
    vecs.concat()
}