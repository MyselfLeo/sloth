
use std::cell::RefCell;
use std::rc::Rc;

use crate::sloth::value::Value;




pub fn equal(lhs: Rc<RefCell<Value>>, rhs: Rc<RefCell<Value>>) -> Result<Value, String> {
    let rhs = rhs.borrow().to_owned();
    let lhs = lhs.borrow().to_owned();
    Ok(Value::Boolean(lhs == rhs))
}

pub fn greater(lhs: Rc<RefCell<Value>>, rhs: Rc<RefCell<Value>>) -> Result<Value, String> {
    let rhs = rhs.borrow().to_owned();
    let lhs = lhs.borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x > y)),
        (x, y) => Err(format!("Can't compare {} with {}", x, y))
    }
}

pub fn lower(lhs: Rc<RefCell<Value>>, rhs: Rc<RefCell<Value>>) -> Result<Value, String> {
    let rhs = rhs.borrow().to_owned();
    let lhs = lhs.borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x < y)),
        (x, y) => Err(format!("Can't compare {} with {}", x, y))
    }
}

pub fn greater_equal(lhs: Rc<RefCell<Value>>, rhs: Rc<RefCell<Value>>) -> Result<Value, String> {
    let rhs = rhs.borrow().to_owned();
    let lhs = lhs.borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x >= y)),
        (x, y) => Err(format!("Can't compare {} with {}", x, y))
    }
}

pub fn lower_equal(lhs: Rc<RefCell<Value>>, rhs: Rc<RefCell<Value>>) -> Result<Value, String> {
    let rhs = rhs.borrow().to_owned();
    let lhs = lhs.borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x <= y)),
        (x, y) => Err(format!("Can't compare {} with {}", x, y))
    }
}