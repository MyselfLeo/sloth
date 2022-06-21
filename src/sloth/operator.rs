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


pub fn apply_op(op: &Operator, lhs: Option<Value>, rhs: Option<Value>) -> Value {
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



fn add(lhs: Option<Value>, rhs: Option<Value>) -> Value {
    if rhs.is_none() {panic!("Tried to add with no right hand side value")}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap_or(Value::Number(0.0));

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
        (Value::String(x), Value::String(y)) => Value::String(x + &y),
        (Value::String(x), Value::Number(y)) => Value::String(x + &y.to_string()),
        (Value::Number(x), Value::String(y)) => Value::String(x.to_string() + &y),
        (Value::String(x), Value::Boolean(y)) => Value::String(x + &y.to_string()),
        (Value::Boolean(x), Value::String(y)) => Value::String(x.to_string() + &y),

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



        (_, Value::Number(_)) => panic!("Tried to add a non-number value to a number"),
        (Value::Number(_), _) => panic!("Tried to add a number to a non-number value"),
        (_, _) => panic!("Tried to add two non-number values")
    }
}

fn sub(lhs: Option<Value>, rhs: Option<Value>) -> Value {
    if rhs.is_none() {panic!("Tried to sub with no right hand side value")}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap_or(Value::Number(0.0));

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Value::Number(x - y),
        (_, Value::Number(_)) => panic!("Tried to sub a non-number value to a number"),
        (Value::Number(_), _) => panic!("Tried to sub a number to a non-number value"),
        (_, _) => panic!("Tried to sub two non-number values")
    }
}

fn mul(lhs: Option<Value>, rhs: Option<Value>) -> Value {
    if rhs.is_none() {panic!("Tried to mul with no right hand side value")}
    if lhs.is_none() {panic!("Tried to mul with no left hand side value")}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Value::Number(x * y),
        (_, Value::Number(_)) => panic!("Tried to mul a non-number value to a number"),
        (Value::Number(_), _) => panic!("Tried to mul a number to a non-number value"),
        (_, _) => panic!("Tried to mul two non-number values")
    }
}

fn div(lhs: Option<Value>, rhs: Option<Value>) -> Value {
    if rhs.is_none() {panic!("Tried to div with no right hand side value")}
    if lhs.is_none() {panic!("Tried to div with no left hand side value")}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Value::Number(x / y),
        (_, Value::Number(_)) => panic!("Tried to div a non-number value to a number"),
        (Value::Number(_), _) => panic!("Tried to div a number to a non-number value"),
        (_, _) => panic!("Tried to div two non-number values")
    }
}



fn equal(lhs: Option<Value>, rhs: Option<Value>) -> Value {
    if rhs.is_none() || lhs.is_none() {panic!()}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Value::Boolean(x == y),
        (Value::Boolean(x), Value::Boolean(y)) => Value::Boolean(x == y),
        _ => Value::Boolean(false)
    }
}

fn greater(lhs: Option<Value>, rhs: Option<Value>) -> Value {
    if rhs.is_none() || lhs.is_none() {panic!()}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Value::Boolean(x > y),
        (x, y) => panic!("Can't compare {} and {}", x, y)
    }
}

fn lower(lhs: Option<Value>, rhs: Option<Value>) -> Value {
    if rhs.is_none() || lhs.is_none() {panic!()}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Value::Boolean(x < y),
        _ => panic!("Can't compare Values other than numbers")
    }
}

fn greater_equal(lhs: Option<Value>, rhs: Option<Value>) -> Value {
    if rhs.is_none() || lhs.is_none() {panic!()}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Value::Boolean(x >= y),
        (x, y) => panic!("Can't compare {} and {}", x, y)
    }
}

fn lower_equal(lhs: Option<Value>, rhs: Option<Value>) -> Value {
    if rhs.is_none() || lhs.is_none() {panic!()}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => Value::Boolean(x <= y),
        (x, y) => panic!("Can't compare {} and {}", x, y)
    }
}

fn and(lhs: Option<Value>, rhs: Option<Value>) -> Value {
    if rhs.is_none() || lhs.is_none() {panic!()}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Boolean(x), Value::Boolean(y)) => Value::Boolean(x && y),
        _ => panic!("Can't 'and' values other than booleans")
    }
}

fn or(lhs: Option<Value>, rhs: Option<Value>) -> Value {
    if rhs.is_none() || lhs.is_none() {panic!()}

    let rhs = rhs.unwrap();
    let lhs = lhs.unwrap();

    match (lhs, rhs) {
        (Value::Boolean(x), Value::Boolean(y)) => Value::Boolean(x || y),
        _ => panic!("Can't 'or' values other than booleans")
    }
}

fn inverse(lhs: Option<Value>, rhs: Option<Value>) -> Value {
    if rhs.is_some() {panic!("Unexpected operand")}
    if lhs.is_none() {panic!("Expected operand")}

    let lhs = lhs.unwrap();

    match lhs {
        Value::Boolean(x) => Value::Boolean(!x),
        _ => panic!("Expected boolean")
    }
}