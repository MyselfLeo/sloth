use crate::lexer::{Token, TokenStream};
use crate::sloth::program::SlothProgram;
use crate::errors::Error;
use crate::sloth::expression::Expression;



/// Parse a variable call
pub fn parse_variablecall(stream: &mut TokenStream, _: &mut SlothProgram, _: bool) -> Result<Expression, Error> {
    // Get the identifier
    let expr = match stream.current() {
        Some((Token::Identifier(n), p)) => Expression::VariableAccess(None, n, p),
        o => return Err(super::wrong_token(o, "variable name"))
    };
    stream.next();
    Ok(expr)
}