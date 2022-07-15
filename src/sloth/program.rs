use std::collections::{HashMap, BTreeMap};
use crate::errors::{Error, ErrorMessage};
use crate::tokenizer::ElementPosition;
use super::function::{SlothFunction, FunctionSignature};
use super::scope::{Scope, ScopeID};
use super::expression::{Expression, ExpressionID};
use super::structure::StructDefinition;
use super::types::Type;
use super::value::Value;
use crate::built_in;





/// Main structure of a Sloth program. Stores global definitions (function definition, structs definition, scopes)
/// Note: Variables are stored in the scopes
pub struct SlothProgram {
    filename: String,
    functions: BTreeMap<FunctionSignature, Box<dyn SlothFunction>>,
    structures: BTreeMap<String, StructDefinition>,
    scopes: HashMap<ScopeID, Scope>,
    expressions: HashMap<ExpressionID, Expression>,
    scope_nb: u64,
    expressions_nb: u64,

    builtins: Vec<built_in::BuiltInImport>,

    main_scope: Option<ScopeID>
}

impl SlothProgram {
    pub fn new(filename: String) -> SlothProgram {
        let mut program = SlothProgram {
            filename,
            functions: BTreeMap::new(),
            structures: BTreeMap::new(),
            scopes: HashMap::new(), 
            expressions: HashMap::new(),
            scope_nb: 0,
            expressions_nb: 0,

            builtins: Vec::new(),

            main_scope: None
        };

        let s_id = program.new_scope(None);
        program.main_scope = Some(s_id.clone());


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
        //println!("[DEBUG] Looking for {:?}", signature);

        /*for (f, _) in &self.functions {
            println!("[DEBUG]     I have {:?}", f)
        }*/

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
;
        match fitting_functions.len() {
            0 => Err(format!("No function named '{}' with the given inputs", signature.name)),
            1 => self.get_function(&fitting_functions[0]),
            _ => {Err(format!("Ambiguous function name: '{}' is found in multiple modules. Consider specifying the module ( module:{}(input1 input2 ...) )", signature.name, signature.name))}
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
        let (f, ()) = built_in::collapse_imports(&self.builtins)?;
        for function in f {self.push_function(function);}
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


        // Convert given arguments to Values, push them to the Expression Stack and store its Expression ids
        let mut args_id: Vec<ExpressionID> = Vec::new();

        let dummy_pos = ElementPosition {filename: "".to_string(), line: 0, first_column: 0, last_column: 0};

        for arg in s_args {
            let expr = Expression::Literal(Value::from_string(arg), dummy_pos.clone());
            args_id.push(self.push_expr(expr))
        }

        // Call the main function
        let scope = self.get_scope(self.main_scope.unwrap()).unwrap().clone();
        let f_call = Expression::FunctionCall(main_func_id, args_id, dummy_pos.clone());

        f_call.evaluate(&scope, self)
    }
}