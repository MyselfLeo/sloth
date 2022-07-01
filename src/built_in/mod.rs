use crate::sloth::function::SlothFunction;
pub mod io;





/// Struct representing a builtin. It can be a function or a structure
#[derive(PartialEq, Clone)]
pub struct BuiltInIdent {
    submodule: String,
    name: String
}

impl BuiltInIdent {
    pub fn new(submodule: String, name: String) -> BuiltInIdent {
        BuiltInIdent {submodule, name}
    }
}




/// Return a vec of each builtins from the given module
pub fn get_sub_builtins(submodule: String) -> Result<Vec<String>, String> {
    let list = match submodule.as_str() {
        "io" => {io::BUILTINS},
        n => return Err(format!("Built-in category '{}' does not exists", n))
    };

    let res: Vec<String> = Vec::new();
    for bi in list {res.push(bi.to_string())};
    Ok(res)
}




/// Takes a BuiltInIdent and return a Box to the given SlothFunction
fn get_function_builtin(builtin: &BuiltInIdent) -> Box::<dyn SlothFunction> {
    let full_name = format!("{} {}", builtin.submodule, builtin.name);
    match full_name.as_str() {
        "io print" => Box::new(io::BuiltinIoPrint {}),
        "io read" => Box::new(io::BuiltinIoRead {}),
        _ => panic!("Trying to access a built-in that does not exists")
    }
}



/// Return a Boxed SlothFunction from the requested BuiltInIdent
pub fn get_builtin_from_ident(bi: &BuiltInIdent) -> Result<Box::<dyn SlothFunction>, String> {
    let sub_builtins = match bi.submodule.as_str() {
        "io" => io::BUILTINS,
        n => {
            return Err(format!("Built-in category '{}' does not exists", n))
        }
    };

    if sub_builtins.contains(&bi.name.as_str()) {Ok(get_function_builtin(bi))}
    else {Err(format!("Built-in '{}' does not exists in category '{}'", bi.submodule, bi.name))}
}