use super::OperatorFunction;
use crate::lexer::Operator::Add;
use crate::sloth::types::Type::*;
use crate::sloth::value::Value;





pub fn get_all() -> Vec<OperatorFunction> {
    // This is stupid i hate it
    static NUM_NUM: fn(Vec<Value>) -> Value = num_num;
    static NUM_STRING: fn(Vec<Value>) -> Value = num_string;

    vec![
        // Number and Strings
        OperatorFunction::new(Add, vec![Number, Number], Number, &NUM_NUM),
        OperatorFunction::new(Add, vec![Number, String], String, &NUM_STRING),
        OperatorFunction::new(Add, vec![String, Number], String, &NUM_STRING),

        // Lists
    ]
}


fn num_num(v: Vec<Value>) -> Value {
    if let (Value::Number(x), Value::Number(y)) = (&v[0], &v[1]) {
        Value::Number(x + y)
    } else {unreachable!()}
}

fn num_string(v: Vec<Value>) -> Value {
    if let (Value::Number(x), Value::String(y)) = (&v[0], &v[1]) {
        Value::String(x.to_string() + y)
    }
    else if let (Value::String(x), Value::Number(y)) = (&v[0], &v[1]) {
        Value::String(y.to_string() + x)
    } else {unreachable!()}
}