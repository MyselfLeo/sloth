use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use super::function::{FunctionCallSignature, SlothFunction};
use super::structure::{StructSignature};
use super::types::Type;
use super::value::{Value, DeepClone};
use super::scope::Scope;
use super::program::{SlothProgram, ENTRY_POINT_NAME};
use crate::errors::{Error, ErrMsg};
use crate::position::Position;
use crate::propagate;


#[derive(Clone, Debug)]
/// Expressions are objects that can be evaluated into a value
pub enum Expression {
    Literal(Value, Position),                                                                   // value of the literal
    ListInit(Vec<Rc<Expression>>, Position),                                                    // list initialised in code. Example: [1 2 3 4 5]
    VariableAccess(Option<Rc<Expression>>, String, Position),                                   // ExpressionID to the owner of the field and its name,
    BracketAccess(Rc<Expression>, Rc<Expression>, Position),                                    // Owner, indexing expression
    FunctionCall(Option<Rc<Expression>>, FunctionCallSignature, Vec<Rc<Expression>>, Position), // optional owner (for method calls), name of the function and its list of expressions to be evaluated
    ObjectConstruction(StructSignature, Vec<Rc<Expression>>, Position),                         // The construction of an Object, with the 'new' keyword
    MainCall(Vec<String>)                                                                       // Fake expression used to call the main function
}




impl Expression {
    /// Evaluate the expression in the given context (scope and program) and return its value
    pub unsafe fn evaluate(&self, scope: Rc<RefCell<Scope>>, program: *mut SlothProgram, for_assignment: bool) -> Result<Rc<RefCell<Value>>, Error> {

        let res = match self {
            // return this literal value
            Expression::Literal(v, _) => Ok(Rc::new(RefCell::new(v.clone()))),

            // a list
            Expression::ListInit(exprs, p) => {
                let mut values = Vec::new();
                let mut list_type = Type::Any;

                if exprs.len() != 0 {
                    values.push(propagate!(exprs[0].evaluate(scope.clone(), program, false), &self.get_pos()));
                    list_type = values[0].borrow().get_type();

                    // Add the other elements to the value list, but checking the type of the value first
                    for expr in exprs.iter().skip(1) {
                        let value = propagate!(expr.evaluate(scope.clone(), program, false), p);

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
                        let owner_ref = propagate!(o.evaluate(scope.clone(), program, false), p);
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
                        scope.borrow().get_variable(name.clone(), program.as_mut().unwrap())
                    }
                }
            },







            Expression::MainCall(arguments) => {
                // get the entry point
                let main_function = match program.as_ref().unwrap().get_main() {
                    Ok(f) => f,
                    Err(e) => return Err(Error::new(ErrMsg::NoEntryPoint(e), None))
                };

                // Check that the entry point output is of type num
                if main_function.get_output_type() != Type::Number {
                    let err_msg = format!("Your '{}' function must return a value of type '{}'", ENTRY_POINT_NAME, Type::Number);
                    return Err(Error::new(ErrMsg::ReturnValueError(err_msg), None));
                }


                // Try to parse each inputs (strings) into a value of the expected type
                let values: Vec<Rc<RefCell<Value>>> = match main_function.get_input_types() {
                    None => {
                        let err_msg = format!("The '{}' function has no defined input types", ENTRY_POINT_NAME);
                        return Err(Error::new(ErrMsg::RustError(err_msg), None));
                    },

                    Some(t) => {
                        if t.len() != arguments.len() {
                            let s = if t.len() > 1 {"s"} else {""};
                            let err_msg = format!("Expected {} argument{s}, but received {}.\nNote: Expected types: {}", t.len(), arguments.len(), format_list(t));
                            return Err(Error::new(ErrMsg::InvalidArguments(err_msg), None));
                        }

                        // generate the values
                        let mut values = Vec::new();

                        for (i, (given, expected_type)) in std::iter::zip(arguments, t).enumerate() {
                            match Value::string_to_value(given.clone(), expected_type) {
                                Ok(v) => values.push(Rc::new(RefCell::new(v))),
                                Err(e) => {
                                    let err_msg = format!("Invalid argument {}: {}", i, e);
                                    return Err(Error::new(ErrMsg::InvalidArguments(err_msg), None))
                                }
                            }
                        };

                        values
                    }
                };



                // The function is correct, proceed to run it
                Expression::execute_function(main_function, None, values, program)
            },



            
            Expression::FunctionCall(owner, signature, arguments, p) => {

                // Get the reference to each value. The inputs by value (without "~") are deep-cloned at a later step,
                // and are added to the function scope even after

                let inputs = arguments.iter().map(|e| e.evaluate(scope.clone(), program, false)).collect::<Result<Vec<Rc<RefCell<Value>>>, Error>>();
                let inputs = propagate!(inputs, p);

                // Get the reference to the owner value, if any
                let owner_value = match owner {
                    Some(s) => {
                        Some(propagate!(s.evaluate(scope.clone(), program, false), p))
                    },
                    None => None
                };

                
                // we can complete the signature with the input types and the owner type
                let mut signature = signature.clone();
                if let Some(v) = &owner_value {signature.owner_type = Some(v.borrow().get_type())}
                let input_types: Vec<Type> = inputs.iter().map(|i| i.borrow().get_type()).collect();
                signature.input_types = input_types;
                
                // get the function corresponding to the signature
                let function = match program.as_ref().unwrap().get_function(&signature) {
                    Ok(f) => f,
                    Err(e) => {
                        return Err(Error::new(ErrMsg::FunctionError(e), Some(p.clone())))
                    }
                };
                
                Expression::execute_function(function, owner_value, inputs, program)
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
                    given_values.push(propagate!(expr.evaluate(scope.clone(), program, false), p));
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
                let access_ref = propagate!(access.evaluate(scope.clone(), program, false), p);
                let access_str = access_ref.borrow().to_string();


                // create a new expression::variableaccess and evaluate it
                let expr = Expression::VariableAccess(Some(owner.clone()), access_str, p.clone());
                expr.evaluate(scope, program, for_assignment)
            },
        };


        match res {
            Ok(v) => Ok(v),
            Err(e) => {

                // don't add pos if we're in the main call
                if let Expression::MainCall(..) = self {Err(e)}
                else {Err(e.with(&self.get_pos()))}
            }
        }
    }






    unsafe fn execute_function(function: &Box<dyn SlothFunction>, owner_value: Option<Rc<RefCell<Value>>>, arguments: Vec<Rc<RefCell<Value>>>, program: *mut SlothProgram) -> Result<Rc<RefCell<Value>>, Error> {

        // Whether the arguments are passed by value or by reference
        let inputs_ref_or_cloned: Vec<bool> = match function.get_signature().input_types {
            Some(v) => v.iter().map(|(_, b)| *b).collect(),
            None => vec![true; arguments.len()]
        };


        // Create a new scope for the execution of the function
        let func_scope = Rc::new(RefCell::new(Scope::new()));


        // Create the input variable (@0, @1, etc.) with the default value
        for (i, value) in arguments.iter().enumerate() {
            let mut v = value.clone();

            // if the values are cloned, allocate a new Value instead of using the reference
            // TODO: Is it inverted ?
            if !inputs_ref_or_cloned[i] {
                let cloned_value = match value.borrow().to_owned().deep_clone() {
                    Ok(v) => v,
                    Err(e) => return Err(Error::new(ErrMsg::InvalidArguments(e), None))
                };

                v = cloned_value;
            }


            match func_scope.try_borrow_mut() {
                Ok(mut reference) => match (*reference).push_variable(format!("@{}", i), v) {
                    Ok(()) => (),
                    Err(e) => return Err(Error::new(ErrMsg::RuntimeError(e), None))
                },
                Err(e) => return Err(Error::new(ErrMsg::RustError(e.to_string()), None))
            };
        }

        // Create the @return variable, with default value, and the "@self" variable, containing a copy of the value stored in the variable
        {
            let default_value = function.get_output_type().default();
            match func_scope.try_borrow_mut() {
                Ok(mut reference) => {
                    match (*reference).push_variable("@return".to_string(), Rc::new(RefCell::new(default_value))) {
                        Ok(()) => (),
                        Err(e) => return Err(Error::new(ErrMsg::RuntimeError(e), None))
                    };

                    match owner_value {
                        Some(v) => {
                            match (*reference).push_variable("@self".to_string(), v.clone()) {
                                Ok(()) => (),
                                Err(e) => return Err(Error::new(ErrMsg::RuntimeError(e), None))
                            };
                        },

                        None => ()
                    };
                },
                Err(e) => return Err(Error::new(ErrMsg::RustError(e.to_string()), None))
            };
        }

        // run the method in the given scope
        function.call(func_scope.clone(), program.as_mut().unwrap())?;



        // return the value in the '@return' variable, but check its type first
        let res = match func_scope.borrow().get_variable("@return".to_string(), program.as_mut().unwrap()) {
            Ok(v) => {
                let brrw = v.borrow();
                if brrw.get_type() != function.get_output_type() {
                    let err_msg = format!("Function {} should return a value of type {}, but it returned {} which is of type {}", function.get_name(), function.get_output_type(), brrw.to_string(), brrw.get_type());
                    Err(Error::new(ErrMsg::ReturnValueError(err_msg), None))
                }
                else {Ok(v.clone())}
            },
            Err(e) => Err(e)
        };

        res
    }








    /// Return the position of the expression
    pub fn get_pos(&self) -> Position {
        match self {
            Expression::Literal(_, p) => p,
            Expression::ListInit(_, p) => p,
            Expression::VariableAccess(_, _, p) => p,
            Expression::FunctionCall(_, _, _, p) => p,
            Expression::ObjectConstruction(_, _, p) => p,
            Expression::BracketAccess(_, _, p) => p,
            Expression::MainCall(_) => unreachable!()
        }.clone() 
    }
}







fn format_list<T: Display>(v: Vec<T>) -> String {
    let mut s = String::new();
    for e in v {s += &format!("{e} ")}
    s.trim_end().to_string()
}