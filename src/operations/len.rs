use super::OperatorFunction;
use crate::lexer::Operator::Len;
use crate::sloth::types::Type::*;
use crate::sloth::value::Value;





pub fn get_all() -> Vec<OperatorFunction> {
    vec![
        OperatorFunction::new(Len, vec![String], Number, len_value),
        OperatorFunction::new(Len, vec![List(Box::new(Any))], Number, len_value),
    ]
}




fn len_value(v1: Value, v2: Value) -> Value {
    match (v1, v2) {
        (Value::String(x1), Value::Any) => Value::Number(x1.len() as f64),
        (Value::List(_, v), Value::Any) => Value::Number(v.len() as f64),
        _ => unreachable!()
    }
}