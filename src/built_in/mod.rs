use crate::sloth::function::SlothFunction;
pub mod io;
pub mod numbers;
pub mod strings;
pub mod lists;



pub const SUBMODULES: [&str; 4] = [
    "io",
    "numbers",
    "strings",
    "lists"
];






/// Struct representing the import of a builtin.
/// It contains the submodule being imported, and a list of builtins from this submodule
/// or the whole submodule if the list is None
/// Note: This struct CAN represent builtins that do not exists (either a fake submodule or fake builtin)
#[derive(Clone, PartialEq)]
pub struct BuiltInImport {
    submodule: String,
    builtins: Option<Vec<String>>
}

impl BuiltInImport {
    pub fn new(submodule: String, builtins: Option<Vec<String>>) -> BuiltInImport {
        BuiltInImport {submodule, builtins}
    }


    /// Check if the import is valid (submodule and each builtins exists). If it isn't, return an error msg
    pub fn is_valid(&self) -> Result<(), String> {
        if !SUBMODULES.contains(&self.submodule.as_str()) {
            return Err(format!("Built-in submodule '{}' does not exists", self.submodule))
        }

        match &self.builtins {
            None => Ok(()),

            Some(v) => {
                // Get the list of builtins from the submodule
                let builtins = match self.submodule.as_str() {
                    "io" => io::BUILTINS.to_vec(),
                    "numbers" => numbers::BUILTINS.to_vec(),
                    "strings" => strings::BUILTINS.to_vec(),
                    "lists" => lists::BUILTINS.to_vec(),
                    _ => panic!("Trying to access builtins of submodule '{}', which do not exists", self.submodule)
                };

                // Check that each builtins requested is in the submodule
                for import in v {
                    if !builtins.contains(&import.as_str()) {
                        return Err(format!("Built-in '{}' does not exists in the submodule '{}'", import, self.submodule))
                    }
                }

                Ok(())
            }
        }
    }
}










/// Take a vec of imports and collaspes them into 2 vectors: one of functions and one
/// of structures (to be imported to the program's scope)
/// This function takes care of duplicates in the imports
pub fn collapse_imports(imports: &Vec<BuiltInImport>) -> Result<(Vec<Box<dyn SlothFunction>>, ()), String> {
    let mut imported: Vec<String> = Vec::new();                  // Keeps track of which imports are already in the 2 vectors
    let mut funcs: Vec<Box<dyn SlothFunction>> = Vec::new();
    // todo: add support for structures

    for import in imports {
        import.is_valid()?;

        // Get each built in requested, or every builtins of the submodule
        let builtins = match &import.builtins {
            Some(v) => v.clone(),
            None => {
                let list = match import.submodule.as_str() {
                    "io" => io::BUILTINS.to_vec(),
                    "numbers" => numbers::BUILTINS.to_vec(),
                    "strings" => strings::BUILTINS.to_vec(),
                    "lists" => lists::BUILTINS.to_vec(),
                    _ => panic!()
                };

                let mut v = Vec::new();
                for e in list {v.push(e.to_string())}

                v
            }
        };


        for bi in builtins {
            let import_id = format!("{}:{}", import.submodule, bi);
            if !imported.contains(&import_id) {

                // todo: add structure support
                let f = match import.submodule.as_str() {
                    "io" => io::get_function(bi),
                    "numbers" => numbers::get_function(bi),
                    "strings" => strings::get_function(bi),
                    "lists" => lists::get_function(bi),
                    _ => panic!()
                };
                funcs.push(f);
                imported.push(import_id);
            }
        }
    };

    Ok((funcs, ()))
}