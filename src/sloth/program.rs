use std::collections::{HashMap, BTreeMap};
use std::iter::zip;

use crate::errors::{Error, ErrorMessage, formatted_vec_string};
use crate::tokenizer::ElementPosition;
use super::function::{SlothFunction, FunctionSignature};
use super::scope::{Scope, ScopeID};
use super::expression::{Expression, ExpressionID};
use super::structure::{StructSignature, StructDefinition};
use super::types::Type;
use super::value::Value;
use crate::built_in;




const DEFAULT_SUBMODULE_IMPORTS: [&str; 0] = [];






/// Main structure of a Sloth program. Stores global definitions (function definition, structs definition, scopes)
/// Note: Variables are stored in the scopes
pub struct SlothProgram {
    _filename: String,
    functions: BTreeMap<FunctionSignature, Box<dyn SlothFunction>>,
    structures: HashMap<StructSignature, StructDefinition>,
    scopes: HashMap<ScopeID, Scope>,
    expressions: HashMap<ExpressionID, Expression>,
    scope_nb: u64,
    expressions_nb: u64,

    builtins: Vec<built_in::BuiltInImport>,

    main_scope: Option<ScopeID>
}

impl SlothProgram {
    pub fn new(filename: String, import_default_builtins: bool) -> SlothProgram {
        let mut program = SlothProgram {
            _filename: filename,
            functions: BTreeMap::new(),
            structures: HashMap::new(),
            scopes: HashMap::new(), 
            expressions: HashMap::new(),
            scope_nb: 0,
            expressions_nb: 0,

            builtins: Vec::new(),

            main_scope: None
        };

        let s_id = program.new_scope(None);
        program.main_scope = Some(s_id.clone());


        if import_default_builtins {
            for submod in DEFAULT_SUBMODULE_IMPORTS {
                program.add_import(built_in::BuiltInImport::new(submod.to_string(), None));
            }
        }


        program
    }


    
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
        match self.functions.get(&signature) {
            None => {}
            Some(v) => {return Ok(v);}
        };


        if signature.module.is_some() {
            return Err(format!("No function named '{}' in the module '{}'", signature.name, signature.module.clone().unwrap()));
        }


        let mut fitting_functions: Vec<FunctionSignature> = Vec::new();

        for (sign, _) in &self.functions {
            if sign.name == signature.name
            && sign.owner_type == signature.owner_type
            && (sign.input_types == signature.input_types || sign.input_types.is_none() || signature.input_types.is_none())
            && (sign.output_type == signature.output_type || signature.output_type.is_none()) {fitting_functions.push(sign.clone());}
        }

        match fitting_functions.len() {
            0 => {
                match &signature.owner_type {
                    Some(t) => Err(format!("No function named '{}' for type '{}' with the given inputs", signature.name, t)),
                    None => Err(format!("No function named '{}' with the given inputs", signature.name))
                }
            },
            1 => self.get_function(&fitting_functions[0]),
            _ => {Err(format!("Ambiguous function name: '{}' is found in multiple modules. Consider specifying the module ( module:{}(input1 input2 ...) )", signature.name, signature.name))}
        }
    }




    // TODO: Make a StructureSignature (name + module) instead of just using the name



    /// Push a new Structure definition to the program
    /// Can return an optional warning message if a previously defined function was overwritten
    pub fn push_struct(&mut self, structure: StructDefinition, module_name: Option<String>) -> Option<String> {
        let signature = StructSignature::new(module_name, structure.name.clone());
        match self.structures.insert(signature.clone(), structure) {
            Some(f) => {
                let msg = format!("Redefinition of structure {}. Previous definition was overwritten", f.name);
                Some(msg)
            }
            None => None
        }
    }



    /// Return the structure definition of the given structure name
    pub fn get_struct(&self, signature: &StructSignature) -> Result<StructDefinition, String> {
        // A perfect fit is found
        match self.structures.get(signature) {
            None => (),
            Some(v) => {return Ok(v.clone());}
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
                    1 => return Ok(matching_def[0].clone()),
                    0 => return Err(format!("Structure '{}' does not exists", signature.name)),
                    n => return Err(format!("{} instances of structure '{}' found in the scope. Precise the module like that: module:StructureName {{...}}", n, signature.name))
                }
            }

        }
    }




    /// Create a scope to the Scope stack and return its ID
    pub fn new_scope(&mut self, parent: Option<ScopeID>) -> ScopeID {
        let scope_id = ScopeID::new(self.scope_nb);

        let new_scope = Scope {
            id: scope_id.clone(),
            variables: HashMap::new(),
            parent: parent.clone()
        };

        self.scopes.insert(scope_id.clone(), new_scope);
        self.scope_nb += 1;

        scope_id
    }


    /// Remove the scope, freeing memory
    pub fn dump_scope(&mut self, scope: &ScopeID) {
        self.scopes.remove(scope);
    }


    /// Return a reference to the scope with the given ScopeID
    pub fn get_scope(&mut self, id: ScopeID) -> Result<&Scope, String>{
        match self.scopes.get(&id) {
            Some(v) => Ok(v),
            None => Err("Tried to access a scope with a wrong scope ID".to_string())
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
            self.builtins.push(import)
        }
    }



    /// Import the requested builtins
    pub fn import_builtins(&mut self) -> Result<(), String> {
        let (f, s) = built_in::collapse_imports(self.builtins.clone())?;
        for function in f {self.push_function(function);}
        for structure in s {self.push_struct(structure.clone(), structure.module);}
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
            let input_types_list = formatted_vec_string(&main_inputs, ',');
            let err_msg = format!("Given {} command-line argument(s), but the main function requires {} argument(s): {}", s_args.len(), main_inputs.len(), input_types_list);
            return Err(Error::new(ErrorMessage::InvalidArguments(err_msg), None))
        }

        for (arg, t) in zip(s_args, main_inputs) {
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
        let mut scope = self.get_scope(self.main_scope.unwrap()).unwrap().clone();
        let f_call = Expression::FunctionCall(main_func_id, args_id, dummy_pos.clone());

        f_call.evaluate(&mut scope, self)
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
                    for t in v {
                        if res.is_empty() {res = format!("{t}")}
                        else {res = format!("{}, {}", res, t);}
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



    /// Print to console the list of structures defined in the program
    pub fn print_structs(self) {
        for s in self.structures {
            println!("{:<15}{:?}", s.0.name, s.1);
        }
    }
}