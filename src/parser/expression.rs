use std::rc::Rc;

use crate::lexer::{Token, TokenStream, Keyword, Separator};
use crate::sloth::expression::Expression;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;

use super::bracket::parse_bracket_access;
use super::list::parse_list;
use super::literal::parse_literal;
use super::operation::parse_operation;
use super::object_construction::parse_object_construction;
use super::varcall::parse_variablecall;
use super::functioncall::parse_functioncall;



pub fn parse_expression(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool, first_expr: Option<Rc<Expression>>) -> Result<Expression, Error> {
    println!("parsing expression {:?}", stream.current());
    
    // guess expr type from first token
    let expr = match stream.current() {
        Some((Token::Literal(_), ..)) => parse_literal(stream, program, warning)?,
        Some((Token::Separator(Separator::OpenSquareBracket), _)) => {
            match first_expr {
                Some(e) => parse_bracket_access(stream, program, warning, e)?,
                None => parse_list(stream, program, warning)?
            }
        },
        Some((Token::Operator(_), ..)) => parse_operation(stream, program, warning)?,
        Some((Token::Keyword(Keyword::New), _)) => parse_object_construction(stream, program, warning)?,

        // Function call or field access, determined by the following token
        Some((Token::Identifier(_), _)) =>  {
            match stream.peek(1) {
                Some((Token::Separator(Separator::OpenParenthesis), _)) | Some((Token::Separator(Separator::Colon), _)) => parse_functioncall(stream, program, warning, first_expr)?,
                _ => parse_variablecall(stream, program, first_expr, warning)?,
            }
        },

        o => return Err(super::wrong_token(o, "expression"))
    };


    // check if the expression is finished here or not
    match stream.current() {
        Some((Token::Separator(Separator::Period), _)) => {
            stream.next();
            parse_expression(stream, program, warning, Some(Rc::new(expr)))
        },
        Some((Token::Separator(Separator::OpenSquareBracket), _)) => {
            parse_expression(stream, program, warning, Some(Rc::new(expr)))
        },
        _ => Ok(expr)
    }
}












/*

/// In the case of a MethodCall (expr.method()), this function parses the second part (after the period)
/// It is given the ExpressionID and Position of the first expression
fn parse_expr_suffix(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool, first_expr: Rc<Expression>, is_parenthesied: bool) -> Result<Expression, Error> {
    // name of the variable or function to use
    let (ident, ident_pos) = match stream.next() {
        Some((Token::Identifier(n), p)) => (n, p),
        Some((t, p)) => {
            let err_msg = format!("Expected identifier, got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)));
        },
        None => return Err(eof_error(line!()))
    };



    // Check whether the call is a method call or a field access
    let expr = match stream.peek(1) {
        // method call
        Some((Token::Separator(Separator::OpenParenthesis), _)) | Some((Token::Separator(Separator::Colon), _)) => {
            let function = parse_functioncall(stream, program, warning)?;
            // Transforms the FunctionCall expression given by the parse_functioncall function into a MethodCall
            if let Expression::FunctionCall(_, signature, input_exprs, pos) = function {
                let expr_pos = first_expr.1.until(pos);
                let method_call = Expression::FunctionCall(Some(first_expr.0), signature, input_exprs, expr_pos.clone());
                (program.push_expr(method_call), expr_pos)
            }
            else {panic!("Function 'parse_functioncall' did not return an Expression::Functioncall value")}
        },
        
        // Field
        Some((_, _)) => {
            stream.next();
            let expr_pos = first_expr.1.until(ident_pos);

            let field_access = Expression::VariableAccess(Some(first_expr.0), ident, expr_pos.clone());
            (program.push_expr(field_access), expr_pos)
        },

        None => return Err(eof_error(line!()))
    };



    // determines whether the expression if finished here or not.
    match stream.current() {
        Some((Token::Separator(Separator::CloseParenthesis), _)) => {
            if is_parenthesied {stream.next(); Ok(expr)}
            else {Ok(expr)}
        },
        Some((Token::Separator(Separator::OpenSquareBracket), _)) => {
            parse_bracket_access(stream, program, warning, expr, is_parenthesied)
        },
        Some((Token::Separator(Separator::Period), _)) => {
            parse_second_expr(stream, program, warning, expr, is_parenthesied)
        },
        Some((t, p)) => {
            if !is_parenthesied {Ok(expr)}
            else {
                let err_msg = format!("Expected ')', got unexpected token '{}'", t.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)));
            }
        },
        None => Err(eof_error(line!())),
    }



}

*/
