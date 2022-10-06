use crate::lexer::{Token, TokenStream, Keyword, Separator};
use crate::sloth::expression::Expression;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;

use super::list::parse_list;
use super::literal::parse_literal;



pub fn parse_expression(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Expression, Error> {
    // If the first token is an open parenthesis, we expect the expression to end on a closed parenthesis.
    let is_parenthesied = super::current_equal(stream, Token::Separator(Separator::OpenParenthesis))?;
    
    // guess expr type from first token
    let expr = match stream.next() {
        Some((Token::Literal(_), ..)) => parse_literal(stream, program, warning)?,
        Some((Token::Separator(Separator::OpenSquareBracket), _)) => parse_list(stream, program, warning)?,

        /*
        // The token is an identifier. Check the next token to see if its a function call, or field access
        Some((Token::Identifier(_), _)) =>  {
            match stream.peek(1) {
                Some((Token::Separator(Separator::OpenParenthesis), _)) | Some((Token::Separator(Separator::Colon), _)) => {
                    let func_call = parse_functioncall(stream, program, warning)?;
                    if let Expression::FunctionCall(_, _, _, p) = func_call.clone() {(func_call, p)}
                    else {panic!("parse_functioncall did not return an Expression::FunctionCall enum")}
                },
                _ => {
                    let var_call = parse_variablecall(stream, program, warning)?;
                    if let Expression::VariableAccess(_, _, p) = var_call.clone() {(var_call, p)}
                    else {panic!("pare_variablecall did not return an Expression::VariableAccess")}
                }
            }
        },
        */

        // The token is an operator, so it's an operation
        Some((Token::Operator(_), _)) => {
            let operation = parse_operation(stream, program, warning)?;
            if let Expression::Operation(_, _, _, p) = operation.clone() {(operation, p)}
            else {panic!("parse_operation did not return an Expression::Operation enum")}
        },



        // The token is the "new" keyword: it's the construction of a struct
        Some((Token::Keyword(Keyword::New), _)) => {
            parse_object_construction(stream, program, warning)?
        },




        Some((t, p)) => {
            let err_msg = format!("Unexpected expression start '{}'", t.original_string());
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)))
        }

        None => return Err(eof_error(line!()))
    };

    let first_expr = (program.push_expr(expr.clone()), expr_pos);




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
}