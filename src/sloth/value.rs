use super::types::Type;
use super::structure::StructDefinition;


pub trait Value {
    fn get_type(&self) -> Type;
}



struct NumberValue {
    value: f64
}
impl Value for NumberValue {
    fn get_type(&self) -> Type {
        Type::Number
    }
}



struct BooleanValue {
    value: bool
}
impl Value for BooleanValue {
    fn get_type(&self) -> Type {
        Type::Boolean
    }
}




struct StringValue {
    value: String
}
impl Value for StringValue {
    fn get_type(&self) -> Type {
        Type::String
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
}




struct StructValue {
    struct_def: StructDefinition,
    fields_values: Vec<Box<dyn Value>>,
}
impl Value for StructValue {
    fn get_type(&self) -> Type {
        Type::Struct(self.struct_def.name.clone())
    }
}