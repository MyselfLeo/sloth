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

    // Apply the statement to the given scope
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


    pub fn get_pos(&self) -> ElementPosition {
        match self {
            Statement::Assignment(_, _, p) => p.clone(),
            Statement::ExpressionCall(_, p) => p.clone(),
            Statement::If(_, _, p) => p.clone(),
            Statement::While(_, _, p) => p.clone(),
        }
    }
}









#[derive(Clone, Debug, PartialEq)]
pub enum IdentifierWrapperType {
    Value,
    Function
}



/// Facilitate the access to elements from their name
pub struct IdentifierWrapper {
    element_type: IdentifierWrapperType,
    ident_sequence: Vec<String>
}


impl IdentifierWrapper {

    pub fn new(element_type: IdentifierWrapperType, ident_sequence: Vec<String>) -> IdentifierWrapper {
        IdentifierWrapper {element_type, ident_sequence}
    }




    pub fn get_value(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<Value, String> {
        if self.ident_sequence.len() == 0 {panic!("IdentifierWrapper has a length of 0")}
        if self.element_type != IdentifierWrapperType::Value {panic!("Called get_value on an IdentifierWrapper that is not a Value")}

        // Get the value of each ident element successively to get the final value
        let mut value = scope.get_variable(self.ident_sequence[0].clone(), program)?;
        for (i, ident) in self.ident_sequence.iter().enumerate() {
            if i > 0 {value = value.get_field(ident)?;}
        }

        Ok(value)
    }


    fn update_value_rec(parent_value: Value, changed_value: Value, sequence: &mut Vec<String>) -> Result<Value, String> {
        if sequence.is_empty() {return Ok(changed_value)}

        let mut parent_value = parent_value.clone();

        let child_name = sequence[0].clone();
        sequence.remove(0);

        let mut child_value = parent_value.get_field(&child_name)?; 
        child_value = Self::update_value_rec(child_value, changed_value, sequence)?;
        
        parent_value.set_field(&child_name, child_value)?;

        Ok(parent_value)
    }


    pub fn set_value(&self, value: Value, scope: &mut Scope, program: &mut SlothProgram) -> Result<(), String> {
        if self.element_type != IdentifierWrapperType::Value {panic!("Called set_value on an IdentifierWrapper that is not a Value")}

        let parent_variable_name = self.ident_sequence[0].clone();
        let first_value = scope.get_variable(parent_variable_name.clone(), program)?;

        let mut sequence = self.ident_sequence.clone();
        
        scope.set_variable(parent_variable_name, Self::update_value_rec(first_value, value, &mut sequence)?);

        Ok(())
    }
}