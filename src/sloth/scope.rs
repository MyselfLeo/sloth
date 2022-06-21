use std::collections::HashMap;
use super::program::SlothProgram;

use super::value::Value;



#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/// Used by scopes to reference to parent scope in the Scope stack
pub struct ScopeID {
    id: u64
}
impl ScopeID {
    pub fn new(value: u64) -> ScopeID {
        ScopeID { id: value }
    }
}



/// A scope is an environment in which variables lives.
pub struct Scope {
    variables: HashMap<String, Value>,
    parent: Option<ScopeID>
}

impl Clone for Scope {
    fn clone(&self) -> Self {
        let mut variable_clones: HashMap<String, Value> = HashMap::new();
        for v in &self.variables {variable_clones.insert(v.0.clone(), v.1.clone());}

        Self {
            variables: variable_clones,
            parent: self.parent.clone()
        }
    }
}

impl Scope {
    /// Return the value contained in the given variable. Prefer variable in this scope,
    /// but can also query parent scope for variable
    pub fn get_variable(&self, name: String, program: &SlothProgram) -> Result<Value, String> {
        match self.variables.get(&name) {
            Some(v) => Ok(v.clone()),
            None => {
                if self.parent.is_some() {
                    let parent_scope = program.get_scope(self.parent.unwrap());

                    if parent_scope.is_err() {return Err("Parent scope ID is not valid when querying inherited variable".to_string())}
                    parent_scope.unwrap().get_variable(name, program)
                }
                else {
                    let error_msg = format!("Called uninitialised variable '{}'", name);
                    Err(error_msg.to_string())
                }
            }
        }
    }


    /// Set value of a given variable
    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
}