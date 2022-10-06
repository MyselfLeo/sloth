use crate::lexer::{Token, TokenStream, Operator};
use crate::sloth::expression::Expression;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;



/// Parse an operation
fn parse_operation(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Expression, Error> {

    // The starting token must be an operator
    let (operator, first_pos) = match iterator.current() {
        Some((Token::Operator(s), p)) => (s, p),
        o => return Err(super::wrong_token(o, "operator"))
    };


    // determine the number of operands
    let mut nb_operands = 2;

    let operator = match operator.as_str() {
        // 2 elements operators
        "+" => Operator::Add,
        "-" => Operator::Sub,
        "*" => Operator::Mul,
        "/" => Operator::Div,
        "==" => Operator::Eq,
        ">" => Operator::Gr,
        "<" => Operator::Lw,
        ">=" => Operator::Ge,
        "<=" => Operator::Le,
        "?" => Operator::Or,
        "&" => Operator::And,

        // 1 element operators
        _ => {
            nb_operands = 1;
            match operator.as_str() {
                "!" => Operator::Inv,
                t => {
                    let err_msg = format!("Unimplemented operator {}", t);
                    return Err(Error::new(ErrMsg::OperationErrror(err_msg), Some(first_pos)))
                }
            }
        }
    };

    // get the first and potential second expression
    iterator.next();
    let (first_expr_id, mut last_pos) = parse_expression(iterator, program, warning)?;

    // Get second expression, if needed
    let second_expr_id = match nb_operands > 1 {
        true => {
            let (expr_id, pos) = parse_expression(iterator, program, warning)?;
            last_pos = pos;
            Some(expr_id)
        }, 
        false => None,
    };

    let op_pos = first_pos.until(last_pos);
    Ok(Expression::Operation(operator, Some(first_expr_id), second_expr_id, op_pos))
}