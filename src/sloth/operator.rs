use std::{cell::RefCell, rc::Rc, ops::Deref};

use super::value::Value;





#[derive(Clone, Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Eq,     // Equal
    Gr,     // Greater than
    Lw,     // Lower than
    Ge,     // Greater or Equal
    Le,     // Lower or Equal
    And,
    Or,
    Inv,    // Inverse of boolean
}


impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Eq => "==",
            Operator::Gr => ">",
            Operator::Lw => "<",
            Operator::Ge => ">=",
            Operator::Le => "<=",
            Operator::And => "&",
            Operator::Or => "?",
            Operator::Inv => "!",
        };

        write!(f, "{}", txt)
    }
}


pub fn apply_op(op: &Operator, lhs: Option<Rc<RefCell<Value>>>, rhs: Option<Rc<RefCell<Value>>>) -> Result<Value, String> {
    match op {
        Operator::Add => add(lhs, rhs),
        Operator::Sub => sub(lhs, rhs),
        Operator::Mul => mul(lhs, rhs),
        Operator::Div => div(lhs, rhs),
        Operator::Eq => equal(lhs, rhs),
        Operator::Gr => greater(lhs, rhs),
        Operator::Ge => greater_equal(lhs, rhs),
        Operator::Lw => lower(lhs, rhs),
        Operator::Le => lower_equal(lhs, rhs),
        Operator::And => and(lhs, rhs),
        Operator::Or => or(lhs, rhs),
        Operator::Inv => inverse(lhs, rhs),
    }
}



fn add(lhs: Option<Rc<RefCell<Value>>>, rhs: Option<Rc<RefCell<Value>>>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("The addition requires 2 operands".to_string())}

    let rhs = rhs.unwrap().borrow().to_owned();
    let lhs = lhs.unwrap().borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x + y)),
        (Value::String(x), Value::String(y)) => Ok(Value::String(x + &y)),
        (Value::String(x), Value::Number(y)) => Ok(Value::String(x.to_owned() + &y.to_string())),
        (Value::Number(x), Value::String(y)) => Ok(Value::String(x.to_string() + &y)),
        (Value::String(x), Value::Boolean(y)) => Ok(Value::String(x.to_owned() + &y.to_string())),
        (Value::Boolean(x), Value::String(y)) => Ok(Value::String(x.to_string() + &y)),

        // List merging
        (Value::List(t1, v1), Value::List(t2, v2)) => {
            if t1 != t2 {return Err(format!("Tried to merge a list of type {} to a list of type {}", t2, t1))}

            let mut n = v1.clone();
            let mut other = v2.clone();
            n.append(&mut other);
            Ok(Value::List(t1, n))
        },


        // Appending to a list
        (Value::List(t1, v1), v) => {
            let value_type = v.get_type();
            if t1 != value_type {return Err(format!("Tried to append a value of type {} to a list of type {}", value_type, t1))}

            let mut n = v1.clone();
            n.push(Rc::new(RefCell::new(v)));
            Ok(Value::List(t1, n))
        },


        // Prepending to a list
        (v, Value::List(t1, v1)) => {
            let value_type = v.get_type();
            if t1 != value_type {return Err(format!("Tried to prepend a value of type {} to a list of type {}", value_type, t1))}

            let mut n = v1.clone();
            n.insert(0, Rc::new(RefCell::new(v)));
            Ok(Value::List(t1, n))
        },



        (_, Value::Number(_)) => Err("Tried to add a non-number value to a number".to_string()),
        (Value::Number(_), _) => Err("Tried to add a number to a non-number value".to_string()),
        (_, _) => Err("Tried to add two non-number values".to_string())
    }
}

fn sub(lhs: Option<Rc<RefCell<Value>>>, rhs: Option<Rc<RefCell<Value>>>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("The substraction requires 2 operands".to_string())}

    let rhs = rhs.unwrap().borrow().to_owned();
    let lhs = lhs.unwrap().borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x - y)),
        (_, Value::Number(_)) => Err("Tried to sub a non-number value to a number".to_string()),
        (Value::Number(_), _) => Err("Tried to sub a number to a non-number value".to_string()),
        (_, _) => Err("Tried to sub two non-number values".to_string())
    }
}

fn mul(lhs: Option<Rc<RefCell<Value>>>, rhs: Option<Rc<RefCell<Value>>>) -> Result<Value, String> {
    if rhs.is_none() {return Err("Tried to mul with no right hand side value".to_string())}
    if lhs.is_none() {return Err("Tried to mul with no left hand side value".to_string())}

    let rhs = rhs.unwrap().borrow().to_owned();
    let lhs = lhs.unwrap().borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x * y)),
        (_, Value::Number(_)) => Err("Tried to mul a non-number value to a number".to_string()),
        (Value::Number(_), _) => Err("Tried to mul a number to a non-number value".to_string()),
        (_, _) => Err("Tried to mul two non-number values".to_string())
    }
}

fn div(lhs: Option<Rc<RefCell<Value>>>, rhs: Option<Rc<RefCell<Value>>>) -> Result<Value, String> {
    if rhs.is_none() {return Err("Tried to div with no right hand side value".to_string())}
    if lhs.is_none() {return Err("Tried to div with no left hand side value".to_string())}

    let rhs = rhs.unwrap().borrow().to_owned();
    let lhs = lhs.unwrap().borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x / y)),
        (_, Value::Number(_)) => Err("Tried to div a non-number value to a number".to_string()),
        (Value::Number(_), _) => Err("Tried to div a number to a non-number value".to_string()),
        (_, _) => Err("Tried to div two non-number values".to_string())
    }
}



fn equal(lhs: Option<Rc<RefCell<Value>>>, rhs: Option<Rc<RefCell<Value>>>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("Expected two values to compare".to_string())}

    let rhs = rhs.unwrap().borrow().to_owned();
    let lhs = lhs.unwrap().borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x == y)),
        (Value::Boolean(x), Value::Boolean(y)) => Ok(Value::Boolean(x == y)),
        _ =>Ok(Value::Boolean(false))
    }
}

fn greater(lhs: Option<Rc<RefCell<Value>>>, rhs: Option<Rc<RefCell<Value>>>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("Expected two values to compare".to_string())}

    let rhs = rhs.unwrap().borrow().to_owned();
    let lhs = lhs.unwrap().borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x > y)),
        (x, y) => Err(format!("Can't compare {} and {}", x, y).to_string())
    }
}

fn lower(lhs: Option<Rc<RefCell<Value>>>, rhs: Option<Rc<RefCell<Value>>>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("Expected two values to compare".to_string())}

    let rhs = rhs.unwrap().borrow().to_owned();
    let lhs = lhs.unwrap().borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x < y)),
        _ => Err("Can't compare Values other than numbers".to_string())
    }
}

fn greater_equal(lhs: Option<Rc<RefCell<Value>>>, rhs: Option<Rc<RefCell<Value>>>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("Expected two values to compare".to_string())}

    let rhs = rhs.unwrap().borrow().to_owned();
    let lhs = lhs.unwrap().borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x >= y)),
        (x, y) => Err(format!("Can't compare {} and {}", x, y).to_string())
    }
}

fn lower_equal(lhs: Option<Rc<RefCell<Value>>>, rhs: Option<Rc<RefCell<Value>>>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("Expected two values to compare".to_string())}

    let rhs = rhs.unwrap().borrow().to_owned();
    let lhs = lhs.unwrap().borrow().to_owned();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x <= y)),
        (x, y) => Err(format!("Can't compare {} and {}", x, y).to_string())
    }
}

fn and(lhs: Option<Rc<RefCell<Value>>>, rhs: Option<Rc<RefCell<Value>>>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("Expected two values to compare".to_string())}

    let rhs = rhs.unwrap().borrow().to_owned();
    let lhs = lhs.unwrap().borrow().to_owned();

    match (lhs, rhs) {
        (Value::Boolean(x), Value::Boolean(y)) => Ok(Value::Boolean(x && y)),
        _ => Err("Can't 'and' values other than booleans".to_string())
    }
}

fn or(lhs: Option<Rc<RefCell<Value>>>, rhs: Option<Rc<RefCell<Value>>>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("Expected two values to compare".to_string())}

    let rhs = rhs.unwrap().borrow().to_owned();
    let lhs = lhs.unwrap().borrow().to_owned();

    match (lhs, rhs) {
        (Value::Boolean(x), Value::Boolean(y)) => Ok(Value::Boolean(x || y)),
        _ => Err("Can't 'or' values other than booleans".to_string())
    }
}

fn inverse(lhs: Option<Rc<RefCell<Value>>>, rhs: Option<Rc<RefCell<Value>>>) -> Result<Value, String> {
    if rhs.is_some() {return Err("Unexpected operand".to_string())}
    if lhs.is_none() {return Err("Expected operand".to_string())}

    let lhs = lhs.unwrap().borrow().to_owned();

    match lhs {
        Value::Boolean(x) => Ok(Value::Boolean(!x)),
        _ => Err("Expected boolean".to_string())
    }
}