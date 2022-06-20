use crate::sloth::function::SlothFunction;


struct BuiltIn_print {}

impl SlothFunction for BuiltIn_print {
    fn call(&self, args: Vec<Box<dyn crate::sloth::value::Value>>, _: &crate::sloth::scope::Scope) -> Result<Option<Box<dyn crate::sloth::value::Value>>, String> {
        for v in args {
            print!("{}", v.to_string())
        };
        Ok(None)
    }
}