use super::OperatorFunction;
use crate::lexer::Operator::Inv;
use crate::sloth::types::Type::*;
use crate::sloth::value::Value;





pub fn get_all() -> Vec<OperatorFunction> {
    vec![
        OperatorFunction::new(Inv, vec![Boolean], Boolean, inv_value),
    ]
}




fn inv_value(v1: Value, v2: Value) -> Value {
    match (v1, v2) {
        (Value::Boolean(x1), Value::Any) => Value::Boolean(!x1),
        _ => unreachable!()
    }
}