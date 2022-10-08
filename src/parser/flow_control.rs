use std::rc::Rc;

use crate::lexer::{Token, TokenStream, Keyword, Separator};
use crate::sloth::program::SlothProgram;
use crate::errors::Error;
use crate::sloth::statement::Statement;

use super::expression::parse_expression;
use super::statement::parse_statement;




pub fn parse_if(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Statement, Error> {
    let (_, first_pos) = super::expect_token(stream, Token::Keyword(Keyword::If))?;
    let condition = parse_expression(stream, program, warning, None)?;
    super::expect_token(stream, Token::Separator(Separator::OpenBracket))?;

    // parse the succession of statements until a closed bracket is reached
    let mut statements: Vec<Statement> = Vec::new();
    while !super::current_equal(stream, Token::Separator(Separator::CloseBracket))? {
        statements.push(parse_statement(stream, program, warning)?)
    }

    let (_, last_pos) = super::expect_token(stream, Token::Separator(Separator::CloseBracket))?;
    Ok(Statement::If(Rc::new(condition), statements, first_pos.until(last_pos)))
}



pub fn parse_while(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Statement, Error> {
    let (_, first_pos) = super::expect_token(stream, Token::Keyword(Keyword::While))?;
    let condition = parse_expression(stream, program, warning, None)?;
    super::expect_token(stream, Token::Separator(Separator::OpenBracket))?;

    // parse the succession of statements until a closed bracket is reached
    let mut statements: Vec<Statement> = Vec::new();
    while !super::current_equal(stream, Token::Separator(Separator::CloseBracket))? {
        statements.push(parse_statement(stream, program, warning)?)
    }

    let (_, last_pos) = super::expect_token(stream, Token::Separator(Separator::CloseBracket))?;
    Ok(Statement::While(Rc::new(condition), statements, first_pos.until(last_pos)))
}
