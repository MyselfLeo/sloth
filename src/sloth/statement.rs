use super::expression::ExpressionID;
use super::scope::Scope;
use super::program::SlothProgram;
use super::value::Value;

/// Statements are instructions that modify a scope (variable assignment for example)
#[derive(Clone, Debug)]
pub enum Statement {
    Assignment(String, ExpressionID),      // Assignment of an expression evaluation to a variable
    ExpressionCall(ExpressionID),          // Evaluation of an expression, not storing it
    Block(Vec<Statement>),                 // a list of statements
    If(ExpressionID, Vec<Statement>),      // If block. Condition expr index and list of statements
    While(ExpressionID, Vec<Statement>),   // while look. same specs as if
}





impl Statement {

    pub fn apply(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<(), String> {
        match self {
            Statement::Assignment(name, expr_id) => {
                let expr = program.get_expr(*expr_id)?;

                let value = expr.evaluate(scope, program)?;
                scope.set_variable(name.clone(), value);

                Ok(())
            },

            Statement::ExpressionCall(expr_id) => {
                program.get_expr(*expr_id)?.evaluate(scope, program)?;

                Ok(())
            },

            Statement::Block(statements) => {
                for statement in statements {
                    statement.apply(scope, program)?
                }

                Ok(())
            },

            Statement::If(cond_expr_id, statements) => {
                let cond_value = program.get_expr(*cond_expr_id)?.evaluate(scope, program)?;
                
                match cond_value {
                    Value::Boolean(true) => {
                        for statement in statements {statement.apply(scope, program)?}
                    }
                    Value::Boolean(false) => {},
                    _ => {return Err("Expected boolean expression as 'if' condition".to_string())}
                }

                Ok(())
            },

            Statement::While(cond_expr_id, statements) => {
                let expr = program.get_expr(*cond_expr_id)?;

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