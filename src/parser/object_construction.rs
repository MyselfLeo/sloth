use std::rc::Rc;

use crate::lexer::{Token, TokenStream, Keyword, Separator};
use crate::position::Position;
use crate::sloth::expression::Expression;
use crate::sloth::program::SlothProgram;
use crate::sloth::statement::Statement;
use crate::errors::Error;
use crate::sloth::structure::StructSignature;

use super::expression::parse_expression;




pub fn parse_object_construction(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Expression, Error> {
    let (_, first_pos) = super::expect_token(stream, Token::Keyword(Keyword::New))?;

    // if there is a ':' then the current token is the module name
    let module_name = {
        if let Some((Token::Separator(Separator::Colon), _)) = stream.peek(1) {
            let res = match stream.current() {
                Some((Token::Identifier(n), _)) => Some(n),
                o => return Err(super::wrong_token(o, "module")),
            };
            // go over the colon, set the stream to the structure name
            stream.skip(2);

            res
        }
        else {None}
    };

    let struct_name = match stream.current() {
        Some((Token::Identifier(n), p)) => n,
        o => return Err(super::wrong_token(o, "structure")),
    };

    // opening of the arguments
    super::expect_token(stream, Token::Separator(Separator::OpenParenthesis))?;

    let mut exprs = Vec::new();
    // Next is a sequence of expressions, until a closed parenthesis is met
    while !super::current_equal(stream, Token::Separator(Separator::OpenParenthesis))? {
        exprs.push(parse_expression(stream, program, warning)?);
    }

    // closing of the arguments
    let (_, end_pos) = super::expect_token(stream, Token::Separator(Separator::CloseParenthesis))?;

    let signature = StructSignature::new(module_name, struct_name);

    Ok(Expression::ObjectConstruction(signature, exprs, first_pos.until(end_pos)))
}