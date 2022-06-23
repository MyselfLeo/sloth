use crate::errors::Error;
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


pub fn apply_op(op: &Operator, lhs: Option<Value>, rhs: Option<Value>) -> Result<Value, String> {
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



fn add(lhs: Option<Value>, rhs: Option<Value>) -> Result<Value, String> {
    if rhs.is_none() {return Err("Tried to add with no right hand side value".to_string())}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap_or(Value::Number(0.0));

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x + y)),
        (Value::String(x), Value::String(y)) => Ok(Value::String(x + &y)),
        (Value::String(x), Value::Number(y)) => Ok(Value::String(x + &y.to_string())),
        (Value::Number(x), Value::String(y)) => Ok(Value::String(x.to_string() + &y)),
        (Value::String(x), Value::Boolean(y)) => Ok(Value::String(x + &y.to_string())),
        (Value::Boolean(x), Value::String(y)) => Ok(Value::String(x.to_string() + &y)),

        /*
        // list operations
        (Value::List(l), y) => {
            let mut new = l.clone();
            
            // support for List + List
            if let Value::List(j) = y {
                let mut buff = j.clone();
                new.append(&mut buff);
            }
            else {new.push(y);}

            Value::List(new)
        },

        (x, Value::List(l)) => {
            let mut new = l.clone();
            new.insert(0, x);
            Value::List(new)
        },
         */



        (_, Value::Number(_)) => Err("Tried to add a non-number value to a number".to_string()),
        (Value::Number(_), _) => Err("Tried to add a number to a non-number value".to_string()),
        (_, _) => Err("Tried to add two non-number values".to_string())
    }
}

fn sub(lhs: Option<Value>, rhs: Option<Value>) -> Result<Value, String> {
    if rhs.is_none() {return Err("Tried to sub with no right hand side value".to_string())}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap_or(Value::Number(0.0));

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x - y)),
        (_, Value::Number(_)) => Err("Tried to sub a non-number value to a number".to_string()),
        (Value::Number(_), _) => Err("Tried to sub a number to a non-number value".to_string()),
        (_, _) => Err("Tried to sub two non-number values".to_string())
    }
}

fn mul(lhs: Option<Value>, rhs: Option<Value>) -> Result<Value, String> {
    if rhs.is_none() {return Err("Tried to mul with no right hand side value".to_string())}
    if lhs.is_none() {return Err("Tried to mul with no left hand side value".to_string())}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x * y)),
        (_, Value::Number(_)) => Err("Tried to mul a non-number value to a number".to_string()),
        (Value::Number(_), _) => Err("Tried to mul a number to a non-number value".to_string()),
        (_, _) => Err("Tried to mul two non-number values".to_string())
    }
}

fn div(lhs: Option<Value>, rhs: Option<Value>) -> Result<Value, String> {
    if rhs.is_none() {return Err("Tried to div with no right hand side value".to_string())}
    if lhs.is_none() {return Err("Tried to div with no left hand side value".to_string())}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x / y)),
        (_, Value::Number(_)) => Err("Tried to div a non-number value to a number".to_string()),
        (Value::Number(_), _) => Err("Tried to div a number to a non-number value".to_string()),
        (_, _) => Err("Tried to div two non-number values".to_string())
    }
}



fn equal(lhs: Option<Value>, rhs: Option<Value>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("Expected two values to compare".to_string())}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x == y)),
        (Value::Boolean(x), Value::Boolean(y)) => Ok(Value::Boolean(x == y)),
        _ =>Ok(Value::Boolean(false))
    }
}

fn greater(lhs: Option<Value>, rhs: Option<Value>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("Expected two values to compare".to_string())}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x > y)),
        (x, y) => Err(format!("Can't compare {} and {}", x, y).to_string())
    }
}

fn lower(lhs: Option<Value>, rhs: Option<Value>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("Expected two values to compare".to_string())}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x < y)),
        _ => Err("Can't compare Values other than numbers".to_string())
    }
}

fn greater_equal(lhs: Option<Value>, rhs: Option<Value>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("Expected two values to compare".to_string())}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x >= y)),
        (x, y) => Err(format!("Can't compare {} and {}", x, y).to_string())
    }
}

fn lower_equal(lhs: Option<Value>, rhs: Option<Value>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("Expected two values to compare".to_string())}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x <= y)),
        (x, y) => Err(format!("Can't compare {} and {}", x, y).to_string())
    }
}

fn and(lhs: Option<Value>, rhs: Option<Value>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("Expected two values to compare".to_string())}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Boolean(x), Value::Boolean(y)) => Ok(Value::Boolean(x && y)),
        _ => Err("Can't 'and' values other than booleans".to_string())
    }
}

fn or(lhs: Option<Value>, rhs: Option<Value>) -> Result<Value, String> {
    if rhs.is_none() || lhs.is_none() {return Err("Expected two values to compare".to_string())}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Boolean(x), Value::Boolean(y)) => Ok(Value::Boolean(x || y)),
        _ => Err("Can't 'or' values other than booleans".to_string())
    }
}

fn inverse(lhs: Option<Value>, rhs: Option<Value>) -> Result<Value, String> {
    if rhs.is_some() {return Err("Unexpected operand".to_string())}
    if lhs.is_none() {return Err("Expected operand".to_string())}

    let lhs = lhs.unwrap();

    match lhs {
        Value::Boolean(x) => Ok(Value::Boolean(!x)),
        _ => Err("Expected boolean".to_string())
    }
}