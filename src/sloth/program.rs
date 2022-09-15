use std::cell::RefCell;
use std::collections::{HashMap, BTreeMap};
use std::iter::zip;
use std::rc::Rc;

use crate::errors::{Error, ErrorMessage, formatted_vec_string};
use crate::tokenizer::ElementPosition;
use super::function::{SlothFunction, FunctionSignature};
use super::scope::Scope;
use super::expression::{Expression, ExpressionID};
use super::structure::{StructSignature, ObjectBlueprint};
use super::types::Type;
use super::value::Value;
use crate::built_in;




const DEFAULT_BUILTIN_IMPORTS: [&str; 1] = ["io"];






/// Main structure of a Sloth program. Stores global definitions (function definition, structs definition, scopes)
/// Note: Variables are stored in the scopes
pub struct SlothProgram {
    _filename: String,
    functions: BTreeMap<FunctionSignature, Box<dyn SlothFunction>>,
    structures: HashMap<StructSignature, Box<dyn ObjectBlueprint>>,
    expressions: HashMap<ExpressionID, Expression>,
    expressions_nb: u64,

    imported_modules: Vec<String>,
    builtins: Vec<built_in::BuiltInImport>,

    main_scope: Rc<RefCell<Scope>>
}

impl SlothProgram {
    pub fn new(filename: String, import_default_builtins: bool) -> SlothProgram {
        let mut program = SlothProgram {
            _filename: filename,
            functions: BTreeMap::new(),
            structures: HashMap::new(),
            expressions: HashMap::new(),
            expressions_nb: 0,

            imported_modules: Vec::new(),
            builtins: Vec::new(),

            main_scope: Rc::new(RefCell::new(Scope::new(None)))
        };

        if import_default_builtins {
            for import in DEFAULT_BUILTIN_IMPORTS {
                program.add_import(built_in::BuiltInImport::new(import.to_string(), None));
            }
        }


        program
    }



    pub fn main_scope(&self) -> Rc<RefCell<Scope>> {self.main_scope.clone()}


    
    /// Add a function to the Function Hashmap.
    /// Can return an optional warning message if a previously defined function was overwritten
    pub fn push_function(&mut self, function: Box<dyn SlothFunction>) -> Option<String> {
        match self.functions.insert(function.get_signature(), function) {
            Some(f) => {
                let msg = format!("Redefinition of function {}. Previous definition was overwritten", f.get_name());
                Some(msg)
            }
            None => None
        }
    }

    /// Return a clone of the requested function definition
    pub fn get_function(&self, signature: &FunctionSignature) -> Result<&Box<dyn SlothFunction>, String> {
        
        // If the module is given, we can try to find the perfect match
        if signature.module.is_some() {
            for (sign, f) in &self.functions {
                if sign.name == signature.name && sign.module == signature.module {return Ok(f.clone())}
            }
            return Err(format!("No function named '{}' in the module '{}'", signature.name, signature.module.clone().unwrap()));
        }


        let mut fitting_functions: Vec<&Box<dyn SlothFunction>> = Vec::new();

        for (sign, f) in &self.functions {
            if sign.name == signature.name
            && sign.owner_type == signature.owner_type
            && (sign.input_types == signature.input_types || sign.input_types.is_none() || signature.input_types.is_none())
            && (sign.output_type == signature.output_type || signature.output_type.is_none()) {fitting_functions.push(f.clone());}
        }

        match fitting_functions.len() {
            0 => {
                match &signature.owner_type {
                    Some(t) => Err(format!("No function named '{}' for type '{}' with the given inputs", signature.name, t)),
                    None => Err(format!("No function named '{}' with the given inputs", signature.name))
                }
            },
            1 => Ok(fitting_functions[0].clone()),
            _ => {Err(format!("Ambiguous function name: '{}' is found in multiple modules. Consider specifying the module ( module:{}(input1 input2 ...) )", signature.name, signature.name))}
        }
    }







    /// Push a new ObjectBlueprint to the program
    /// Can return an optional warning message if a previously defined function was overwritten
    pub fn push_struct(&mut self, struct_name: String, struct_module: Option<String>, blueprint: Box<dyn ObjectBlueprint>) -> Option<String> {
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





    /// Add an expression to the Expression stack and return its ID
    pub fn push_expr(&mut self, expr: Expression) -> ExpressionID {
        let expr_id = ExpressionID::new(self.expressions_nb);
        self.expressions.insert(expr_id.clone(), expr.clone());
        self.expressions_nb += 1;

        expr_id
    }

    /// Return a clone of an expression with the given ExpressionID
    pub fn get_expr(&self, id: ExpressionID) -> Result<Expression, String> {
        match self.expressions.get(&id) {
            Some(v) => Ok(v.clone()),
            None => Err("Tried to access an expression with a wrong expression ID".to_string())
        }
    }

    /// Add a new import to the program
    pub fn add_import(&mut self, import: built_in::BuiltInImport) {
        if !self.builtins.contains(&import) {
            if !self.imported_modules.contains(&import.module) {
                self.imported_modules.push(import.module.clone())
            }
            self.builtins.push(import)
        }
    }



    /// Import the requested builtins
    pub fn import_builtins(&mut self) -> Result<(), String> {
        let (f, s) = built_in::collapse_imports(self.builtins.clone())?;
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
            Err(_) => {return Err(Error::new(ErrorMessage::NoEntryPoint("Your program needs a 'main' function, returning a Number (the return code of your program), as an entry point.".to_string()), None))}
        };

        let main_inputs = main_func.get_signature().input_types.unwrap();

        // Convert given arguments to Values, push them to the Expression Stack and store its Expression ids
        let mut args_id: Vec<ExpressionID> = Vec::new();

        let dummy_pos = ElementPosition {filename: "".to_string(), line: 0, first_column: 0, last_column: Some(0)};

        if s_args.len() != main_inputs.len() {
            // Create a string representing the required arguments types, like "num, bool, string"
            let input_types_list = formatted_vec_string(&main_inputs.iter().map(|(t, _)| t).collect(), ',');
            let err_msg = format!("Given {} command-line argument(s), but the main function requires {} argument(s): {}", s_args.len(), main_inputs.len(), input_types_list);
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None))
        }

        for (arg, (t, _)) in zip(s_args, main_inputs) {
            let value = match Value::string_to_value(arg, t) {
                Ok(v) => v,
                Err(e) => {
                    let err_msg = format!("Error while parsing command-line arguments: {}", e);
                    return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None))
                }
            };

            let expr = Expression::Literal(value, dummy_pos.clone());
            args_id.push(self.push_expr(expr))
        }

        // Call the main function
        let f_call = Expression::FunctionCall(main_func_id, args_id, dummy_pos.clone());

        match f_call.evaluate(self.main_scope.clone(), self) {
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



    /// Print to console the list of expressions defined in the program
    pub fn print_exprs(self)  {
        println!("{:15}{}", "EXPRESSION ID", "EXPRESSION");
        for (id, e) in self.expressions {
            println!("{:<15}{:?}", id.id, e);
        }
    }
}