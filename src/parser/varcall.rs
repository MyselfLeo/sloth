use std::rc::Rc;

use crate::lexer::{Token, TokenStream};
use crate::sloth::program::SlothProgram;
use crate::errors::Error;
use crate::sloth::expression::Expression;



/// Parse a variable call
pub fn parse_variablecall(stream: &mut TokenStream, _: &mut SlothProgram, first_expr: Option<Rc<Expression>>, _: bool) -> Result<Expression, Error> {
    println!("parsing varcall");

    // Get the identifier
    let expr = match stream.current() {
        Some((Token::Identifier(n), p)) => Expression::VariableAccess(first_expr, n, p),
        o => return Err(super::wrong_token(o, "variable name"))
    };
    stream.next();
    Ok(expr)
}