use crate::lexer::{Token, TokenStream};
use crate::sloth::expression::Expression;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;
use crate::sloth::value::Value;


pub fn parse_literal(stream: &mut TokenStream, _: &mut SlothProgram, _: bool) -> Result<Expression, Error> {
    println!("parsing literal");

    let (literal_str, pos) = match stream.current() {
        Some((Token::Literal(s), p)) => (s, p),
        o => return Err(super::wrong_token(o, "literal"))
    };

    stream.next();
    Ok(Expression::Literal(Value::from_raw_token(literal_str), pos))
}