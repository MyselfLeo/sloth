use super::function::FunctionSignature;
use super::value::Value;
use super::operator::{Operator, apply_op};
use super::scope::Scope;
use super::program::SlothProgram;
use crate::errors::{Error, ErrorMessage};
use crate::tokenizer::ElementPosition;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/// Used by scopes to reference to parent scope in the Scope stack
pub struct ExpressionID {
    id: u64
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
    VariableCall(String, ElementPosition),                                               // name of the variable
    ParameterCall(ExpressionID, String, ElementPosition),                                // name of a parameter of a structure or built-in that can be accessed
    Operation(Operator, Option<ExpressionID>, Option<ExpressionID>, ElementPosition),    // Operator to apply to one or 2 values from the Scope Expression stack (via index)
    FunctionCall(FunctionSignature, Vec<ExpressionID>, ElementPosition),                 // name of the function and its list of expressions to be evaluated
    MethodCall(ExpressionID, FunctionSignature, Vec<ExpressionID>, ElementPosition)      // call of a method of a Value
}




impl Expression {
    /// Evaluate the expression in the given context (scope and program) and return its value
    pub unsafe fn evaluate(&self, scope: &mut Scope, program: *mut SlothProgram) -> Result<Value, Error> {
        match self {
            // return this literal value
            Expression::Literal(v, _) => Ok(v.clone()),

            // return the value stored in this variable
            Expression::VariableCall(name, p) => {
                match scope.get_variable(name.clone(), program.as_mut().unwrap()) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::new(ErrorMessage::UnexpectedExpression(e), Some(p.clone())))
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
                        
                        Some(expr.evaluate(scope, program)?)
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
                    Ok(v) => Ok(v),
                    Err(s) => Err(Error::new(ErrorMessage::InvalidArguments(s), Some(p.clone())))
                }
            }

            // return the result of the function call
            Expression::FunctionCall(f_id, param, p) => {
                // Create a new scope for the execution of the function
                let func_scope_id = program.as_mut().unwrap().new_scope(Some(scope.id));

                let mut func_scope = match program.as_mut().unwrap().get_scope(func_scope_id) {
                    Ok(s) => s.clone(),
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };

                // Evaluate each given expression in the scope, and create an input variable (@0, @1, etc.) with the set value
                for (i, param_expr_id) in param.iter().enumerate() {

                    let expr = match program.as_ref().unwrap().get_expr(*param_expr_id) {
                        Ok(e) => e,
                        Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                    };

                    let value = expr.evaluate(scope, program)?;
                    func_scope.set_variable(format!("@{}", i), value);
                }

                // Get the function
                let function = match program.as_ref().unwrap().get_function(f_id) {
                    Ok(f) => f,
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };


                // Create the @return variable, with default value
                let default_value = function.get_output_type().default();
                func_scope.set_variable("@return".to_string(), default_value);
                
                // run the function in the given scope.
                // If the function call returned an error without position, set its position to this function call's
                match function.call(&mut func_scope, program.as_mut().unwrap()) {
                    Ok(()) => (),
                    Err(mut e) => {
                        if e.position.is_none() && p.filename != "" {e.position = Some(p.clone());}
                        return Err(e)
                    }
                }

                // remove the scope from the program
                program.as_mut().unwrap().dump_scope(&func_scope_id);

                // return the value in the '@return' variable, but check its type first
                match func_scope.get_variable("@return".to_string(), program.as_mut().unwrap()) {
                    Ok(v) => {
                        if v.get_type() != function.get_output_type() {
                            let err_msg = format!("Function {} should return a value of type {}, but it returned {} which is of type {}", function.get_name(), function.get_output_type(), v.to_string(), v.get_type());
                            Err(Error::new(ErrorMessage::ReturnValueError(err_msg), Some(p.clone())))
                        }
                        else {Ok(v)}
                    },
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                }
            },


            Expression::ParameterCall(_owner, _v_name, _p) => unimplemented!("Parameters calls are not implemented yet"),
            
            Expression::MethodCall(owner, signature, arguments, p) => {

                let mut signature_clone = signature.clone();

                // Get the expression on which is called the method
                let expr = match program.as_ref().unwrap().get_expr(*owner) {
                    Ok(e) => e,
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };

                // get the value stored in the variable
                let value = expr.clone().evaluate(scope, program)?;
                
                // try to find if the method, applied to the type of the value
                signature_clone.owner_type = Some(value.get_type());
                let method = match program.as_ref().unwrap().get_function(&signature_clone) {
                    Ok(f) => f,
                    Err(e) => {
                        return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };


                // Create a new scope for the execution of the method
                let method_scope_id = program.as_mut().unwrap().new_scope(Some(scope.id));
                let mut func_scope = match program.as_mut().unwrap().get_scope(method_scope_id) {
                    Ok(s) => s.clone(),
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };

                // Evaluate each given expression in the scope, and create an input variable (@0, @1, etc.) with the set value
                for (i, param_expr_id) in arguments.iter().enumerate() {

                    let expr = match program.as_ref().unwrap().get_expr(*param_expr_id) {
                        Ok(e) => e,
                        Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                    };

                    let value = expr.evaluate(scope, program)?;
                    func_scope.set_variable(format!("@{}", i), value);
                }


                // Create the @return variable, with default value
                let default_value = method.get_output_type().default();
                func_scope.set_variable("@return".to_string(), default_value);


                // create the "@self" variable, containing a copy of the value stored in the variable
                func_scope.set_variable("@self".to_string(), value);


                // run the method in the given scope.
                // If the method call returned an error without position, set its position to this function call's
                match method.call(&mut func_scope, program.as_mut().unwrap()) {
                    Ok(()) => (),
                    Err(mut e) => {
                        if e.position.is_none() && p.filename != "" {e.position = Some(p.clone());}
                        return Err(e)
                    }
                };


                if let Expression::VariableCall(name, _) = expr {
                    // Set the variable on which was called the function to the new value of "@self"
                    let new_self = match func_scope.get_variable("@self".to_string(), program.as_mut().unwrap()) {
                        Ok(v) => (v),
                        Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                    };
                    scope.set_variable(name, new_self);
                }


                // remove the scope from the program
                program.as_mut().unwrap().dump_scope(&method_scope_id);


                // return the value in the '@return' variable, but check its type first
                match func_scope.get_variable("@return".to_string(), program.as_mut().unwrap()) {
                    Ok(v) => {
                        if v.get_type() != method.get_output_type() {
                            let err_msg = format!("Function {} should return a value of type {}, but it returned {} which is of type {}", method.get_name(), method.get_output_type(), v.to_string(), v.get_type());
                            return Err(Error::new(ErrorMessage::ReturnValueError(err_msg), Some(p.clone())))
                        }
                        else {return Ok(v)}
                    },
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                }
            }
        }
    }
}