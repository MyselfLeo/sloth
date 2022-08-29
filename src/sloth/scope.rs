use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use super::program::SlothProgram;
use super::value::Value;



/// A scope is an environment in which variables lives.
pub struct Scope {
    pub variables: HashMap<String, Rc<RefCell<Value>>>,
    pub parent: Option<Rc<RefCell<Scope>>>
}


impl Scope {
    /// Return the value contained in the given variable. Prefer variable in this scope,
    /// but can also query parent scope for variable
    pub fn get_variable(&self, name: String, program: &mut SlothProgram) -> Result<Rc<RefCell<Value>>, String> {
        match self.variables.get(&name) {
            Some(v) => Ok(v.clone()),
            None => {
                if self.parent.is_some() {
                    let parent_scope = program.get_scope(self.parent.unwrap())?;
                    parent_scope.get_variable(name, program)
                }
                else {
                    let error_msg = format!("Called uninitialised variable '{}'", name);
                    Err(error_msg.to_string())
                }
            }
        }
    }




    /// Useful feature to get a list of each input values (@0, @1, @2, etc.), in order
    pub fn get_inputs(&self) -> Vec<Rc<RefCell<Value>>> {
        let mut i = 0;
        let mut res: Vec<Rc<RefCell<Value>>> = Vec::new();

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