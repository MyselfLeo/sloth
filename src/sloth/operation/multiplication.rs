use std::cell::RefCell;
use std::rc::Rc;

use crate::sloth::value::Value;


pub fn mul(lhs: Rc<RefCell<Value>>, rhs: Rc<RefCell<Value>>) -> Result<Value, String> {
    let rhs = rhs.borrow().to_owned();
    let lhs = lhs.borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x * y)),
        (v1, v2) => Err(format!("Cannot multiply '{}' with '{}'", v1.get_type(), v2.get_type()))
    }
}