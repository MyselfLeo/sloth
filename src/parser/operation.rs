use std::rc::Rc;

use crate::lexer::{Token, TokenStream, Operator};
use crate::sloth::expression::Expression;
use crate::sloth::function::FunctionSignature;
use crate::sloth::operation::Operation;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;

use super::expression::parse_expression;



/// Return the name of the function implementing this operator
fn op_func_name(operator: &Operator) -> String {
    format!("@{}", operator.get_name())
}




/// Parse an operation
pub fn parse_operation(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Expression, Error> {
    // The starting token must be an operator
    let (operator, first_pos) = match stream.current() {
        Some((Token::Operator(s), p)) => {
            stream.next();
            (s, p)
        },
        o => return Err(super::wrong_token(o, "operator"))
    };

    let lhs = Rc::new(parse_expression(stream, program, warning, None)?);
    let lhs_pos = lhs.get_pos();

    let func_call = {
        let sign = FunctionSignature::new(
            None,
            op_func_name(&operator),
            None,
            None,
            None,
        );

        // use rhs if not the inverse operator (1 operand)
        if operator != Operator::Inv {
            let rhs = Rc::new(parse_expression(stream, program, warning, None)?);
            let rhs_pos = rhs.get_pos();

            Expression::FunctionCall(None, sign, vec![lhs, rhs], first_pos.until(rhs_pos))
        }
        else {
            Expression::FunctionCall(None, sign, vec![lhs], first_pos.until(lhs_pos))
        }
    };

    Ok(func_call)
}