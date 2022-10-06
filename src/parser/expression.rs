use crate::lexer::{Token, TokenStream, Keyword, Separator};
use crate::sloth::expression::Expression;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;

use super::list::parse_list;
use super::literal::parse_literal;
use super::operation::parse_operation;
use super::object_construction::parse_object_construction;
use super::varcall::parse_variablecall;



pub fn parse_expression(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Expression, Error> {
    // If the first token is an open parenthesis, we expect the expression to end on a closed parenthesis.
    let is_parenthesied = super::current_equal(stream, Token::Separator(Separator::OpenParenthesis))?;
    
    // guess expr type from first token
    let expr = match stream.next() {
        Some((Token::Literal(_), ..)) => parse_literal(stream, program, warning)?,
        Some((Token::Separator(Separator::OpenSquareBracket), _)) => parse_list(stream, program, warning)?,
        Some((Token::Operator(_), ..)) => parse_operation(stream, program, warning)?,
        Some((Token::Keyword(Keyword::New), _)) => parse_object_construction(stream, program, warning)?,

        // Function call or field access, determined by the following token
        Some((Token::Identifier(_), _)) =>  {
            match stream.peek(1) {
                Some((Token::Separator(Separator::OpenParenthesis), _)) | Some((Token::Separator(Separator::Colon), _)) => parse_functioncall(stream, program, warning)?,
                _ => parse_variablecall(stream, program, warning)?,
            }
        },

        o => return Err(super::wrong_token(o, "expression"))
    };

    


    /*
    // determines whether the expression if finished here or not.
    match stream.current() {
        Some((Token::Separator(Separator::CloseParenthesis), _)) => {
            if is_parenthesied {stream.next();}
            
            if let Some((Token::Separator(Separator::Period), _)) = stream.current() {
                parse_second_expr(stream, program, warning, first_expr, false)
            }
            else {Ok(first_expr)}
        },
        Some((Token::Separator(Separator::Period), _)) => {
            parse_second_expr(stream, program, warning, first_expr, is_parenthesied)
        },
        Some((Token::Separator(Separator::OpenSquareBracket), _)) => {
            parse_bracket_access(stream, program, warning, first_expr, is_parenthesied)
        },
        Some((t, p)) => {
            if !is_parenthesied {Ok(first_expr)}
            else {
                let err_msg = format!("Expected ')', got unexpected token '{}'", t.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)));
            }
        },
        None => Err(eof_error(line!())),
    }
    */
}