use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use super::program::SlothProgram;
use super::value::Value;


#[derive(Debug)]
/// A scope is an environment in which variables lives.
pub struct Scope {
    pub variables: HashMap<String, Rc<RefCell<Value>>>,
}


impl Scope {
    pub fn new() -> Scope {
        Scope {
            variables: HashMap::new(),
        }
    }


    /// Return the value contained in the given variable. Prefer variable in this scope,
    /// but can also query parent scope for variable
    /// If for assignment, create the variable instead of returning an error
    pub fn get_variable(&self, name: String, _: &mut SlothProgram) -> Result<Rc<RefCell<Value>>, String> {
        match self.variables.get(&name) {
            Some(v) => Ok(v.clone()),
            None => {
                todo!()
                /* 
                match self.parent.clone() {
                    Some(p) => p.borrow().get_variable(name, program),
                    None => {
                        let error_msg = format!("Called uninitialised variable '{}'", name);
                        Err(error_msg.to_string())
                    }
                }*/
            }
        }
    }


    /// Return whether the given variable is set or not
    pub fn is_set(&self, name: &String) -> bool {
        self.variables.contains_key(name)
    }



    /// Add a new variable to the scope with the given value. Fails if a value with the given id already exists
    pub fn push_variable(&mut self, name: String, value: Rc<RefCell<Value>>) -> Result<(), String> {
        match self.variables.contains_key(&name) {
            true => {
                let error_msg = format!("Variable '{}' already exists", name);
                Err(error_msg.to_string())
            },
            false => {
                self.variables.insert(name, value);
                Ok(())
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