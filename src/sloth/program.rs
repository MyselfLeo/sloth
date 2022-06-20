use std::collections::HashMap;
use super::function::SlothFunction;
use super::scope::{Scope, ScopeID};
use super::structure::StructDefinition;







/// Main structure of a Sloth program. Stores global definitions (function definition, structs definition, scopes)
/// Note: Variables are stored in the scopes
pub struct SlothProgram {
    functions: HashMap<String, Box<dyn SlothFunction>>,
    structures: HashMap<String, StructDefinition>,
    scopes: HashMap<ScopeID, Scope>,
    scope_nb: u64
}

impl SlothProgram {
    pub fn new() -> SlothProgram {
        SlothProgram { functions: HashMap::new(), structures: HashMap::new(), scopes: HashMap::new(), scope_nb: 0 }
    }


    /// Add a scope to the Scope stack and return its ID
    pub fn push_scope(&mut self, scope: Scope) -> ScopeID {
        let scope_id = ScopeID::new(self.scope_nb);
        self.scopes.insert(scope_id.clone(), scope.clone());
        self.scope_nb += 1;

        scope_id
    }

    /// Return a reference to the scope with the given ScopeID
    pub fn get_scope(&self, id: ScopeID) -> Result<&Scope, String> {
        match self.scopes.get(&id) {
            Some(v) => Ok(v),
            None => Err("Tried to access a scope with a wrong scope ID".to_string())
        }
    }
}