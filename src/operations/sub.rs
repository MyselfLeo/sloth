use super::OperatorFunction;
use crate::lexer::Operator::Sub;
use crate::sloth::types::Type::*;
use crate::sloth::value::Value;





pub fn get_all() -> Vec<OperatorFunction> {
    vec![
        OperatorFunction::new(Sub, vec![Number, Number], Number, sub_values),
    ]
}




fn sub_values(v1: Value, v2: Value) -> Value {
    match (v1, v2) {
        (Value::Number(x1), Value::Number(x2)) => Value::Number(x1 - x2),
        _ => unreachable!()
    }
}