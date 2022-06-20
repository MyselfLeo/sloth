use std::string;

use super::types::Type;
use super::structure::StructDefinition;


pub trait Value {
    fn get_type(&self) -> Type;
    fn box_clone(&self) -> Box<dyn Value>;
    fn to_string(&self) -> String;
}


#[derive(Clone)]
struct NumberValue {
    value: f64
}
impl Value for NumberValue {
    fn get_type(&self) -> Type {
        Type::Number
    }
    fn box_clone(&self) -> Box<dyn Value> {
        Box::new(self.clone())
    }
    fn to_string(&self) -> String {
        format!("{}", self.value).to_string()
    }
}


#[derive(Clone)]
struct BooleanValue {
    value: bool
}
impl Value for BooleanValue {
    fn get_type(&self) -> Type {
        Type::Boolean
    }
    fn box_clone(&self) -> Box<dyn Value> {
        Box::new(self.clone())
    }
    fn to_string(&self) -> String {
        if self.value {"true".to_string()}
        else {"false".to_string()}
    }
}



#[derive(Clone)]
struct StringValue {
    value: String
}
impl Value for StringValue {
    fn get_type(&self) -> Type {
        Type::String
    }
    fn box_clone(&self) -> Box<dyn Value> {
        Box::new(self.clone())
    }
    fn to_string(&self) -> String {
        self.value.clone()
    }
}


struct ListValue {
    value_type: Type,
    value: Vec<Box<dyn Value>>
}
impl Value for ListValue {
    fn get_type(&self) -> Type {
        Type::List(Box::new(self.value_type.clone()))
    }
    fn box_clone(&self) -> Box<dyn Value> {
        let mut vec_clone = Vec::new();
        for v in &self.value {vec_clone.push(v.box_clone())}

        Box::new(
            ListValue {
                value_type: self.value_type.clone(),
                value: vec_clone,
            }
        )
    }
    fn to_string(&self) -> String {
        let mut string_vec: Vec<String> = Vec::new();
        for v in &self.value {string_vec.push(v.to_string())}
        
        format!("[{}]", string_vec.join(", ")).to_string()
    }
}



struct StructValue {
    struct_def: StructDefinition,
    fields_values: Vec<Box<dyn Value>>,
}
impl Value for StructValue {
    fn get_type(&self) -> Type {
        Type::Struct(self.struct_def.name.clone())
    }
    fn box_clone(&self) -> Box<dyn Value> {
        let mut vec_clone = Vec::new();
        for v in &self.fields_values {vec_clone.push(v.box_clone())}

        Box::new(
            StructValue {
                struct_def: self.struct_def.clone(),
                fields_values: vec_clone,
            }
        )
    }
    fn to_string(&self) -> String {
        format!("'{}' object", self.struct_def.name).to_string()
    }
}