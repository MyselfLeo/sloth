use super::types::Type;
use super::structure::StructDefinition;



#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    List(Type, Vec<Value>),
    Struct(StructDefinition, Vec<Value>)
}


impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::Number(_) => Type::Number,
            Value::Boolean(_) => Type::Boolean,
            Value::String(_) => Type::String,
            Value::List(t, _) => Type::List(Box::new(t.clone())),
            Value::Struct(struct_def, _) => Type::Struct(struct_def.name.clone())
        }
    }


    pub fn to_string(&self) -> String {
        match self {
            Value::Number(x) => format!("{}", x).to_string(),
            Value::Boolean(b) => {
                if *b {"true".to_string()}
                else {"false".to_string()}
            },

            Value::String(s) => s.clone(),
            
            Value::List(_, values) => {
                let mut string_vec: Vec<String> = Vec::new();
                for v in values {string_vec.push(v.to_string())}
                format!("[{}]", string_vec.join(" ")).to_string()
            },
            Value::Struct(s, _) => format!("'{}' object", s.name).to_string()
        }
    }


    /// Try to convert the given raw token string into a value
    pub fn from_raw_token(s: String) -> Value {
        if s.parse::<f64>().is_ok() {Value::Number(s.parse::<f64>().unwrap())}
        else if s == "true" {Value::Boolean(true)}
        else if s == "false" {Value::Boolean(false)}
        else if s.starts_with("\"") && s.ends_with("\"") {
            let text = s.trim_start_matches("\"").trim_end_matches("\"").to_string();
            Value::String(text)
        }
        else {panic!("Can't generate Value from string '{}'", s)}
    }


    /// Try to convert the given string (potentially a user input) into the desired type
    pub fn string_to_value(s: String, t: Type) -> Result<Value, String> {
        match t {
            Type::Any => panic!("Cannot generate a value from type Unknown"),
            Type::String => Ok(Value::String(s)),
            Type::Number => {
                match s.parse::<f64>() {
                    Ok(v) => Ok(Value::Number(v)),
                    Err(_) => Err(format!("Cannot convert '{}' into a Number value", s))
                }
            },
            Type::Boolean => {
                match s.as_str() {
                    "True" | "true" | "t" | "1" => Ok(Value::Boolean(true)),
                    "False" | "false" | "f" | "0" => Ok(Value::Boolean(false)),
                    _ => Err(format!("Cannot convert '{}' into a Boolean value", s))
                }
            }
            Type::List(_t) => Err("Cannot create a list from a string".to_string()),
            Type::Struct(_n) => unimplemented!()
        }
    }



    pub fn get_field(&self, field_name: &String) -> Result<Value, String> {
        match self {
            Value::Struct(s, f) => {
                match s.fields_names.iter().position(|x| x == field_name) {
                    Some(i) => Ok(f[i].clone()),
                    None => {Err(format!("Structure '{}' doesn't have a field '{}'", s.name, field_name))}
                }
            },

            Value::List(_, list_values) => {
                match field_name.parse::<usize>() {
                    Ok(i) => {
                        match list_values.get(i) {
                            Some(v) => Ok(v.clone()),
                            None => {Err(format!("Tried to access the {}th element of a list with only {} elements", i, list_values.len()))}
                        }
                    },
                    Err(_) => {Err(format!("Cannot index a list with '{}'", field_name))}
                }
            },

            v => Err(format!("Type '{}' doesn't have a field '{}'", v.get_type(), field_name))
        }
    }

    pub fn set_field(&mut self, field_name: &String, value: Value) -> Result<(), String> {
        match self {
            Value::Struct(s, f) => {
                match s.fields_names.iter().position(|x| x == field_name) {
                    Some(i) => {
                        // Check type of new value
                        let value_type = value.get_type();
                        if *s.fields_types[i] != value_type {return Err(format!("Field '{}' expect a value of type '{}', got '{}' instead", field_name, s.fields_types[i], value_type))}
                        
                        f[i] = value.clone();
                        Ok(())
                    },
                    None => {Err(format!("Structure '{}' doesn't have a field '{}'", s.name, field_name))}
                }
            },

            Value::List(t, list_values) => {
                match field_name.parse::<usize>() {
                    Ok(i) => {
                        // Check type of new value
                        let value_type = value.get_type();
                        if *t != value_type {return Err(format!("Tried to set an element of type '{}' in a list of type '{}'", value_type, t))}
                        
                        if i > list_values.len() - 1 {list_values.push(value);}
                        else {list_values[i] = value;}

                        Ok(())
                    },
                    Err(_) => {Err(format!("Cannot index a list with '{}'", field_name))}
                }
            },

            v => Err(format!("Type '{}' doesn't have a field '{}'", v.get_type(), field_name))
        }
    }



}



impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}