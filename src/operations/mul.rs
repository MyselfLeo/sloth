use super::OperatorFunction;
use crate::lexer::Operator::Mul;
use crate::sloth::types::Type::*;
use crate::sloth::value::Value;





pub fn get_all() -> Vec<OperatorFunction> {
    vec![
        OperatorFunction::new(Mul, vec![Number, Number], Number, mul_values),
        OperatorFunction::new(Mul, vec![Boolean, Boolean], Boolean, mul_values),
    ]
}




fn mul_values(v1: Value, v2: Value) -> Value {
    match (v1, v2) {
        (Value::Number(x1), Value::Number(x2)) => Value::Number(x1 * x2),
        (Value::Boolean(b1), Value::Boolean(b2)) => Value::Boolean(b1 && b2),
        _ => unreachable!()
    }
}