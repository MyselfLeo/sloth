use std::cell::RefCell;
use std::rc::Rc;

use crate::errors::Error;

use super::expression::Expression;
use super::scope::Scope;
use super::program::SlothProgram;
use super::value::Value;

mod addition;
mod substraction;
mod multiplication;
mod division;
mod comparaison;
mod boolean;


#[derive(Clone, Debug)]
pub enum Operation {
    Addition(Rc<Expression>, Rc<Expression>),
    Substraction(Rc<Expression>, Rc<Expression>),
    Multiplication(Rc<Expression>, Rc<Expression>),
    Division(Rc<Expression>, Rc<Expression>),
    Equal(Rc<Expression>, Rc<Expression>),
    Greater(Rc<Expression>, Rc<Expression>),
    Lower(Rc<Expression>, Rc<Expression>),
    GreaterEqual(Rc<Expression>, Rc<Expression>),
    LowerEqual(Rc<Expression>, Rc<Expression>),
    And(Rc<Expression>, Rc<Expression>),
    Or(Rc<Expression>, Rc<Expression>),
    Inverse(Rc<Expression>),
}



impl Operation {
    pub unsafe fn execute(&self, scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<Value, Error> {
        let res = match self {
            Operation::Addition(e1, e2) => {
                addition::add(
                    e1.evaluate(scope, program, false)?,
                    e2.evaluate(scope, program, false)?
                )
            },
            Operation::Substraction(e1, e2) => {
                substraction::sub(
                    e1.evaluate(scope, program, false)?,
                    e2.evaluate(scope, program, false)?
                )
            },
            Operation::Multiplication(e1, e2) => {
                multiplication::mul(
                    e1.evaluate(scope, program, false)?,
                    e2.evaluate(scope, program, false)?
                )
            },
            Operation::Division(e1, e2) => {
                division::div(
                    e1.evaluate(scope, program, false)?,
                    e2.evaluate(scope, program, false)?
                )
            },
            Operation::Equal(e1, e2) => {
                comparaison::equal(
                    e1.evaluate(scope, program, false)?,
                    e2.evaluate(scope, program, false)?
                )
            },
            Operation::Greater(e1, e2) => {
                comparaison::greater(
                    e1.evaluate(scope, program, false)?,
                    e2.evaluate(scope, program, false)?
                )
            },
            Operation::Lower(e1, e2) => {
                comparaison::lower(
                    e1.evaluate(scope, program, false)?,
                    e2.evaluate(scope, program, false)?
                )
            },
            Operation::GreaterEqual(e1, e2) => {
                comparaison::greater_equal(
                    e1.evaluate(scope, program, false)?,
                    e2.evaluate(scope, program, false)?
                )
            },
            Operation::LowerEqual(e1, e2) => {
                comparaison::lower_equal(
                    e1.evaluate(scope, program, false)?,
                    e2.evaluate(scope, program, false)?
                )
            },
            Operation::And(e1, e2) => {
                boolean::and(
                    e1.evaluate(scope, program, false)?,
                    e2.evaluate(scope, program, false)?
                )
            },
            Operation::Or(e1, e2) => {
                boolean::or(
                    e1.evaluate(scope, program, false)?,
                    e2.evaluate(scope, program, false)?
                )
            },
            Operation::Inverse(e) => {
                boolean::inverse(
                    e.evaluate(scope, program, false)?
                )
            },
        };

        match res {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(crate::errors::ErrMsg::OperationErrror(e), None))
        }
    }
}