use std::cell::RefCell;
use std::rc::Rc;

use crate::errors::{Error, ErrMsg};
use crate::position::Position;
use crate::propagate;
use super::expression::Expression;
use super::scope::Scope;
use super::program::SlothProgram;
use super::value::Value;

/// Statements are instructions that modify a scope (variable assignment for example)
#[derive(Clone, Debug)]
pub enum Statement {
    Assignment(Rc<Expression>, Rc<Expression>, Position),            // Assignment of an expression evaluation to a variable
    ExpressionCall(Rc<Expression>, Position),                      // Evaluation of an expression, not storing it
    If(Rc<Expression>, Vec<Statement>, Position),                  // If block. Condition expr index and list of statements
    While(Rc<Expression>, Vec<Statement>, Position),               // while look. same specs as if
}





impl Statement {

    // Apply the statement to the given scope
    pub unsafe fn apply(&self, scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
        match self {
            Statement::Assignment(target, source, p) => {
                // Get the reference to the source and target
                let source_ref = source.evaluate(scope.clone(), program, false)?;
                let target_ref = target.evaluate(scope.clone(), program, true)?;

                // Compare the types, and if they match, assign the new value
                let source_type = source_ref.borrow().get_type();
                let target_type = target_ref.borrow().get_type();

                if source_type != target_type {
                    let err_msg = format!("Expected a Value of type '{}', got type '{}' instead", target_type, source_type);
                    return Err(Error::new(ErrMsg::TypeError(err_msg), Some(p.clone())))
                }

                // Replace the value
                match target_ref.try_borrow_mut() {
                    Ok(mut borrow) => *borrow = source_ref.borrow().to_owned(),
                    Err(e) => return Err(Error::new(ErrMsg::RustError(e.to_string()), Some(p.clone())))
                }

                Ok(())
            },

            Statement::ExpressionCall(expr, p) => {
                propagate!(expr.evaluate(scope, program, false), p);
                Ok(())
            },

            Statement::If(cond, statements, p) => {
                let cond_value = cond.evaluate(scope.clone(), program, false)?.borrow().to_owned();
                match cond_value {
                    Value::Boolean(true) => {
                        for statement in statements {statement.apply(scope.clone(), program)?}
                    }
                    Value::Boolean(false) => {},
                    _ => {return Err(Error::new(ErrMsg::UnexpectedExpression("Expected boolean expression as 'if' condition".to_string()), Some(p.clone())))}
                }

                Ok(())
            },

            Statement::While(cond, statements, _) => {
                let mut loop_cond = cond.evaluate(scope.clone(), program, false)?.borrow().to_owned() == Value::Boolean(true);
                
                while loop_cond {
                    for statement in statements {statement.apply(scope.clone(), program)?}
                    loop_cond = cond.evaluate(scope.clone(), program, false)?.borrow().to_owned() == Value::Boolean(true);
                }

                Ok(())
            }
        }
    }


    pub fn get_pos(&self) -> Position {
        match self {
            Statement::Assignment(_, _, p) => p.clone(),
            Statement::ExpressionCall(_, p) => p.clone(),
            Statement::If(_, _, p) => p.clone(),
            Statement::While(_, _, p) => p.clone(),
        }
    }
}