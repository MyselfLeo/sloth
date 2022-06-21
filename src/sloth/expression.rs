use super::value::Value;
use super::operator::Operator;
use super::scope::Scope;
use super::program::SlothProgram;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/// Used by scopes to reference to parent scope in the Scope stack
pub struct ExpressionID {
    id: u64
}
impl ExpressionID {
    pub fn new(value: u64) -> ExpressionID {
        ExpressionID { id: value }
    }
}


/// Expressions are objects that can be evaluated into a value
pub enum Expression {
    Literal(Value),                                                     // value of the literal
    VariableCall(String),                                               // name of the variable
    Operation(Operator, Option<ExpressionID>, Option<ExpressionID>),    // Operator to apply to one or 2 values from the Scope Expression stack (via index)
    FunctionCall(String, Vec<ExpressionID>),                            // name of the function and its list of expressions to be evaluated
}


impl Clone for Expression {
    fn clone(&self) -> Self {
        match self {
            Self::Literal(arg0) => Self::Literal(arg0.clone()),
            Self::VariableCall(arg0) => Self::VariableCall(arg0.clone()),
            Self::Operation(arg0, arg1, arg2) => Self::Operation(arg0.clone(), arg1.clone(), arg2.clone()),
            Self::FunctionCall(arg0, arg1) => Self::FunctionCall(arg0.clone(), arg1.clone()),
        }
    }
}



impl Expression {
    /// Evaluate the expression in the given context (scope and program) and return its value
    pub fn evaluate(&self, scope: &Scope, program: &mut SlothProgram) -> Result<Value, String> {
        match self {
            // return this literal value
            Expression::Literal(v) => Ok(v.clone()),

            // return the value stored in this variable
            Expression::VariableCall(name) => scope.get_variable(name.clone(), program),

            // process the operation and return the result
            Expression::Operation(op, lhs, rhs) => {
                // Get the value of both lhs and rhs
                let lhs = match lhs {
                    None => None,
                    Some(i) => {Some(program.get_expr(*i)?.evaluate(scope, program))}
                };
                let rhs = match rhs {
                    None => None,
                    Some(i) => {Some(program.get_expr(*i)?.evaluate(scope, program))}
                };
                
                // apply op
                //op::apply_op(op, lhs, rhs)

                todo!()
            }

            // return the result of the function call
            Expression::FunctionCall(f_name, param) => {
                // Create a new scope for the execution of the function
                let func_scope_id = program.new_scope(Some(scope.id));
                let func_scope = program.get_scope(func_scope_id)?;

                // Get the function
                let function = program.get_function(f_name.clone())?;



                // Evaluate each given expression in the scope, and create an input variable (@0, @1, etc.) with the set value
                for (i, param_expr_id) in param.iter().enumerate() {
                    let value = program.get_expr(*param_expr_id)?.evaluate(scope, program)?;
                    func_scope.set_variable(format!("@{}", i), value);
                }

                // Create the @return variable, with default value
                let default_value = function.get_output_type().default();
                func_scope.set_variable("@return".to_string(), default_value);
                

                // Execute the function in the scope
                function.call(func_scope, program)?;

                // return the value in the '@return' variable
                Ok(func_scope.get_variable("@return".to_string(), program)?)
            }
        }
    }
}