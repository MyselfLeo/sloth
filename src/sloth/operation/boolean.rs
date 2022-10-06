use std::cell::RefCell;
use std::rc::Rc;

use crate::sloth::value::Value;




pub fn and(lhs: Rc<RefCell<Value>>, rhs: Rc<RefCell<Value>>) -> Result<Value, String> {
    let rhs = rhs.borrow().to_owned();
    let lhs = lhs.borrow().to_owned();

    match (lhs, rhs) {
        (Value::Boolean(x), Value::Boolean(y)) => Ok(Value::Boolean(x && y)),
        _ => Err("Can't 'and' values other than booleans".to_string())
    }
}

pub fn or(lhs: Rc<RefCell<Value>>, rhs: Rc<RefCell<Value>>) -> Result<Value, String> {
    let rhs = rhs.borrow().to_owned();
    let lhs = lhs.borrow().to_owned();

    match (lhs, rhs) {
        (Value::Boolean(x), Value::Boolean(y)) => Ok(Value::Boolean(x || y)),
        _ => Err("Can't 'or' values other than booleans".to_string())
    }
}

pub fn inverse(lhs: Rc<RefCell<Value>>) -> Result<Value, String> {
    let lhs = lhs.borrow().to_owned();

    match lhs {
        Value::Boolean(x) => Ok(Value::Boolean(!x)),
        _ => Err("Expected boolean".to_string())
    }
}