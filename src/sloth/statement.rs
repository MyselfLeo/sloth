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
    pub unsafe fn apply(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<(), Error> {
        match self {
            Statement::Assignment(wrapper, expr_id, p) => {
                let expr = match program.get_expr(*expr_id) {
                    Ok(e) => e,
                    Err(e) => {return Err(Error::new(ErrorMessage::RuntimeError(e), Some(p.clone())))}
                };

                let value = expr.evaluate(scope, program)?;
                
                match wrapper.set_value(value, scope, program) {
                    Ok(()) => Ok(()),
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







#[derive(Clone, Debug)]
pub enum IdentifierElement {
    Identifier(String),         // name of the field
    Indexation(ExpressionID)    // index of the field, to be evaluated
}

impl IdentifierElement {
    pub fn get_field_str(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<String, String> {
        match self {
            IdentifierElement::Identifier(n) => {Ok(n.clone())},
            IdentifierElement::Indexation(e) => {
                let expr = program.get_expr(*e)?;
                unsafe {
                    let index_value = match expr.evaluate(scope, program) {
                        Ok(v) => v,
                        Err(e) => return Err(e.message.as_string())
                    };

                    Ok(index_value.to_string())
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




    pub fn get_value(&self, scope: &mut Scope, program: &mut SlothProgram) -> Result<&mut Value, String> {
        if self.ident_sequence.len() == 0 {panic!("IdentifierWrapper has a length of 0")}
        
        let mut value;

        // Get the first value
        if let IdentifierElement::Identifier(n) = &self.ident_sequence[0] {value = scope.get_variable(n.clone(), program)?;}
        else {panic!("IdentifierWrapper sequence starts with an indexation")}

        // Get the value of each ident element successively to get the final value
        for (i, ident) in self.ident_sequence.iter().enumerate() {
            if i == 0 {continue;}
            value = value.get_field(&ident.get_field_str(scope, program)?)?;
        }

        Ok(value)
    }


    fn update_value_rec(parent_value: Value, changed_value: Value, sequence: &mut Vec<IdentifierElement>, scope: &mut Scope, program: &mut SlothProgram) -> Result<&mut Value, String> {
        if sequence.is_empty() {return Ok(&mut changed_value)}

        let mut parent_value = parent_value.clone();

        let child_field_name = sequence[0].get_field_str(scope, program)?;
        sequence.remove(0);

        let mut child_value = parent_value.get_field(&child_field_name)?; 
        child_value = Self::update_value_rec(child_value, changed_value, sequence, scope, program)?;
        
        parent_value.set_field(&child_field_name, child_value)?;

        Ok(&mut parent_value)
    }



    pub fn set_value(&self, value: Value, scope: &mut Scope, program: &mut SlothProgram) -> Result<(), String> {
        let parent_variable_name = match &self.ident_sequence[0] {
            IdentifierElement::Identifier(n) => n.clone(),
            IdentifierElement::Indexation(_) => {panic!("IdentifierWrapper sequence starts with an indexation")}
        };

        if self.ident_sequence.len() == 1 {
            scope.set_variable(parent_variable_name, value);
            return Ok(())
        }

        let first_value = scope.get_variable(parent_variable_name.clone(), program)?;

        let mut sequence = self.ident_sequence.clone();
        sequence.remove(0);
        
        let new_value = Self::update_value_rec(first_value, value, &mut sequence, scope, program)?;
        scope.set_variable(parent_variable_name, new_value);

        Ok(())
    }
}