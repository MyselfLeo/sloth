use std::cell::RefCell;
use std::rc::Rc;

use super::function::{FunctionSignature};
use super::statement::IdentifierWrapper;
use super::structure::{StructSignature};
use super::types::Type;
use super::value::{Value, RecursiveRereference};
use super::operator::{Operator, apply_op};
use super::scope::Scope;
use super::program::SlothProgram;
use crate::errors::{Error, ErrorMessage};
use crate::tokenizer::ElementPosition;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/// Used by scopes to reference to parent scope in the Scope stack
pub struct ExpressionID {
    pub id: u64
}
impl ExpressionID {
    pub fn new(value: u64) -> ExpressionID {
        ExpressionID { id: value }
    }
}


#[derive(Clone, Debug)]
/// Expressions are objects that can be evaluated into a value
pub enum Expression {
    Literal(Value, ElementPosition),                                                     // value of the literal
    ListInit(Vec<ExpressionID>, ElementPosition),                                        // list initialised in code. Example: [1 2 3 4 5]
    VariableCall(IdentifierWrapper, ElementPosition),                                    // identifierwrapper linking to the variable
    Operation(Operator, Option<ExpressionID>, Option<ExpressionID>, ElementPosition),    // Operator to apply to one or 2 values from the Scope Expression stack (via index)
    FunctionCall(FunctionSignature, Vec<ExpressionID>, ElementPosition),                 // name of the function and its list of expressions to be evaluated
    MethodCall(ExpressionID, FunctionSignature, Vec<ExpressionID>, ElementPosition),     // call of a method of a Value
    ObjectConstruction(StructSignature, Vec<ExpressionID>, ElementPosition),             // The construction of an Object, with the 'new' keyword
}




impl Expression {
    /// Evaluate the expression in the given context (scope and program) and return its value
    pub unsafe fn evaluate(&self, scope: Rc<RefCell<Scope>>, program: *mut SlothProgram) -> Result<Rc<RefCell<Value>>, Error> {

        match self {
            // return this literal value
            Expression::Literal(v, _) => Ok(Rc::new(RefCell::new(v.clone()))),

            // a list
            Expression::ListInit(exprs, p) => {
                let mut values = Vec::new();
                let mut list_type = Type::Any;

                if exprs.len() != 0 {
                    // get the type of the list from the first expression
                    let expr = match program.as_ref().unwrap().get_expr(exprs[0]) {
                        Ok(e) => e,
                        Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                    };
                    values.push(expr.evaluate(scope.clone(), program)?);

                    list_type = values[0].borrow().get_type();


                    // Add the other elements to the value list, but checking the type of the value first
                    for id in exprs.iter().skip(1) {
                        let expr = match program.as_ref().unwrap().get_expr(*id) {
                            Ok(e) => e,
                            Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                        };

                        let value = expr.evaluate(scope.clone(), program)?;

                        if value.borrow().get_type() == list_type {values.push(value);}
                        else {
                            let err_msg = format!("Created a list of type '{}' but this value is of type '{}'", list_type, value.borrow().get_type());
                            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), Some(expr.get_pos())));
                        }
                    }
                }


                Ok(Rc::new(RefCell::new(Value::List(list_type, values))))
            },




            // return the value stored in this variable
            Expression::VariableCall(wrapper, p) => {
                match wrapper.get_value(scope, program.as_mut().unwrap()) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))
                }
            },



            // process the operation and return the result
            Expression::Operation(op, lhs, rhs, p) => {
                // Get the value of both lhs and rhs
                let lhs = match lhs {
                    None => None,
                    Some(i) => {
                        let expr = match program.as_ref().unwrap().get_expr(*i) {
                            Ok(e) => e,
                            Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                        };
                        
                        Some(expr.evaluate(scope.clone(), program)?)
                    }
                };
                let rhs = match rhs {
                    None => None,
                    Some(i) => {
                        let expr = match program.as_ref().unwrap().get_expr(*i) {
                            Ok(e) => e,
                            Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                        };
                        
                        Some(expr.evaluate(scope, program)?)
                    }
                };
                
                //apply op
                match apply_op(op, lhs, rhs) {
                    Ok(v) => Ok(Rc::new(RefCell::new(v))),
                    Err(s) => Err(Error::new(ErrorMessage::InvalidArguments(s), Some(p.clone())))
                }
            }

            // return the result of the function call
            Expression::FunctionCall(f_id, param, p) => {
                // Create a new scope for the execution of the function
                let func_scope = Rc::new(RefCell::new(Scope::new(Some(program.as_ref().unwrap().main_scope()))));

                // Get the function
                let function = match program.as_ref().unwrap().get_function(f_id) {
                    Ok(f) => f,
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };

                let inputs_ref_or_cloned: Vec<bool> = match function.get_signature().input_types {
                    Some(v) => v.iter().map(|(_, b)| *b).collect(),
                    None => vec![true; param.len()]
                };

                // Evaluate each given expression in the scope, and create an input variable (@0, @1, etc.) with the set value
                for (i, param_expr_id) in param.iter().enumerate() {

                    let expr = match program.as_ref().unwrap().get_expr(*param_expr_id) {
                        Ok(e) => e,
                        Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                    };

                    let mut value = expr.evaluate(scope.clone(), program)?;


                    // if the values are cloned, allocate a new Value instead of using the reference given by expr.evaluate()
                    if !inputs_ref_or_cloned[i] {
                        let cloned_value = value.borrow().rereference();
                        value = match cloned_value {
                            Ok(v) => v,
                            Err(e) => return Err(Error::new(ErrorMessage::RustError(e), Some(p.clone())))
                        };
                    }

                    match func_scope.try_borrow_mut() {
                        Ok(mut reference) => match (*reference).push_variable(format!("@{}", i), value) {
                            Ok(()) => (),
                            Err(e) => return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))
                        },
                        Err(e) => return Err(Error::new(ErrorMessage::RustError(e.to_string()), Some(p.clone())))
                    };
                }



                // Create the @return variable, with default value
                let default_value = function.get_output_type().default();

                match func_scope.try_borrow_mut() {
                    Ok(mut reference) => match (*reference).push_variable("@return".to_string(), Rc::new(RefCell::new(default_value))){
                        Ok(()) => (),
                        Err(e) => return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))
                    },
                    Err(e) => return Err(Error::new(ErrorMessage::RustError(e.to_string()), Some(p.clone())))
                };
                
                // run the function in the given scope.
                // If the function call returned an error without position, set its position to this function call's
                match function.call(func_scope.clone(), program.as_mut().unwrap()) {
                    Ok(()) => (),
                    Err(mut e) => {
                        if e.position.is_none() && p.filename != "" {e.position = Some(p.clone());}
                        return Err(e)
                    }
                }

                // return the value in the '@return' variable, but check its type first
                let res = match func_scope.borrow().get_variable("@return".to_string(), program.as_mut().unwrap()) {
                    Ok(v) => {
                        let brrw = v.borrow();
                        if brrw.get_type() != function.get_output_type() {
                            let err_msg = format!("Function {} should return a value of type {}, but it returned '{}' which is of type {}", function.get_name(), function.get_output_type(), brrw.to_string(), brrw.get_type());
                            Err(Error::new(ErrorMessage::ReturnValueError(err_msg), Some(p.clone())))
                        }
                        else {Ok(v.clone())}
                    },
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };

                res
            },

            
            Expression::MethodCall(owner, signature, arguments, p) => {

                let mut signature_clone = signature.clone();

                // Get the expression on which is called the method
                let expr = match program.as_ref().unwrap().get_expr(*owner) {
                    Ok(e) => e,
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };

                // get the value stored in the variable
                let value = expr.evaluate(scope.clone(), program)?;

                
                // try to find if the method, applied to the type of the value, exists
                // TODO: Make defining owned function both work for 'list' (means List(Any)) and 'list[type]'
                signature_clone.owner_type = match value.borrow().get_type() {
                    Type::List(_t) => Some(Type::List(Box::new(Type::Any))),
                    t => Some(t),
                };


                let method = match program.as_ref().unwrap().get_function(&signature_clone) {
                    Ok(f) => f,
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };


                // Create a new scope for the execution of the method
                let func_scope = Rc::new(RefCell::new(Scope::new(Some(program.as_ref().unwrap().main_scope()))));


                // Evaluate each given expression in the scope, and create an input variable (@0, @1, etc.) with the set value
                for (i, param_expr_id) in arguments.iter().enumerate() {

                    let expr = match program.as_ref().unwrap().get_expr(*param_expr_id) {
                        Ok(e) => e,
                        Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                    };

                    let v = expr.evaluate(scope.clone(), program)?;

                    match func_scope.try_borrow_mut() {
                        Ok(mut reference) => match (*reference).push_variable(format!("@{}", i), v) {
                            Ok(()) => (),
                            Err(e) => return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))
                        },
                        Err(e) => return Err(Error::new(ErrorMessage::RustError(e.to_string()), Some(p.clone())))
                    };
                }

                // Create the @return variable, with default value, and the "@self" variable, containing a copy of the value stored in the variable
                {
                    let default_value = method.get_output_type().default();
                    match func_scope.try_borrow_mut() {
                        Ok(mut reference) => {
                            match (*reference).push_variable("@return".to_string(), Rc::new(RefCell::new(default_value))) {
                                Ok(()) => (),
                                Err(e) => return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))
                            };
                            match (*reference).push_variable("@self".to_string(), value.clone()) {
                                Ok(()) => (),
                                Err(e) => return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))
                            };
                        },
                        Err(e) => return Err(Error::new(ErrorMessage::RustError(e.to_string()), Some(p.clone())))
                    };
                }

                // run the method in the given scope.
                // If the method call returned an error without position, set its position to this function call's
                match method.call(func_scope.clone(), program.as_mut().unwrap()) {
                    Ok(()) => (),
                    Err(mut e) => {
                        if e.position.is_none() && p.filename != "" {e.position = Some(p.clone());}
                        return Err(e)
                    }
                };




                // return the value in the '@return' variable, but check its type first
                let res = match func_scope.borrow().get_variable("@return".to_string(), program.as_mut().unwrap()) {
                    Ok(v) => {
                        let brrw = v.borrow();
                        if brrw.get_type() != method.get_output_type() {
                            let err_msg = format!("Function {} should return a value of type {}, but it returned {} which is of type {}", method.get_name(), method.get_output_type(), brrw.to_string(), brrw.get_type());
                            Err(Error::new(ErrorMessage::ReturnValueError(err_msg), Some(p.clone())))
                        }
                        else {Ok(v.clone())}
                    },
                    Err(e) => {Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };

                res
            },





            Expression::ObjectConstruction(signature, given_fields, p) => {
                // Get the structure definition from the program
                let blueprint = match program.as_mut().unwrap().get_struct(signature) {
                    Ok(v) => v,
                    Err(e) => return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))
                };

                // Evaluate each given values
                let mut given_values = Vec::new();

                for expr_id in given_fields {
                    let expr = match program.as_ref().unwrap().get_expr(*expr_id) {
                        Ok(e) => e,
                        Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                    };

                    given_values.push(expr.evaluate(scope.clone(), program)?);
                }

                // Build the object
                let object = match blueprint.build(given_values) {
                    Ok(v) => v,
                    Err(e) => return Err(Error::new(ErrorMessage::InvalidArguments(e), Some(p.clone())))
                };
                // Return the value
                Ok(Rc::new(RefCell::new(Value::Object(object))))
            },
        }
    }





    /// Return the position of the expression
    pub fn get_pos(&self) -> ElementPosition {
        match self {
            Expression::Literal(_, p) => p,
            Expression::ListInit(_, p) => p,
            Expression::VariableCall(_, p) => p,
            Expression::Operation(_, _, _, p) => p,
            Expression::FunctionCall(_, _, p) => p,
            Expression::MethodCall(_, _, _, p) => p,
            Expression::ObjectConstruction(_, _, p) => p,
        }.clone() 
    }
}
