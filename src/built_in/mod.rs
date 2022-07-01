use crate::sloth::function::SlothFunction;
pub mod io;




/// Struct representing a builtin. It can be a function or a structure
pub struct BuiltInIdent {
    submodule: String,
    name: String
}

impl BuiltInIdent {
    pub fn new(submodule: String, name: String) -> BuiltInIdent {
        BuiltInIdent {submodule, name}
    }
}





/// Takes a BuiltInIdent and return a Box to the given SlothFunction
fn get_function_builtin(builtin: BuiltInIdent) -> Box::<dyn SlothFunction> {
    let full_name = format!("{} {}", builtin.submodule, builtin.name);
    match full_name.as_str() {
        "io print" => Box::new(io::BuiltinIoPrint {}),
        "io read" => Box::new(io::BuiltinIoRead {}),
        _ => panic!("Trying to access a built-in that does not exists")
    }
}





/// Return a list of BuiltInIdent from the given submodule and optional builtin name
pub fn get_builtins_ident(sub: &str, name: Option<&str>) -> Result<Vec<BuiltInIdent> , String> {
    let mut res: Vec<BuiltInIdent> = Vec::new();


    let sub_builtins = match sub {
        "io" => io::FUNCTIONS,
        n => {
            return Err(format!("Built-in category '{}' does not exists", n))
        }
    };


    match name {
        Some(n) => {
            if sub_builtins.contains(&n) {
                res.push(BuiltInIdent::new(sub.to_string(), n.to_string()))
            }
            else {return Err(format!("Built-in '{}' does not exists in category '{}'", sub, n))}
        }
        None => {
            for n in sub_builtins {
                res.push(BuiltInIdent::new(sub.to_string(), n.to_string()))
            }
        }
    }


    Ok(res)
}