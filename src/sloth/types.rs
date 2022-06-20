trait Typed {
    /// Compare the types of 2 Primitives
    fn is_same_type(&self, other: &Self) -> bool;
    fn get_type(&self) -> Primitive;
}




#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    Number(f64),
    Boolean(bool),
    String(String),
}


impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Number(x) => write!(f, "{}", x),
            Primitive::Boolean(x) => if *x {write!(f, "true")} else {write!(f, "false")},
            Primitive::String(x) => write!(f, "{}", x),
        }
    }
}






struct List {
    value_type: Primitive,
    values: Vec<Primitive>
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[");
        for (i, e) in self.values.iter().enumerate() {
            write!(f, "{}", e);
            if i < self.values.len() -1 {write!(f, " ");}
        }
        write!(f, "]")
    }
}



impl Typed for Primitive {
    fn is_same_type(&self, other: &Primitive) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    fn get_type(&self) -> Primitive {
        return self.clone()
    }
}


impl Typed for List {
    fn is_same_type(&self, other: &List) -> bool {
        std::mem::discriminant(&self.value_type) == std::mem::discriminant(&other.value_type)
    }

    fn get_type(&self) -> Primitive {
        return self.value_type.clone()
    }
}












struct Class {
    name: String,
    fields_name: Vec<String>,
    fields_types: Vec<Typed>
}