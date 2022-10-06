use std::rc::Rc;

use crate::lexer::{Token, TokenStream, Keyword, Separator};
use crate::sloth::expression::Expression;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;
use crate::sloth::structure::StructSignature;





pub fn parse_functioncall(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Expression, Error> {
    let mut module: Option<String> = None;
    let mut function_name= String::new();

    let start_pos;

    // Get the first identifier. It can either be a module name (followed by a colon) or the name of the function (followed by a '(')
    let current = iterator.current().clone();
    if let Some((Token::Identifier(s), pos)) = current {
        start_pos = pos;

        match iterator.peek(1) {
            Some((Token::Separator(Separator::OpenParenthesis), _)) => function_name = s,
            Some((Token::Separator(Separator::Colon), _)) => module = Some(s),
            Some((t, p)) => {
                let err_msg = format!("Expected '(' or ':', got unexpected token '{}'", t.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)))
            },
            _ => return Err(eof_error(line!()))
        };

    }
    else {panic!("Function 'parse_functioncall' called but token iterator is not on a function call.")}



    if module.is_some() {

        // Next token must be ":"
        match iterator.next() {
            Some((token, position)) => {
                if token != Token::Separator(Separator::Colon) {
                    let err_msg = format!("Expected ':', got unexpected token '{}'", token.original_string());
                    return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(position.clone())));
                }
            }
            None => return Err(eof_error(line!()))
        };


        // Next token is the function's name
        match iterator.next() {
            Some((Token::Identifier(s), _)) => function_name = s,
            Some((t, p)) => {
                let err_msg = format!("Expected function name, got unexpected token '{}'", t.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())));
            },
            None => return Err(eof_error(line!()))
        };
    }


    // Next token must be an open parenthesis
    match iterator.next() {
        Some((token, position)) => {
            if token != Token::Separator(Separator::OpenParenthesis) {
                let err_msg = format!("Expected '(', got unexpected token '{}'", token.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(position.clone())));
            }
        }
        None => return Err(eof_error(line!()))
    };
    iterator.next();

    // Now, we parse expressions until we reach a closed parenthesis
    let mut inputs_expr_id: Vec<ExpressionID> = Vec::new();

    while match iterator.current() {
        Some((Token::Separator(Separator::CloseParenthesis), _)) => false,
        Some(_) => true,
        None => return Err(eof_error(line!()))
    } {
        inputs_expr_id.push(parse_expression(iterator, program, warning)?.0);
    };


    // Next token must be ')'
    let last_pos;
    match iterator.current() {
        Some((token, position)) => {
            last_pos = position.clone();
            if token != Token::Separator(Separator::CloseParenthesis) {
                let err_msg = format!("Expected ')', got unexpected token '{}'", token.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(position.clone())));
            }
        }
        None => return Err(eof_error(line!()))
    };

    let functioncall_pos = start_pos.until(last_pos);

    let func_id = FunctionSignature::new(module, function_name, None, None, None);

    iterator.next();
    Ok(Expression::FunctionCall(None, func_id, inputs_expr_id, functioncall_pos))
}