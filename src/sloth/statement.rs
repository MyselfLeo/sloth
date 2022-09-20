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