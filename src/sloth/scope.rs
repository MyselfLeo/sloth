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
    pub id: ScopeID,
    pub variables: HashMap<String, Value>,
    pub parent: Option<ScopeID>
}


impl Clone for Scope {
    fn clone(&self) -> Self {
        let mut variable_clones: HashMap<String, Value> = HashMap::new();
        for v in &self.variables {variable_clones.insert(v.0.clone(), v.1.clone());}

        Self {
            id: self.id.clone(),
            variables: variable_clones,
            parent: self.parent.clone()
        }
    }
}

impl Scope {
    /// Return the value contained in the given variable. Prefer variable in this scope,
    /// but can also query parent scope for variable
    pub fn get_variable(&mut self, name: String, program: &mut SlothProgram) -> Result<&mut Value, String> {
        match self.variables.get(&name) {
            Some(v) => Ok(&mut v),
            None => {
                if self.parent.is_some() {
                    let parent_scope = program.get_scope(self.parent.unwrap())?;

                    parent_scope.clone().get_variable(name, program)
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


    /// Useful feature to get a list of each input values (@0, @1, @2, etc.), in order
    pub fn get_inputs(&self) -> Vec<Value> {
        let mut i = 0;
        let mut res: Vec<Value> = Vec::new();

        loop {
            let name = format!("@{}", i);
            match self.variables.get(&name) {
                Some(v) => {
                    res.push(v.clone());
                    i += 1;
                }
                None => break
            }
        }

        res
    }
}