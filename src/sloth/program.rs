use std::collections::HashMap;
use crate::errors::{Error, ErrorMessage};
use crate::tokenizer::ElementPosition;
use super::function::SlothFunction;
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
    functions: HashMap<String, Box<dyn SlothFunction>>,
    structures: HashMap<String, StructDefinition>,
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
            functions: HashMap::new(),
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


        program
    }

    /// Add a function to the Function Hashmap.
    /// Can return an optional warning message if a previously defined function was overwritten
    pub fn push_function(&mut self, function: Box<dyn SlothFunction>) -> Option<String> {
        match self.functions.insert(function.get_name(), function) {
            Some(f) => {
                let msg = format!("Redefinition of function {}. Previous definition was overwritten", f.get_name());
                Some(msg)
            }
            None => None
        }
    }

    /// Return a clone of the requested function definition, or an error if the function is not defined
    pub fn get_function(&self, name: String) -> Result<&Box<dyn SlothFunction>, String> {
        match self.functions.get(&name) {
            None => {
                let msg = format!("Called undefined function {}", name);
                Err(msg)
            }
            Some(v) => Ok(v)
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
        let main_func = match self.get_function("main".to_string()) {
            Ok(v) => v,
            Err(_) => {return Err(Error::new(ErrorMessage::NoEntryPoint("Your program needs a 'main' function, returning a 'num' value, as an entry point.".to_string()), None))}
        };

        // the 'main' function must return a Number value, which will be rounded and returned as exit code
        if main_func.get_output_type() != Type::Number {
            return Err(Error::new(ErrorMessage::InvalidEntryPoint("The 'main' function must return a Number, that will be the exit code of your program".to_string()), None))
        }

        // Convert given arguments to Values, push them to the Expression Stack and store its Expression ids
        let mut args_id: Vec<ExpressionID> = Vec::new();

        let dummy_pos = ElementPosition {filename: "".to_string(), line: 0, first_column: 0, last_column: 0};

        for arg in s_args {
            let expr = Expression::Literal(Value::from_string(arg), dummy_pos.clone());
            args_id.push(self.push_expr(expr))
        }

        // Check that the main function exists
        // technically, the FunctionCall we create below would be able to return an error if 'main' does not exists. HOWEVER it has a dummy_pos, so generating an error from it would
        // panic as no file is named "".
        if !self.functions.contains_key("main") {return Err(Error::new(ErrorMessage::NoEntryPoint("Your program needs a 'main' function".to_string()), None))}

        // Call the main function
        let scope = self.get_scope(self.main_scope.unwrap()).unwrap().clone();
        let f_call = Expression::FunctionCall("main".to_string(), args_id, dummy_pos.clone());

        f_call.evaluate(&scope, self)
    }
}