use super::OperatorFunction;
use crate::lexer::Operator::Add;
use crate::sloth::types::Type::*;
use crate::sloth::value::Value;





pub fn get_all() -> Vec<OperatorFunction> {
    vec![
        OperatorFunction::new(Add, vec![Number, Number], Number, add_values),
        OperatorFunction::new(Add, vec![Number, String], String, add_values),
        OperatorFunction::new(Add, vec![String, Number], String, add_values),
        OperatorFunction::new(Add, vec![String, String], String, add_values),
        OperatorFunction::new(Add, vec![Boolean, Boolean], Boolean, add_values),
    ]
}




fn add_values(v1: Value, v2: Value) -> Value {
    match (v1, v2) {
        (Value::Number(x1), Value::Number(x2)) => Value::Number(x1 + x2),
        (Value::Number(x), Value::String(s)) => Value::String(x.to_string() + &s),
        (Value::Boolean(b1), Value::Boolean(b2)) => Value::Boolean(b1 || b2),
        (Value::String(s), Value::Number(x)) => Value::String(s + &x.to_string()),
        (Value::String(s1), Value::String(s2)) => Value::String(s1 + &s2),
        _ => unreachable!()
    }
}