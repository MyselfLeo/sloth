use std::rc::Rc;
use std::cell::RefCell;

use crate::errors::Error;

use super::expression::Expression;
use super::scope::Scope;
use super::program::SlothProgram;
use super::value::Value;

mod addition;



pub enum Operation {
    Addition(Expression, Expression),
    Substraction(Expression, Expression),
    Multiplication(Expression, Expression),
    Division(Expression, Expression),
    Equal(Expression, Expression),
    Greater(Expression, Expression),
    Lower(Expression, Expression),
    GreaterEqual(Expression, Expression),
    LowerEqual(Expression, Expression),
    And(Expression, Expression),
    Or(Expression, Expression),
    Inverse(Expression),
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
            Operation::Substraction(_, _) => todo!(),
            Operation::Multiplication(_, _) => todo!(),
            Operation::Division(_, _) => todo!(),
            Operation::Equal(_, _) => todo!(),
            Operation::Greater(_, _) => todo!(),
            Operation::Lower(_, _) => todo!(),
            Operation::GreaterEqual(_, _) => todo!(),
            Operation::LowerEqual(_, _) => todo!(),
            Operation::And(_, _) => todo!(),
            Operation::Or(_, _) => todo!(),
            Operation::Inverse(_) => todo!(),
        };

        match res {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(crate::errors::ErrMsg::OperationErrror(e), None))
        }
    }
}