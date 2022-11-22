use super::OperatorFunction;
use crate::lexer::Operator::Add;
use crate::sloth::types::Type::*;
use crate::sloth::value::Value;





pub fn get_all() -> Vec<OperatorFunction> {
    vec![
        // Number and Strings
        OperatorFunction::new(Add, vec![Number, Number], Number, |v| {if let (Value::Number(x), Value::Number(y)) = (&v[0], &v[1]) {Value::Number(x + y)} else {unreachable!()}}),
        OperatorFunction::new(Add, vec![Number, String], String, |v| {if let (Value::Number(x), Value::String(y)) = (&v[0], &v[1]) {Value::String(x.to_string() + y)} else {unreachable!()}}),
        OperatorFunction::new(Add, vec![String, Number], String, |v| {if let (Value::String(x), Value::Number(y)) = (&v[0], &v[1]) {Value::String(x.clone() + &y.to_string())} else {unreachable!()}}),

        // Lists
    ]
}