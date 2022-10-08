use std::rc::Rc;

use crate::lexer::{Token, TokenStream, Separator};
use crate::sloth::expression::Expression;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;
use super::expression::parse_expression;


pub fn parse_list(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Expression, Error> {
    println!("parsing list");

    let (_, start_pos) = super::expect_token(stream, Token::Separator(Separator::OpenSquareBracket))?;

    // parse each expr until the ]
    let mut exprs = Vec::new();
    while !super::current_equal(stream, Token::Separator(Separator::CloseSquareBracket))? {
        exprs.push(Rc::new(parse_expression(stream, program, warning, None)?));
    }

    let (_, end_pos) = super::expect_token(stream, Token::Separator(Separator::CloseSquareBracket))?;

    Ok(Expression::ListInit(exprs, start_pos.until(end_pos)))
}
