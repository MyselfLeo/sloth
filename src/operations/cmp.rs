use super::OperatorFunction;
use crate::lexer::Operator::{And, Or, Eq, Gr, Ge, Lw, Le};
use crate::sloth::types::Type::*;
use crate::sloth::value::Value;





pub fn get_all() -> Vec<OperatorFunction> {
    vec![
        OperatorFunction::new(And, vec![Boolean, Boolean], Boolean, and_values),
        OperatorFunction::new(Or, vec![Boolean, Boolean], Boolean, or_values),

        OperatorFunction::new(Eq, vec![Any, Any], Boolean, eq_values),

        OperatorFunction::new(Gr, vec![Number, Number], Boolean, gr_values),
        OperatorFunction::new(Gr, vec![String, String], Boolean, gr_values),

        OperatorFunction::new(Ge, vec![Number, Number], Boolean, ge_values),
        OperatorFunction::new(Ge, vec![String, String], Boolean, ge_values),

        OperatorFunction::new(Lw, vec![Number, Number], Boolean, lw_values),
        OperatorFunction::new(Lw, vec![String, String], Boolean, lw_values),

        OperatorFunction::new(Le, vec![Number, Number], Boolean, le_values),
        OperatorFunction::new(Le, vec![String, String], Boolean, le_values),
    ]
}



fn and_values(v1: Value, v2: Value) -> Value {
    match (v1, v2) {
        (Value::Boolean(x1), Value::Boolean(x2)) => Value::Boolean(x1 && x2),
        _ => unreachable!()
    }
}
fn or_values(v1: Value, v2: Value) -> Value {
    match (v1, v2) {
        (Value::Boolean(x1), Value::Boolean(x2)) => Value::Boolean(x1 || x2),
        _ => unreachable!()
    }
}

fn eq_values(v1: Value, v2: Value) -> Value {Value::Boolean(v1 == v2)}

fn gr_values(v1: Value, v2: Value) -> Value {
    match (v1, v2) {
        (Value::Number(x1), Value::Number(x2)) => Value::Boolean(x1 > x2),
        (Value::String(x1), Value::String(x2)) => Value::Boolean(x1 > x2),
        _ => unreachable!()
    }
}
fn ge_values(v1: Value, v2: Value) -> Value {
    match (v1, v2) {
        (Value::Number(x1), Value::Number(x2)) => Value::Boolean(x1 >= x2),
        (Value::String(x1), Value::String(x2)) => Value::Boolean(x1 >= x2),
        _ => unreachable!()
    }
}
fn lw_values(v1: Value, v2: Value) -> Value {
    match (v1, v2) {
        (Value::Number(x1), Value::Number(x2)) => Value::Boolean(x1 < x2),
        (Value::String(x1), Value::String(x2)) => Value::Boolean(x1 < x2),
        _ => unreachable!()
    }
}
fn le_values(v1: Value, v2: Value) -> Value {
    match (v1, v2) {
        (Value::Number(x1), Value::Number(x2)) => Value::Boolean(x1 <= x2),
        (Value::String(x1), Value::String(x2)) => Value::Boolean(x1 <= x2),
        _ => unreachable!()
    }
}