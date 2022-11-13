use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::zip;
use std::rc::Rc;

use crate::errors::{Error, ErrMsg, formatted_vec_string};
use crate::position::Position;
use super::function::{SlothFunction, FunctionSignature, FunctionCallSignature};
use super::scope::Scope;
use super::expression::Expression;
use super::structure::{StructSignature, ObjectBlueprint};
use super::types::Type;
use super::value::Value;
use crate::builtins;




const DEFAULT_BUILTIN_IMPORTS: [&str; 2] = ["io", "lists"];






/// Main structure of a Sloth program. Stores global definitions (function definition, structs definition, scopes)
/// Note: Variables are stored in the scopes
#[derive(Debug)]

pub struct SlothProgram {
    _filename: String,
    functions: HashMap<FunctionSignature, Box<dyn SlothFunction>>,
    structures: HashMap<StructSignature, Box<dyn ObjectBlueprint>>,

    // A static is an expression defined like a global variable (ex: static NUMBER = 34;). The expression
    // is evaluated in a blank scope each time it is called.
    // note: this is my workaround for constants. It's not really constant but it's not really mutable....
    statics: HashMap<String, Rc<Expression>>,

    builtins: Vec<builtins::BuiltInImport>,

    // list of every module name that can be called by module_name:function()
    imported_modules: Vec<String>,
}

impl SlothProgram {
    pub fn new(filename: String, import_default_builtins: bool) -> SlothProgram {
        let mut program = SlothProgram {
            _filename: filename,
            functions: HashMap::new(),
            structures: HashMap::new(),

            statics: HashMap::new(),

            imported_modules: Vec::new(),
            builtins: Vec::new()
        };

        if import_default_builtins {
            for import in DEFAULT_BUILTIN_IMPORTS {
                program.add_import(builtins::BuiltInImport::new(import.to_string(), None));
            }
        }


        program
    }


    
    /// Add a function to the Function Hashmap
    /// Can return an optional warning message if a previously defined function was overwritten
    pub fn push_function(&mut self, function: Box<dyn SlothFunction>) -> Option<String> {
        // add the module of the function in the imported_module vec
        if let Some(m) = function.get_module() {
            if !self.imported_modules.contains(&m) {self.imported_modules.push(m)}
        }

        match self.functions.insert(function.get_signature(), function) {
            Some(f) => {
                let msg = format!("Redefinition of function {}. Previous definition was overwritten", f.get_name());
                Some(msg)
            }
            None => None
        }
    }




    /// Return the requested function definition
    pub fn get_function(&self, signature: &FunctionCallSignature) -> Result<&Box<dyn SlothFunction>, String> {
        let mut signatures = Vec::new();
        for (key, _) in self.functions {
            signatures.push(key)
        }

        // remove each signature that don't match, criteria by criteria.
        // at each point, check if no signature is left, in order to return a
        // fitting error msg

        // name of the function
        signatures.retain(|k| k.name == *signature.name);
        if signatures.is_empty() {
            return Err(format!("Function '{}' is not defined", signature.name))
        }

        // module of the function
        if let Some(module) = signature.module {
            // return err if the module was never imported
            if !self.imported_modules.contains(&module) {
                return Err(format!("Module '{}' was not imported", module))
            }

            signatures.retain(|k| k.module.is_none() || k.module == Some(module));
            if signatures.is_empty() {
                return Err(format!("Function '{}' is not defined in the module '{}'", signature.name, module))
            }
        }

        // owner type
        signatures.retain(|k| k.owner_type == signature.owner_type);
        if signatures.is_empty() {
            return match signature.owner_type {
                Some(t) => Err(format!("Function '{}' is not defined for the type {}", signature.name, t)),
                None => Err(format!("Function '{}' is not defined", signature.name))
            }
        }

        // input types
        signatures.retain(
            |k| {
                match k.input_types {
                    None => true,
                    Some(t) => {
                        let types: Vec<Type> = t.iter().map(|(v, _)| *v).collect();
                        types == signature.input_types
                    }
                }
            }
        );
        if signatures.is_empty() {
            let type_str = signature.input_types.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", ");
            return Err(format!("Function '{}' is not defined for the following input types: {}", signature.name, type_str))
        }

        // output type
        signatures.retain(|k| k.output_type.is_none() || k.output_type == Some(signature.output_type));
        if signatures.is_empty() {
            return Err(format!("Function '{}' is not defined with an output type of {}", signature.name, signature.output_type))
        }


        // At this point, there should be only one signature left:
        // - 2 same signatures should not exist (hashmap)
        // - 'no signature' was previously tested

        // return the function
        match signatures.get(0) {
            Some(s) => {
                Ok(self.functions.get(s).unwrap())
            },
            None => unreachable!()
        }
    }







    /// Push a new ObjectBlueprint to the program
    /// Can return an optional warning message if a previously defined function was overwritten
    pub fn push_struct(&mut self, struct_name: String, struct_module: Option<String>, blueprint: Box<dyn ObjectBlueprint>) -> Option<String> {
        // add the module of the struct in the imported_module vec
        if let Some(m) = struct_module {
            if !self.imported_modules.contains(&m) {self.imported_modules.push(m)}
        }
        
        let signature = StructSignature::new(struct_module, struct_name.clone());
        match self.structures.insert(signature.clone(), blueprint) {
            Some(_) => {
                let msg = format!("Redefinition of structure {}. Previous definition was overwritten", struct_name);
                Some(msg)
            }
            None => None
        }
    }



    /// Return the blueprint of the given object name
    pub fn get_struct(&self, signature: &StructSignature) -> Result<Box<dyn ObjectBlueprint>, String> {

        // Check that if the potentially specified module exists
        match &signature.module {
            Some(n) => {
                if !self.imported_modules.contains(n) {
                    return Err(format!("Unknown module '{}'", n))
                }
            },
            None => ()
        };

        // A perfect fit is found
        match self.structures.get(signature) {
            None => (),
            Some(v) => {return Ok(v.box_clone());}
        };

        match &signature.module {
            Some(m) => {
                return Err(format!("Structure '{}' does not exists in module '{}'", signature.name, m))
            },

            None => {
                // Get the list of structures with the given name
                let mut matching_def = Vec::new();
                for (sign, def) in &self.structures {
                    if sign.name == signature.name {matching_def.push(def.clone())}
                };

                match matching_def.len() {
                    1 => return Ok(matching_def[0].box_clone()),
                    0 => return Err(format!("Structure '{}' does not exists", signature.name)),
                    n => return Err(format!("{} instances of structure '{}' found in the scope. Precise the module like that: module:StructureName {{...}}", n, signature.name))
                }
            }

        }
    }





    /// Add an expression to the statics, return error if the name is already used
    pub fn push_static(&mut self, name: &String, expr: Rc<Expression>) -> Result<(), String> {
        match self.statics.insert(name.clone(), expr) {
            Some(_) => Err(format!("Static expression '{}' is already defined", name)),
            None => Ok(()),
        }
    }

    /// Return a reference to the value resulting of the evaluation of the given static expr,
    /// None if it does not exists, or an error string if something occured
    pub fn get_static(&mut self, name: &String) -> Result<Option<Rc<RefCell<Value>>>, Error> {
        let expr = match self.statics.get(name) {
            Some(v) => v.clone(),
            None => return Ok(None)
        };

        // empty scope for the expression evaluation
        let scope = Rc::new(RefCell::new(Scope::new()));
        let res = unsafe {
            expr.evaluate(scope,self, false)?
        };
        Ok(Some(res))
    }



    /// Return whether the given static is set or not
    pub fn is_set(&self, name: &String) -> bool {
        self.statics.contains_key(name)
    }








    /// Add a new import to the program
    pub fn add_import(&mut self, import: builtins::BuiltInImport) {
        if !self.builtins.contains(&import) {
            if !self.imported_modules.contains(&import.module) {
                self.imported_modules.push(import.module.clone())
            }
            self.builtins.push(import)
        }
    }



    /// Import the requested builtins
    pub fn import_builtins(&mut self) -> Result<(), String> {
        let (f, s) = builtins::collapse_imports(self.builtins.clone())?;
        for function in f {self.push_function(function);}
        for blueprint in s {
            let sign = blueprint.get_signature();
            self.push_struct(sign.name.clone(), sign.module.clone(), blueprint);
        }
        Ok(())
    }


    /// Find the 'main' function, check its validity, execute it with the given arguments and return what the main function returned
    pub unsafe fn run(&mut self, s_args: Vec<String>) -> Result<Value, Error> {
        let main_func_id = FunctionSignature::new(None, "main".to_string(), None, None, Some(Type::Number));

        // Check if the main function exists and is well defined
        let main_func = match self.get_function(&main_func_id) {
            Ok(v) => v,
            Err(_) => {return Err(Error::new(ErrMsg::NoEntryPoint("Your program needs a 'main' function, returning a Number (the return code of your program), as an entry point.".to_string()), None))}
        };

        let main_inputs = main_func.get_signature().input_types.unwrap();

        // Convert given arguments to Values, push them to the Expression Stack and store its Expression ids
        let mut args: Vec<Rc<Expression>> = Vec::new();

        let dummy_pos = Position {filename: "".to_string(), line: 0, first_column: 0, last_column: Some(0)};

        if s_args.len() != main_inputs.len() {
            // Create a string representing the required arguments types, like "num, bool, string"
            let input_types_list = formatted_vec_string(&main_inputs.iter().map(|(t, _)| t).collect(), ',');
            let err_msg = format!("Given {} command-line argument(s), but the main function requires {} argument(s): {}", s_args.len(), main_inputs.len(), input_types_list);
            return Err(Error::new(ErrMsg::InvalidArguments(err_msg), None))
        }

        for (arg, (t, _)) in zip(s_args, main_inputs) {
            let value = match Value::string_to_value(arg, t) {
                Ok(v) => v,
                Err(e) => {
                    let err_msg = format!("Error while parsing command-line arguments: {}", e);
                    return Err(Error::new(ErrMsg::InvalidArguments(err_msg), None))
                }
            };

            args.push(Rc::new(Expression::Literal(value, dummy_pos.clone())));
        }

        // Call the main function
        let f_call = Expression::FunctionCall(None, main_func_id, args, dummy_pos.clone());

        let scope = Rc::new(RefCell::new(Scope::new()));
        match f_call.evaluate(scope, self, false) {
            Ok(reference) => Ok(reference.borrow().to_owned()),
            Err(e) => Err(e)
        }
    }





    /// Print to console the list of functions defined in the program
    pub fn print_functions(self) {
        println!("{:25}{:15}{:15}{:25}{:15}", "FUNCTION NAME", "OWNER TYPE", "MODULE", "INPUT TYPES", "OUTPUT TYPE");
        for (signature, _) in self.functions {
            let type_txt = match signature.owner_type {
                Some(v) => format!("{}", v),
                None => "-".to_string(),
            };
            let module_txt = match signature.module {
                Some(v) => format!("{}", v),
                None => "-".to_string(),
            };
            let input_types_txt = match signature.input_types {
                Some(v) => {
                    let mut res = "".to_string();
                    for (t, b) in v {
                        if b {
                            if res.is_empty() {res = format!("~{t}")}
                            else {res = format!("{}, ~{}", res, t);}
                        }
                        else {
                            if res.is_empty() {res = format!("{t}")}
                            else {res = format!("{}, {}", res, t);}
                        }
                    }
                    res
                },
                None => "-".to_string()
            };
            let output_type_str = match signature.output_type {
                Some(v) => format!("{v}"),
                None => "-".to_string()
            };

            println!("{:25}{:15}{:15}{:25}{:15}", signature.name, type_txt, module_txt, input_types_txt, output_type_str);
        }
    }
}