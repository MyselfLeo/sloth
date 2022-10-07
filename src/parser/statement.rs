use std::rc::Rc;

use crate::lexer::{Token, TokenStream, Keyword};
use crate::sloth::program::SlothProgram;
use crate::errors::Error;
use crate::sloth::statement::Statement;

use super::expression::parse_expression;
use super::flow_control::parse_if;
use super::flow_control::parse_while;



pub fn parse_statement(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Statement, Error> {
    let statement = match stream.current() {
        // expr call or assignment
        Some((Token::Identifier(_), _)) => {
            let target = parse_expression(stream, program, warning, None)?;

            // if a '=' follows, it's an assignment, else it's an expression call
            if super::current_equal(stream, Token::Keyword(Keyword::Equal))? {
                stream.next();
                let source = parse_expression(stream, program, warning, None)?;
                // return the assignment
                Statement::Assignment(Rc::new(target), Rc::new(source), target.get_pos().until(source.get_pos()))
            }
            else {
                Statement::ExpressionCall(Rc::new(target), target.get_pos())
            }
        },

        Some((Token::Keyword(Keyword::If), _)) => parse_if(stream, program, warning)?,
        Some((Token::Keyword(Keyword::While), _)) => parse_while(stream, program, warning)?,

        o => return Err(super::wrong_token(o, "statement"))
    };

    super::check_semicolon(stream, warning, &statement.get_pos())?;
    Ok(statement)
}