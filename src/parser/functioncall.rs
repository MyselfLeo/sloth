use std::rc::Rc;

use crate::lexer::{Token, TokenStream, Separator};
use crate::sloth::expression::Expression;
use crate::sloth::function::FunctionSignature;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;

use super::expression::parse_expression;



pub fn parse_functioncall(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool, first_expr: Option<Rc<Expression>>) -> Result<Expression, Error> {
    // the user can specify a module
    let (module_name, temp_fpos) = match super::module_check(stream)? {
        Some((m, p)) => (Some(m), Some(p)),
        None => (None, None)
    };

    // function name
    let (func_name, first_pos) = match stream.current() {
        Some((Token::Identifier(n), n_pos)) => {
            let p = match temp_fpos {
                Some(p) => p,
                None => n_pos
            };
            (n, p)
        },
        o => return Err(super::wrong_token(o, "function"))
    };

    // open parenthesis
    super::expect_token(stream, Token::Separator(Separator::OpenParenthesis))?;

    // each arguments
    let mut arg_exprs: Vec<Rc<Expression>> = Vec::new();
    while !super::current_equal(stream, Token::Separator(Separator::CloseParenthesis))? {
        arg_exprs.push(Rc::new(parse_expression(stream, program, warning)?))
    }

    // closing of the arguments
    let (_, end_pos) = super::expect_token(stream, Token::Separator(Separator::CloseParenthesis))?;

    let functioncall_pos = first_pos.until(end_pos);
    let func_sign = FunctionSignature::new(module_name, func_name, None, None, None);
    
    Ok(Expression::FunctionCall(first_expr, func_sign, arg_exprs, functioncall_pos))
}