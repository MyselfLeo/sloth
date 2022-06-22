use crate::errors::{Error, ErrorMessage};
use crate::tokenizer::ElementPosition;
use super::expression::ExpressionID;
use super::scope::Scope;
use super::program::SlothProgram;
use super::value::Value;

/// Statements are instructions that modify a scope (variable assignment for example)
#[derive(Clone, Debug)]
pub enum Statement {
    Assignment(String, ExpressionID, ElementPosition),      // Assignment of an expression evaluation to a variable
    ExpressionCall(ExpressionID, ElementPosition),          // Evaluation of an expression, not storing it
    If(ExpressionID, Vec<Statement>, ElementPosition),      // If block. Condition expr index and list of statements
    While(ExpressionID, Vec<Statement>, ElementPosition),   // while look. same specs as if
}





impl Statement {

    pub unsafe fn apply(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
        match self {
            Statement::Assignment(name, expr_id, p) => {
                let expr = match program.get_expr(*expr_id) {
                    Ok(e) => e,
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };

                let value = expr.evaluate(scope, program)?;
                scope.set_variable(name.clone(), value);

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

                let cond_value = expr.evaluate(scope, program)?;
                
                match cond_value {
                    Value::Boolean(true) => {
                        for statement in statements {statement.apply(scope, program)?}
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

                let mut loop_cond = expr.evaluate(scope, program)? == Value::Boolean(true);
                
                while loop_cond {
                    for statement in statements {statement.apply(scope, program)?}
                    loop_cond = expr.evaluate(scope, program)? == Value::Boolean(true);
                }

                Ok(())
            }
        }
    }
}