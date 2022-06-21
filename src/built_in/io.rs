use crate::sloth::function::SlothFunction;
use crate::sloth::scope::Scope;
use crate::sloth::value::Value;



const FUNCTIONS: [&str; 1] = [
    "print",
];


const STRUCTS: [&str; 0] = [
];





struct BuiltinIoPrint {}

impl SlothFunction for BuiltinIoPrint {
    fn get_name(&self) -> String {
        "print".to_string()
    }
    fn call(&self, args: Vec<Value>, _: &Scope) -> Result<Option<Value>, String> {
        for v in args {
            print!("{}", v.to_string())
        };
        Ok(None)
    }
}