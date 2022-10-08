use std::rc::Rc;

use crate::lexer::{Token, TokenStream, Separator};
use crate::sloth::expression::Expression;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;

use super::expression::parse_expression;




/// expression[access]
pub fn parse_bracket_access(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool, first_expr: Rc<Expression>) -> Result<Expression, Error> {

    println!("parsing bracket access");

    // opening [
    let (_, first_pos) = super::expect_token(stream, Token::Separator(Separator::OpenSquareBracket))?;
    // field identifier
    let access = parse_expression(stream, program, warning, None)?;
    // closing ]
    let (_, end_pos) = super::expect_token(stream, Token::Separator(Separator::CloseSquareBracket))?;

    Ok(Expression::BracketAccess(first_expr, Rc::new(access), first_pos.until(end_pos)))
}