use std::rc::Rc;

use crate::lexer::{Token, TokenStream, Operator};
use crate::sloth::expression::Expression;
use crate::sloth::operation::Operation;
use crate::sloth::program::SlothProgram;
use crate::errors::Error;

use super::expression::parse_expression;


/// Parse an operation
pub fn parse_operation(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Expression, Error> {
    // The starting token must be an operator
    let (operator, first_pos) = match stream.current() {
        Some((Token::Operator(s), p)) => (s, p),
        o => return Err(super::wrong_token(o, "operator"))
    };

    let lhs = Rc::new(parse_expression(stream, program, warning, None)?);

    let (operation, pos) = {
        // use rhs if not the inverse operator (1 operand)
        if operator != Operator::Inv {
            let rhs = Rc::new(parse_expression(stream, program, warning, None)?);

            let operation = match operator {
                Operator::Add => Operation::Addition(lhs, rhs),
                Operator::Sub => Operation::Substraction(lhs, rhs),
                Operator::Mul => Operation::Multiplication(lhs, rhs),
                Operator::Div => Operation::Division(lhs, rhs),
                Operator::Eq => Operation::Equal(lhs, rhs),
                Operator::Gr => Operation::Greater(lhs, rhs),
                Operator::Lw => Operation::Lower(lhs, rhs),
                Operator::Ge => Operation::GreaterEqual(lhs, rhs),
                Operator::Le => Operation::LowerEqual(lhs, rhs),
                Operator::And => Operation::And(lhs, rhs),
                Operator::Or => Operation::Or(lhs, rhs),
                _ => unreachable!(),
            };
            (operation, first_pos.until(rhs.get_pos()))
        }
        else {
            (Operation::Inverse(lhs), first_pos.until(lhs.get_pos()))
        }
    };

    Ok(Expression::Operation(operation, pos))
}