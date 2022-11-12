use std::cell::RefCell;
use std::rc::Rc;

use super::function::{FunctionCallSignature};
use super::operation::Operation;
use super::structure::{StructSignature};
use super::types::Type;
use super::value::{Value, DeepClone};
use super::scope::Scope;
use super::program::SlothProgram;
use crate::errors::{Error, ErrMsg};
use crate::position::Position;


#[derive(Clone, Debug)]
/// Expressions are objects that can be evaluated into a value
pub enum Expression {
    Literal(Value, Position),                                                                   // value of the literal
    ListInit(Vec<Rc<Expression>>, Position),                                                    // list initialised in code. Example: [1 2 3 4 5]
    VariableAccess(Option<Rc<Expression>>, String, Position),                                   // ExpressionID to the owner of the field and its name,
    BracketAccess(Rc<Expression>, Rc<Expression>, Position),                                    // Owner, indexing expression
    Operation(Operation, Position),                                                             // Operator to apply to one or 2 values from the Scope Expression stack (via index)
    FunctionCall(Option<Rc<Expression>>, FunctionCallSignature, Vec<Rc<Expression>>, Position), // optional owner (for method calls), name of the function and its list of expressions to be evaluated
    ObjectConstruction(StructSignature, Vec<Rc<Expression>>, Position),                         // The construction of an Object, with the 'new' keyword
    MainCall(Vec<Rc<Value>>)                                                                    // Fake expression used to call the main function
}




impl Expression {
    /// Evaluate the expression in the given context (scope and program) and return its value
    pub unsafe fn evaluate(&self, scope: Rc<RefCell<Scope>>, program: *mut SlothProgram, for_assignment: bool) -> Result<Rc<RefCell<Value>>, Error> {

        let res = match self {
            // return this literal value
            Expression::Literal(v, _) => Ok(Rc::new(RefCell::new(v.clone()))),

            // a list
            Expression::ListInit(exprs, _) => {
                let mut values = Vec::new();
                let mut list_type = Type::Any;

                if exprs.len() != 0 {
                    values.push(exprs[0].evaluate(scope.clone(), program, false)?);
                    list_type = values[0].borrow().get_type();

                    // Add the other elements to the value list, but checking the type of the value first
                    for expr in exprs.iter().skip(1) {
                        let value = expr.evaluate(scope.clone(), program, false)?;

                        if value.borrow().get_type() == list_type {values.push(value);}
                        else {
                            let err_msg = format!("Created a list of type '{}' but this value is of type '{}'", list_type, value.borrow().get_type());
                            return Err(Error::new(ErrMsg::InvalidArguments(err_msg), Some(expr.get_pos())));
                        }
                    }
                }


                Ok(Rc::new(RefCell::new(Value::List(list_type, values))))
            },




            // return the value stored in this variable
            Expression::VariableAccess(owner, name, p) => {

                match owner {
                    // Field of a value
                    Some(o) => {
                        // Get the reference to the owner
                        let owner_ref = o.evaluate(scope.clone(), program, false)?;
                        let field = owner_ref.borrow().get_field(name);
                        match field {
                            Ok(v) => Ok(v),
                            Err(e) => Err(Error::new(ErrMsg::RuntimeError(e), Some(p.clone())))
                        }
                    },

                    // Variable in the scope
                    None => {
                        
                        // if not set, create the variable or return an error depending if it's an assignment or not
                        if !scope.borrow().is_set(name) && !program.as_ref().unwrap().is_set(name) {

                            if for_assignment {
                                match scope.try_borrow_mut() {
                                    Ok(mut brrw) => {
                                        match brrw.push_variable(name.clone(), Rc::new(RefCell::new(Value::Any))) {
                                            Ok(()) => (),
                                            Err(e) => return Err(Error::new(ErrMsg::RuntimeError(e.to_string()), Some(p.clone())))
                                        }
                                    },
                                    Err(e) => {
                                        return Err(Error::new(ErrMsg::RustError(e.to_string()), Some(p.clone())))
                                    }
                                }
                            }
                            else {
                                let err_msg = format!("Called uninitialized variable '{}'", name);
                                return Err(Error::new(ErrMsg::RuntimeError(err_msg), Some(p.clone()))) 
                            }
                        }

                        // Prevent using statics for assignment
                        if !scope.borrow().is_set(name) && program.as_ref().unwrap().is_set(name) && for_assignment {
                            let err_msg = format!("{} is a static expression, it cannot be assigned a value", name);
                            return Err(Error::new(ErrMsg::RuntimeError(err_msg), Some(p.clone())))
                        }

                        // Get the value directly from the scope
                        match scope.borrow().get_variable(name.clone(), program.as_mut().unwrap()) {
                            Ok(r) => Ok(r),
                            Err(mut e) => {
                                e.clog_pos(p.clone());
                                Err(e)
                            }
                        }
                    }
                }
            },




            // process the operation and return the result
            Expression::Operation(operation, p) => {
                let value = match operation.execute(scope, program.as_mut().unwrap()) {
                    Ok(v) => v,
                    Err(mut e) => {
                        e.clog_pos(p.clone());
                        return Err(e)
                    }
                };
                Ok(Rc::new(RefCell::new(value)))
            },





            Expression::MainCall(arguments) => {
                
            },



            
            Expression::FunctionCall(owner, signature, arguments, p) => {

                // Get the reference to each value. The inputs by value (without "~") are deep-cloned at a later step,
                // and are added to the function scope even after
                let mut inputs = arguments.iter().map(|e| e.evaluate(scope.clone(), program, false)).collect::<Result<Vec<Rc<RefCell<Value>>>, Error>>()?;

                // Get the reference to the owner value, if any
                let owner_value = match owner {
                    Some(s) => Some(s.evaluate(scope.clone(), program, false)?),
                    None => None
                };

                
                // we can complete the signature with the input types and the owner type
                let mut signature = signature.clone();
                if let Some(v) = owner_value {signature.owner_type = Some(v.borrow().get_type())}
                let input_types: Vec<Type> = inputs.iter().map(|i| i.borrow().get_type()).collect();
                signature.input_types = input_types;
                

                // get the function corresponding to the signature
                let function = match program.as_ref().unwrap().get_function(&signature_clone) {
                    Ok(f) => f,
                    Err(e) => {return Err(Error::new(ErrMsg::RuntimeError(e), Some(p.clone())))}
                };




                /*
                // if the values are cloned, allocate a new Value instead of using the reference given by expr.evaluate()
                    if !inputs_ref_or_cloned[i] {
                        let cloned_value = value.borrow().deep_clone();
                        value = match cloned_value {
                            Ok(v) => v,
                            Err(e) => return Err(Error::new(ErrMsg::RuntimeError(e), Some(p.clone())))
                        };
                    }

            

                signature_clone.owner_type = match owner_value {
                    Some(ref v) => {
                        match v.borrow().get_type() {
                            Type::List(_t) => Some(Type::List(Box::new(Type::Any))),
                            t => Some(t),
                        }
                    },
                    None => None
                };
                 */


                let method = match program.as_ref().unwrap().get_function(&signature_clone) {
                    Ok(f) => f,
                    Err(e) => {return Err(Error::new(ErrMsg::RuntimeError(e), Some(p.clone())))}
                };


                let inputs_ref_or_cloned: Vec<bool> = match method.get_signature().input_types {
                    Some(v) => v.iter().map(|(_, b)| *b).collect(),
                    None => vec![true; arguments.len()]
                };


                // Create a new scope for the execution of the method
                let func_scope = Rc::new(RefCell::new(Scope::new()));


                // Evaluate each given expression in the scope, and create an input variable (@0, @1, etc.) with the set value
                for (i, param) in arguments.iter().enumerate() {
                    let mut value = param.evaluate(scope.clone(), program, false)?;

                    // if the values are cloned, allocate a new Value instead of using the reference given by expr.evaluate()
                    if !inputs_ref_or_cloned[i] {
                        let cloned_value = value.borrow().deep_clone();
                        value = match cloned_value {
                            Ok(v) => v,
                            Err(e) => return Err(Error::new(ErrMsg::RuntimeError(e), Some(p.clone())))
                        };
                    }


                    match func_scope.try_borrow_mut() {
                        Ok(mut reference) => match (*reference).push_variable(format!("@{}", i), value) {
                            Ok(()) => (),
                            Err(e) => return Err(Error::new(ErrMsg::RuntimeError(e), Some(p.clone())))
                        },
                        Err(e) => return Err(Error::new(ErrMsg::RustError(e.to_string()), Some(p.clone())))
                    };
                }

                // Create the @return variable, with default value, and the "@self" variable, containing a copy of the value stored in the variable
                {
                    let default_value = method.get_output_type().default();
                    match func_scope.try_borrow_mut() {
                        Ok(mut reference) => {
                            match (*reference).push_variable("@return".to_string(), Rc::new(RefCell::new(default_value))) {
                                Ok(()) => (),
                                Err(e) => return Err(Error::new(ErrMsg::RuntimeError(e), Some(p.clone())))
                            };

                            match owner_value {
                                Some(v) => {
                                    match (*reference).push_variable("@self".to_string(), v.clone()) {
                                        Ok(()) => (),
                                        Err(e) => return Err(Error::new(ErrMsg::RuntimeError(e), Some(p.clone())))
                                    };
                                },

                                None => ()
                            };
                        },
                        Err(e) => return Err(Error::new(ErrMsg::RustError(e.to_string()), Some(p.clone())))
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
                            Err(Error::new(ErrMsg::ReturnValueError(err_msg), Some(p.clone())))
                        }
                        else {Ok(v.clone())}
                    },
                    Err(mut e) => {
                        e.clog_pos(p.clone());
                        Err(e)
                    }
                };

                res
            },





            Expression::ObjectConstruction(signature, given_fields, p) => {
                // Get the structure definition from the program
                let blueprint = match program.as_mut().unwrap().get_struct(signature) {
                    Ok(v) => v,
                    Err(e) => return Err(Error::new(ErrMsg::RuntimeError(e), Some(p.clone())))
                };

                // Evaluate each given values
                let mut given_values = Vec::new();

                for expr in given_fields {
                    given_values.push(expr.evaluate(scope.clone(), program, false)?);
                }

                // Build the object
                let object = match blueprint.build(given_values) {
                    Ok(v) => v,
                    Err(e) => return Err(Error::new(ErrMsg::InvalidArguments(e), Some(p.clone())))
                };
                // Return the value
                Ok(Rc::new(RefCell::new(Value::Object(object))))
            },

            
            Expression::BracketAccess(owner, access, p) => {
                let access_ref = access.evaluate(scope.clone(), program, false)?;
                let access_str = access_ref.borrow().to_string();


                // create a new expression::variableaccess and evaluate it
                let expr = Expression::VariableAccess(Some(owner.clone()), access_str, p.clone());
                expr.evaluate(scope, program, for_assignment)
            },
        };


        match res {
            Ok(v) => Ok(v),
            Err(mut e) => {
                e.clog_pos(self.get_pos());
                Err(e)
            }
        }
    }





    /// Return the position of the expression
    pub fn get_pos(&self) -> Position {
        match self {
            Expression::Literal(_, p) => p,
            Expression::ListInit(_, p) => p,
            Expression::VariableAccess(_, _, p) => p,
            Expression::Operation(_, p) => p,
            Expression::FunctionCall(_, _, _, p) => p,
            Expression::ObjectConstruction(_, _, p) => p,
            Expression::BracketAccess(_, _, p) => p,
        }.clone() 
    }
}
