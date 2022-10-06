use crate::lexer::{Token, TokenStream, Separator};
use crate::sloth::expression::Expression;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;
use super::expression::parse_expression;


pub fn parse_list(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Expression, Error> {
    let (_, start_pos) = super::expect_token(stream, Token::Separator(Separator::OpenSquareBracket))?;

    // parse each expr until the ]
    let mut exprs: Vec<Expression> = Vec::new();
    while !super::current_equal(stream, Token::Separator(Separator::CloseSquareBracket))? {
        exprs.push(parse_expression(stream, program, warning)?);
    }

    let (_, end_pos) = super::expect_token(stream, Token::Separator(Separator::CloseSquareBracket))?;

    todo!()
    // Change Expression enum to use Expression (or Rc<RefCell<Expression>>) instead of ExpressionID
    // Ok(Expression::ListInit(exprs, start_pos.until(end_pos)))
}
