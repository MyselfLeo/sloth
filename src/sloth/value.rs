use std::cell::RefCell;
use std::rc::Rc;

use super::types::Type;
use super::structure::SlothObject;



#[derive(Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    List(Type, Vec<Rc<RefCell<Value>>>),
    Object(Box<dyn SlothObject>)
}

impl PartialEq for Value {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::List(l0, l1), Self::List(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Object(l0), Self::Object(r0)) => l0 == r0,
            (_, _) => false
        }
    }
}


impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::Boolean(arg0) => f.debug_tuple("Boolean").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::List(arg0, arg1) => f.debug_tuple("List").field(arg0).field(arg1).finish(),
            Self::Object(_) => f.debug_tuple("Object").finish(),
        }
    }
}




impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::Number(_) => Type::Number,
            Value::Boolean(_) => Type::Boolean,
            Value::String(_) => Type::String,
            Value::List(t, _) => Type::List(Box::new(t.clone())),
            Value::Object(object) => Type::Object(object.get_signature().name)
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
                for v in values {
                    let borrow = v.borrow();
                    if borrow.get_type() == Type::String {string_vec.push(format!("\"{}\"", borrow));}
                    else {string_vec.push(borrow.to_string())}
                }
                format!("[{}]", string_vec.join(" ")).to_string()
            },
            Value::Object(object) => {
                let mut string_vec: Vec<String> = Vec::new();
                for f in object.get_fields().1 {string_vec.push(f.borrow().to_string())}
                format!("{}({})", object.get_signature().name, string_vec.join(" ")).to_string()
            }
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
            Type::List(_t) => Err("Cannot create a List from a String".to_string()),
            Type::Object(_n) => unimplemented!()
        }
    }


    /// Return a smart pointer to the field of the value
    pub fn get_field(&self, field_name: &String) -> Result<Rc<RefCell<Value>>, String> {
        match self {
            Value::Object(object) => object.get_field(field_name),

            Value::List(_, list_values) => {
                match field_name.parse::<usize>() {
                    Ok(i) => {
                        match list_values.get(i) {
                            Some(v) => Ok(v.clone()),
                            None => {Err(format!("Tried to access the {}th element of a List with only {} elements", i, list_values.len()))}
                        }
                    },
                    Err(_) => {Err(format!("Cannot index a List with '{}'", field_name))}
                }
            },

            // TODO: Change how strings works to allow smart pointer access
            /*
            Value::String(txt) => {
                match field_name.parse::<usize>() {
                    Ok(i) => {
                        match txt.get(i..i+1) {
                            Some(v) => Ok(Value::String(v.to_string())),
                            None => {Err(format!("Tried to access the {}th character of a String of length {}", i, txt.len()))}
                        }
                    },
                    Err(_) => {Err(format!("Cannot index a String with '{}'", field_name))}
                }
            },
             */

            v => Err(format!("Type '{}' doesn't have a field '{}'", v.get_type(), field_name))
        }
    }


    /*
    pub fn set_field(&mut self, field_name: &String, value: Value) -> Result<(), String> {
        match self {
            Value::Object(object) => object.set_field(field_name, value),

            Value::List(t, list_values) => {
                match field_name.parse::<usize>() {
                    Ok(i) => {
                        // Check type of new value
                        let value_type = value.get_type();
                        if *t != value_type {return Err(format!("Tried to set an element of type '{}' in a List of type '{}'", value_type, t))}
                        
                        if i > list_values.len() - 1 {list_values.push(value);}
                        else {list_values[i] = value;}

                        Ok(())
                    },
                    Err(_) => {Err(format!("Cannot index a List with '{}'", field_name))}
                }
            },

            Value::String(txt) => {
                match field_name.parse::<usize>() {
                    Ok(i) => {
                        // Check type of new value
                        let given_str = match value {
                            Value::String(v) => v,
                            v => {return Err(format!("Tried to set a character of a String to a value of type '{}'", v.get_type()))}
                        };
                        
                        if given_str.len() != 1 {return Err("Tried to set a character of a String to multiple characters. Note: You can only set one character to another (string[x] = \"a\")".to_string())}

                        if i > txt.len() - 1 {txt.push_str(given_str.as_str());}
                        else {txt.replace_range(i..i+1, &given_str);}

                        Ok(())
                    },
                    Err(_) => {Err(format!("Cannot index a String with '{}'", field_name))}
                }
            },

            v => Err(format!("Type '{}' doesn't have a field '{}'", v.get_type(), field_name))
        }
    }
     */



}



impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}