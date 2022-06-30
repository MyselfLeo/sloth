use crate::sloth::function::SlothFunction;
pub mod io;




fn get_builtin_from_sub(sub: &str, name: &str) -> Box::<dyn SlothFunction> {
    let full_name = format!("{} {}", sub, name);
    match full_name.as_str() {
        "io print" => Box::new(io::BuiltinIoPrint {}),
        "io read" => Box::new(io::BuiltinIoRead {}),
        _ => panic!("Trying to access a built in that does not exists")
    }
}





pub fn get_builtin(sub: &str, name: Option<&str>) -> Result<Vec<Box::<dyn SlothFunction>> , String>{
    let mut res: Vec<Box::<dyn SlothFunction>> = Vec::new();


    let sub_builtins = match sub {
        "io" => io::FUNCTIONS,
        n => {
            return Err(format!("Built-in category '{}' does not exists", n))
        }
    };


    match name {
        Some(n) => {
            if sub_builtins.contains(&n) {
                res.push(get_builtin_from_sub(sub, n))
            }
            else {return Err(format!("Built-in '{}' does not exists in category '{}'", sub, n))}
        }
        None => {
            for n in sub_builtins {
                res.push(get_builtin_from_sub(sub, n))
            }
        }
    }


    Ok(res)
}