use std::cell::RefCell;
use std::rc::Rc;

use crate::errors::{Error, ErrorMessage};
use crate::tokenizer::ElementPosition;
use super::expression::ExpressionID;
use super::scope::Scope;
use super::program::SlothProgram;
use super::value::Value;

/// Statements are instructions that modify a scope (variable assignment for example)
#[derive(Clone, Debug)]
pub enum Statement {
    Assignment(IdentifierWrapper, ExpressionID, ElementPosition),       // Assignment of an expression evaluation to a variable
    ExpressionCall(ExpressionID, ElementPosition),                      // Evaluation of an expression, not storing it
    If(ExpressionID, Vec<Statement>, ElementPosition),                  // If block. Condition expr index and list of statements
    While(ExpressionID, Vec<Statement>, ElementPosition),               // while look. same specs as if
}





impl Statement {

    // Apply the statement to the given scope
    pub unsafe fn apply(&self, scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
        match self {
            Statement::Assignment(wrapper, expr_id, p) => {

                let expr = match program.get_expr(*expr_id) {
                    Ok(e) => e,
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };

                // The value assigned.
                // Using this RC directly would lead to 'weird' behavior, like:
                //      a = 3
                //      b = a
                //      a = 1
                //      =>   b == 1 (true)
                // so, we reallocate a copy of the inner value so it is not subject to changes
                let inner_value = expr.evaluate(scope.clone(), program)?.borrow().to_owned();
                let value = Rc::new(RefCell::new(inner_value));


                // This is a simple assignment. In that case, we need to check if the variable exists. If not,
                // we push it with the given value, then stops
                match wrapper.is_simple() {
                    Some(ident) => {
                        if let IdentifierElement::Identifier(n) = ident {
                            match scope.try_borrow_mut() {
                                Ok(mut brrw) => {
                                    match (*brrw).push_variable(n, value.clone()) {
                                        Ok(()) => return Ok(()),
                                        Err(_) => (),
                                    }
                                },
                                Err(e) => {return Err(Error::new(ErrorMessage::RustError(e.to_string()), Some(p.clone())))}
                            }

                            
                        }
                        else {panic!("IdentifierWrapper did not start with an IdentifierElement::Identifier")}
                    },
                    None => (),
                };


                match wrapper.get_value(scope, program) {
                    Ok(reference) => {

                        // Check that the type matches
                        {
                            let required_type = reference.borrow().get_type();
                            let given_type = value.borrow().get_type();

                            if required_type != given_type {
                                let err_msg = format!("Expected a Value of type '{}', got type '{}' instead", required_type, given_type);
                                return Err(Error::new(ErrorMessage::TypeError(err_msg), Some(p.clone())))
                            }

                            // Try to set the value
                            match reference.try_borrow_mut() {
                                Ok(mut borrow) => *borrow = value.borrow().to_owned(),
                                Err(e) => return Err(Error::new(ErrorMessage::RustError(e.to_string()), Some(p.clone())))
                            }

                            Ok(())
                        }

                    },
                    Err(e) => Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))
                }
            },

            Statement::ExpressionCall(expr_id, p) => {
                let expr = match program.get_expr(*expr_id) {
                    Ok(e) => e,
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };

                expr.evaluate(scope, program)?;

                Ok(())
            },

            Statement::If(cond_expr_id, statements, p) => {
                let expr = match program.get_expr(*cond_expr_id) {
                    Ok(e) => e,
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };

                let cond_value = expr.evaluate(scope.clone(), program)?;
                
                match cond_value.borrow().to_owned() {
                    Value::Boolean(true) => {
                        for statement in statements {statement.apply(scope.clone(), program)?}
                    }
                    Value::Boolean(false) => {},
                    _ => {return Err(Error::new(ErrorMessage::UnexpectedExpression("Expected boolean expression as 'if' condition".to_string()), Some(p.clone())))}
                }

                Ok(())
            },

            Statement::While(cond_expr_id, statements, p) => {
                let expr = match program.get_expr(*cond_expr_id) {
                    Ok(e) => e,
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };

                let mut loop_cond = expr.evaluate(scope.clone(), program)?.borrow().to_owned() == Value::Boolean(true);
                
                while loop_cond {
                    for statement in statements {statement.apply(scope.clone(), program)?}
                    loop_cond = expr.evaluate(scope.clone(), program)?.borrow().to_owned() == Value::Boolean(true);
                }

                Ok(())
            }
        }
    }


    pub fn get_pos(&self) -> ElementPosition {
        match self {
            Statement::Assignment(_, _, p) => p.clone(),
            Statement::ExpressionCall(_, p) => p.clone(),
            Statement::If(_, _, p) => p.clone(),
            Statement::While(_, _, p) => p.clone(),
        }
    }
}







#[derive(Clone, Debug)]
pub enum IdentifierElement {
    Identifier(String),         // name of the field
    Indexation(ExpressionID)    // index of the field, to be evaluated
}

impl IdentifierElement {
    pub fn get_field_str(&self, scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<String, String> {
        match self {
            IdentifierElement::Identifier(n) => {Ok(n.clone())},
            IdentifierElement::Indexation(e) => {
                let expr = program.get_expr(*e)?;
                unsafe {
                    let index_value = match expr.evaluate(scope, program) {
                        Ok(v) => v,
                        Err(e) => return Err(e.message.as_string())
                    };

                    let res = index_value.borrow().to_string();

                    Ok(res)
                }
            }
        }
    }
}



#[derive(Clone, Debug)]
/// Facilitate the access to values stored in other values from their name
pub struct IdentifierWrapper {
    ident_sequence: Vec<IdentifierElement>
}


impl IdentifierWrapper {

    pub fn new(ident_sequence: Vec<IdentifierElement>) -> IdentifierWrapper {
        IdentifierWrapper {ident_sequence}
    }


    /// Return a smart pointer to the value represented by the IdentifierWrapper
    pub fn get_value(&self, scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<Rc<RefCell<Value>>, String> {
        if self.ident_sequence.len() == 0 {panic!("IdentifierWrapper has a length of 0")}
        
        let mut value;

        // Get the first value
        if let IdentifierElement::Identifier(n) = &self.ident_sequence[0] {value = scope.borrow().get_variable(n.clone(), program)?;}
        else {panic!("IdentifierWrapper sequence starts with an indexation")}

        // Get the value of each ident element successively to get the final value
        for (i, ident) in self.ident_sequence.iter().enumerate() {
            if i == 0 {continue;}
            let new_value = value.borrow().get_field(&ident.get_field_str(scope.clone(), program)?)?;
            value = new_value;
        }

        Ok(value)
    }

    /// Simple = links to a variable in a scope, not an inner value of something
    pub fn is_simple(&self) -> Option<IdentifierElement> {
        match self.ident_sequence.len() == 1 {
            true => Some(self.ident_sequence[0].clone()),
            false => None
        }
    }
}