use std::rc::Rc;

use crate::lexer::{Token, TokenStream, Operator};
use crate::sloth::expression::Expression;
use crate::sloth::function::FunctionCallSignature;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;
use crate::sloth::types::Type;

use super::expression::parse_expression;



const ONE_OPERAND_OPERATORS: [Operator; 2] = [Operator::Inv, Operator::Len];




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

    // the signature is completed when evaluating the expression 
    let sign = FunctionCallSignature::new(
        None,
        op_func_name(&operator),
        None,
        vec![],
        Type::Any,
    );


    let func_call = {
        // use rhs if not the inverse operator (1 operand)
        if !ONE_OPERAND_OPERATORS.contains(&operator) {
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