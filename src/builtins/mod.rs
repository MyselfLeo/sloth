//! The builtin functions of Sloth are defined in this module. They implement [SlothFunction].

use std::cell::RefCell;
use std::rc::Rc;

use crate::sloth::function::{SlothFunction, FunctionSignature};
use crate::sloth::program::SlothProgram;
use crate::sloth::scope::Scope;
use crate::sloth::types::Type;
use crate::sloth::structure::ObjectBlueprint;
use crate::errors::{Error, ErrMsg};
use crate::sloth::value::Value;
pub mod io;
pub mod numbers;
pub mod strings;
pub mod lists;
pub mod maths;
pub mod files;
pub mod clock;
pub mod media;
pub mod random;



#[allow(dead_code)]
pub enum BuiltinTypes {
    Function,
    Structure
}




pub const MODULES: [&str; 9] = [
    "io",
    "numbers",
    "strings",
    "lists",
    "maths",
    "files",
    "clock",
    "media",
    "random"
];






/// Struct representing the import of a builtin.
/// It contains the module being imported, and a list of builtins from this module
/// or the whole module if the list is None
/// Note: This struct CAN represent builtins that do not exists (either a fake module or fake builtin)
#[derive(Clone, PartialEq, Debug)]
pub struct BuiltInImport {
    pub module: String,
    builtins: Option<Vec<String>>
}

impl BuiltInImport {
    pub fn new(module: String, builtins: Option<Vec<String>>) -> BuiltInImport {
        BuiltInImport {module, builtins}
    }


    /// Check if the import is valid (module and each builtins exists). If it isn't, return an error msg
    /// 
    /// # Arguments
    /// * `disabled` - list of builtin modules disabled (via a program argument, usually)
    pub fn is_valid(&self, disabled: &Vec<String>) -> Result<(), String> {
        if !MODULES.contains(&self.module.as_str()) {
            return Err(format!("Built-in module '{}' does not exists", self.module))
        }
        if disabled.contains(&self.module) {
            return Err(format!("Built-in module '{}' is disabled", self.module))
        }

        match &self.builtins {
            None => Ok(()),

            Some(v) => {
                // Get the list of builtins from the submodule
                let builtins = match self.module.as_str() {
                    "io" => io::BUILTINS.to_vec(),
                    "numbers" => numbers::BUILTINS.to_vec(),
                    "strings" => strings::BUILTINS.to_vec(),
                    "lists" => lists::BUILTINS.to_vec(),
                    "maths" => maths::BUILTINS.to_vec(),
                    "files" => files::BUILTINS.to_vec(),
                    "clock" => clock::BUILTINS.to_vec(),
                    "media" => media::BUILTINS.to_vec(),
                    "random" => random::BUILTINS.to_vec(),
                    _ => panic!("Trying to access builtins of module '{}', which do not exists", self.module)
                };

                // Check that each builtins requested is in the submodule
                for import in v {
                    if !builtins.contains(&import.as_str()) {
                        return Err(format!("Built-in '{}' does not exists in the module '{}'", import, self.module))
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
pub fn collapse_imports(mut imports: Vec<BuiltInImport>) -> Result<(Vec<Box<dyn SlothFunction>>, Vec<Box<dyn ObjectBlueprint>>), String> {
    let mut imported: Vec<String> = Vec::new();                  // Keeps track of which imports are already in the 2 vectors
    let mut funcs: Vec<Box<dyn SlothFunction>> = Vec::new();
    let mut structs: Vec<Box<dyn ObjectBlueprint>> = Vec::new();


    let mut i = 0;


    while i < imports.len() {

        let import = imports[i].clone();

        // Get each builtin requested from the module, or every builtins of the module
        let builtins = match &import.builtins {
            Some(v) => v.clone(),
            None => {
                let list = match import.module.as_str() {
                    "io" => io::BUILTINS.to_vec(),
                    "numbers" => numbers::BUILTINS.to_vec(),
                    "strings" => strings::BUILTINS.to_vec(),
                    "lists" => lists::BUILTINS.to_vec(),
                    "maths" => maths::BUILTINS.to_vec(),
                    "files" => files::BUILTINS.to_vec(),
                    "clock" => clock::BUILTINS.to_vec(),
                    "media" => media::BUILTINS.to_vec(),
                    "random" => random::BUILTINS.to_vec(),
                    _ => panic!()
                };

                let mut v = Vec::new();
                for e in list {v.push(e.to_string())}

                v
            }
        };


        for bi in builtins {
            let import_id = format!("{}:{}", import.module, bi);
            if !imported.contains(&import_id) {


                let builtin_type = match import.module.as_str() {
                    "io" => io::get_type(&bi),
                    "numbers" => numbers::get_type(&bi),
                    "strings" => strings::get_type(&bi),
                    "lists" => lists::get_type(&bi),
                    "maths" => maths::get_type(&bi),
                    "files" => files::get_type(&bi),
                    "clock" => clock::get_type(&bi),
                    "media" => media::get_type(&bi),
                    "random" => random::get_type(&bi),
                    _ => panic!()
                }?;


                match builtin_type {
                    BuiltinTypes::Function => {
                        let f = match import.module.as_str() {
                            "io" => io::get_function(bi),
                            "numbers" => numbers::get_function(bi),
                            "strings" => strings::get_function(bi),
                            "lists" => lists::get_function(bi),
                            "maths" => maths::get_function(bi),
                            "files" => files::get_function(bi),
                            "clock" => clock::get_function(bi),
                            "media" => media::get_function(bi),
                            "random" => random::get_function(bi),
                            _ => panic!()
                        };
                        funcs.push(f);
                    },

                    BuiltinTypes::Structure => {
                        let (structure_def, requirements) = match import.module.as_str() {
                            "io" => io::get_struct(bi),
                            "numbers" => numbers::get_struct(bi),
                            "strings" => strings::get_struct(bi),
                            "lists" => lists::get_struct(bi),
                            "maths" => maths::get_struct(bi),
                            "files" => files::get_struct(bi),
                            "clock" => clock::get_struct(bi),
                            "media" => media::get_struct(bi),
                            "random" => random::get_struct(bi),
                            _ => panic!()
                        };

                        structs.push(structure_def);

                        // add the requirements to the stack to be imported
                        imports.push(BuiltInImport::new(import.module.clone(), Some(requirements)))
                    },
                };
                
                
                imported.push(import_id);
            }
        }

        i += 1;
    };

    Ok((funcs, structs))
}















pub struct BuiltInFunction {
    signature: FunctionSignature,
    call_function: fn(Rc<RefCell<Scope>>, &mut SlothProgram) -> Result<(), Error>,
}

impl std::fmt::Debug for BuiltInFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BuiltInFunction").field("signature", &self.signature).finish()
    }
}

impl SlothFunction for BuiltInFunction {
    fn get_owner_type(&self) -> Option<Type> {self.signature.owner_type.clone()}
    fn get_signature(&self) -> FunctionSignature {self.signature.clone()}
    fn get_module(&self) -> Option<String> {self.signature.module.clone()}
    fn get_name(&self) -> String {self.signature.name.clone()}
    fn get_output_type(&self) -> Type {self.signature.output_type.as_ref().unwrap().clone()}
    fn get_input_types(&self) -> Option<Vec<Type>> {
        match &self.signature.input_types {
            None => None,
            Some(v) => {
                Some(
                    v.iter()
                     .map(|(t, _)| t.clone())
                     .collect::<Vec<Type>>()
                )
            }
        }
    }

    unsafe fn call(&self, scope: Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<(), Error> {
        (self.call_function)(scope, program)
    }
}


impl BuiltInFunction {
    pub fn new(name: &str, module: Option<&str>, owner_type: Option<Type>, output_type: Type, call_function: fn(Rc<RefCell<Scope>>, &mut SlothProgram) -> Result<(), Error>) -> BuiltInFunction {
        let new_module = match module {
            Some(s) => Some(s.to_string()),
            None => None
        };

        BuiltInFunction {
            signature: FunctionSignature{module: new_module, name: name.to_string(), owner_type, input_types: None, output_type: Some(output_type)},
            call_function
        }
    }
}











// USEFUL FUNCTIONS


/// Set the return value of the scope to the given value
pub fn set_return(scope: &Rc<RefCell<Scope>>, program: &mut SlothProgram, value: Value) -> Result<(), Error> {
    match scope.borrow().get_variable("@return".to_string(), program) {
        Ok(r) => {
            // Try to set the value
            match r.try_borrow_mut() {
                Ok(mut borrow) => {
                    *borrow = value;
                    Ok(())
                },
                Err(e) => return Err(Error::new(ErrMsg::RustError(e.to_string()), None))
            }
        },
        Err(e) => Err(e)
    }
}





/// Set the self value of the scope to the given value
pub fn set_self(scope: &Rc<RefCell<Scope>>, program: &mut SlothProgram, value: Value) -> Result<(), Error> {
    match scope.borrow().get_variable("@self".to_string(), program) {
        Ok(r) => {
            // Try to set the value
            match r.try_borrow_mut() {
                Ok(mut borrow) => {
                    *borrow = value;
                    Ok(())
                },
                Err(e) => return Err(Error::new(ErrMsg::RustError(e.to_string()), None))
            }
        },
        Err(e) => Err(e)
    }
}




/// Specify argument types, return Error if not matched
pub fn query_inputs(scope: &Rc<RefCell<Scope>>, expected: Vec<Type>, func_name: &str) -> Result<Vec<Value>, Error> {
    let inputs = scope.borrow().get_inputs();

    if inputs.len() != expected.len() {
        let err_msg = match expected.len() {
            0 => format!("Function '{}' requires no arguments, but was given {}", func_name, inputs.len()),
            1 => format!("Function '{}' requires 1 argument, but was given {}", func_name, inputs.len()),
            x => format!("Function '{}' requires {} argument(s), but was given {}", func_name, expected.len(), x)
        };
        return Err(Error::new(ErrMsg::InvalidArguments(err_msg), None));
    }

    let mut res = Vec::new();
    
    let mut i = 0;
    for (given, expected) in std::iter::zip(inputs, expected) {
        let brrw = given.borrow();
        if brrw.get_type() != expected {
            let err_msg = format!("Argument {} of function '{}' must be of type '{}', but was given a value of type '{}'", i, func_name, expected, brrw.get_type());
            return Err(Error::new(ErrMsg::InvalidArguments(err_msg), None));
        }
        else {
            res.push(brrw.to_owned())
        }

        i += 1;
    }

    Ok(res)
}




/// Return the given value as a natural. Return Error if not a positive number, panics if not enum object Value::Number
pub fn expect_natural(value: &Value, limit: Option<(usize, &str)>, arg_pos: usize) -> Result<usize, Error> {
    let res = match value {
        Value::Number(x) => {
            if *x < 0.0 {Err(format!("Argument {} cannot be negative ({})", arg_pos, x))}

            else {
                match limit {
                    Some((l, reason)) => {
                        if (*x as usize) > l {Err(format!("Argument {} cannot be greater than {} ({})", arg_pos, l + 1, reason))}
                        else {Ok(*x as usize)}
                    },
                    None => Ok(*x as usize)
                }
            }
        },
        _ => panic!("expect_natural given a non-Number value"),
    };

    match res {
        Ok(u) => Ok(u),
        Err(e) => Err(Error::new(ErrMsg::InvalidArguments(e), None))
    }
}



/// Return the value stored by the method's caller
pub fn get_self(scope: &Rc<RefCell<Scope>>, program: &mut SlothProgram) -> Result<Value, Error> {
    let value = match scope.borrow().get_variable("@self".to_string(), program) {
        Ok(v) => v.borrow().to_owned(),
        Err(e) => return Err(e),
    };

    Ok(value)
}