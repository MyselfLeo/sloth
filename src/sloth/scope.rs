use std::collections::HashMap;
use super::value::Value;



#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    variables: HashMap<String, Box<dyn Value>>,
    parent: ScopeID
}

impl Clone for Scope {
    fn clone(&self) -> Self {
        let mut variable_clones: HashMap<String, Box<dyn Value>> = HashMap::new();
        for v in &self.variables {variable_clones.insert(v.0.clone(), v.1.box_clone());}

        Self {
            variables: variable_clones,
            parent: self.parent.clone()
        }
    }
}

impl Scope {
    /// Return the value contained in the given variable
    pub fn get_variable(&self, name: String) -> Result<Box<dyn Value>, String> {
        match self.variables.get(&name) {
            Some(v) => Ok(v.box_clone()),
            None => {
                let error_msg = format!("Called uninitialised variable '{}'", name);
                Err(error_msg.to_string())
            }
        }
    }


    /// Set value of a given variable
    pub fn set_variable(&mut self, name: String, value: Box<dyn Value>) {
        self.variables.insert(name, value);
    }
}