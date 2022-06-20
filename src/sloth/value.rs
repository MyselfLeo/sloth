use super::types::Type;
use super::structure::StructDefinition;


pub trait Value {
    fn get_type(&self) -> Type;
    fn box_clone(&self) -> Box<dyn Value>;
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
}