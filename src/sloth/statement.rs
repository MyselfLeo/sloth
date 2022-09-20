use std::cell::RefCell;
use std::rc::Rc;

use crate::errors::{Error, ErrorMessage};
use crate::tokenizer::ElementPosition;
use super::expression::ExpressionID;
use super::scope::Scope;
use super::program::SlothProgram;
use super::value::{Value, DeepClone};

/// Statements are instructions that modify a scope (variable assignment for example)
#[derive(Clone, Debug)]
pub enum Statement {
    Assignment(ExpressionID, ExpressionID, ElementPosition),            // Assignment of an expression evaluation to a variable
    ExpressionCall(ExpressionID, ElementPosition),                      // Evaluation of an expression, not storing it
    If(ExpressionID, Vec<Statement>, ElementPosition),                  // If block. Condition expr index and list of statements
    While(ExpressionID, Vec<Statement>, ElementPosition),               // while look. same specs as if
}





impl Statement {

    // Apply the statement to the given scope
    pub unsafe fn apply(&self, scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
        match self {
            Statement::Assignment(target_id, source_id, p) => {

                // Get the reference to the source
                let source_expr = match program.get_expr(*source_id) {
                    Ok(e) => e,
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };
                let source_ref = source_expr.evaluate(scope.clone(), program)?;


                // Get the reference to the target
                let target_expr = match program.get_expr(*target_id) {
                    Ok(e) => e,
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };
                let target_ref = target_expr.evaluate(scope.clone(), program)?;

                // Compare the types, and if they match, assign the new value
                let source_type = source_ref.borrow().get_type();
                let target_type = target_ref.borrow().get_type();

                if source_type != target_type {
                    let err_msg = format!("Expected a Value of type '{}', got type '{}' instead", target_type, source_type);
                    return Err(Error::new(ErrorMessage::TypeError(err_msg), Some(p.clone())))
                }

                // Replace the value
                match target_ref.try_borrow_mut() {
                    Ok(mut borrow) => *borrow = source_ref.borrow().to_owned(),
                    Err(e) => return Err(Error::new(ErrorMessage::RustError(e.to_string()), Some(p.clone())))
                }

                Ok(())
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

                let cond_value = expr.evaluate(scope.clone(), program)?.borrow().to_owned();
                match cond_value {
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





/*

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

*/