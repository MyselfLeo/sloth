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



#[derive(Clone)]
/// A scope is an environment in which variables lives.
pub struct Scope {
    variables: HashMap<String, Box<dyn Value>>,
    parent: ScopeID
}