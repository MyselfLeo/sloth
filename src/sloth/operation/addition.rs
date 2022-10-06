use std::cell::RefCell;
use std::rc::Rc;

use crate::sloth::value::Value;




pub fn add(lhs: Rc<RefCell<Value>>, rhs: Rc<RefCell<Value>>) -> Result<Value, String> {
    let rhs = rhs.borrow().to_owned();
    let lhs = lhs.borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x + y)),
        (Value::String(x), Value::String(y)) => Ok(Value::String(x + &y)),
        (Value::String(x), Value::Number(y)) => Ok(Value::String(x.to_owned() + &y.to_string())),
        (Value::Number(x), Value::String(y)) => Ok(Value::String(x.to_string() + &y)),
        (Value::String(x), Value::Boolean(y)) => Ok(Value::String(x.to_owned() + &y.to_string())),
        (Value::Boolean(x), Value::String(y)) => Ok(Value::String(x.to_string() + &y)),

        // List merging
        (Value::List(t1, v1), Value::List(t2, v2)) => {
            if t1 != t2 {return Err(format!("Cannot merge a list of '{}' with a list of '{}'", t2, t1))}

            let mut n = v1.clone();
            let mut other = v2.clone();
            n.append(&mut other);
            Ok(Value::List(t1, n))
        },

        // Appending to a list
        (Value::List(t1, v1), v) => {
            let value_type = v.get_type();
            if t1 != value_type {return Err(format!("Cannot append '{}' to a list of '{}'", value_type, t1))}

            let mut n = v1.clone();
            n.push(Rc::new(RefCell::new(v)));
            Ok(Value::List(t1, n))
        },

        // Prepending to a list
        (v, Value::List(t1, v1)) => {
            let value_type = v.get_type();
            if t1 != value_type {return Err(format!("Cannot prepend '{}' to a list of '{}'", value_type, t1))}

            let mut n = v1.clone();
            n.insert(0, Rc::new(RefCell::new(v)));
            Ok(Value::List(t1, n))
        },

        (v1, v2) => Err(format!("Cannot add '{}' to '{}'", v2.get_type(), v1.get_type()))
    }
}