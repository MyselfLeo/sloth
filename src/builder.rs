use crate::built_in::{BuiltInImport, self};
use crate::sloth::expression::{ExpressionID, Expression};
use crate::sloth::function::{CustomFunction, FunctionSignature};
use crate::sloth::operator::{Operator};
use crate::sloth::program::SlothProgram;
use crate::sloth::statement::Statement;
use crate::sloth::types::Type;
use crate::sloth::value::Value;
use crate::tokenizer::{TokenizedProgram, Token, ElementPosition, Separator};
use crate::errors::{Error, ErrorMessage, Warning};
use regex::Regex;


const UNEXPECTED_EOF_ERR_MSG: &str = "Unexpected End Of File";





fn eof_error(i: u32) -> Error {
    Error::new(ErrorMessage::UnexpectedEOF(format!("{} ({})", UNEXPECTED_EOF_ERR_MSG, i)), None)
}







struct TokenIterator {
    tokens: TokenizedProgram,
    nb_tokens: usize,
    current: usize,
}

impl TokenIterator {
    pub fn new(tokens: TokenizedProgram) -> TokenIterator {
        TokenIterator {
            tokens: tokens.clone(),
            nb_tokens: tokens.tokens.len(),
            current: 0
        }
    }

    /// return the nth value of the iterator
    pub fn nth(&self, index: usize) -> Option<(Token, ElementPosition)> {
        if index >= self.nb_tokens {None}
        else {
            Some((self.tokens.tokens[index].clone(), self.tokens.positions[index].clone()))
        }
    }

    /// return current value of the iterator
    pub fn current(&self) -> Option<(Token, ElementPosition)> {
        self.nth(self.current)
    }

    /// switch to the next value of the iterator and returns it
    pub fn next(&mut self) -> Option<(Token, ElementPosition)> {
        self.current += 1;
        self.nth(self.current)
    }

    /// return the nth next value without switching to it
    pub fn peek(&mut self, nth: isize) -> Option<(Token, ElementPosition)> {
        if nth < 0 {self.nth(self.current - (-nth as usize))}
        else {self.nth(self.current + nth as usize)}
    }
}






















/// Parse a function call
fn parse_functioncall(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool) -> Result<Expression, Error> {
    let mut module: Option<String> = None;
    let mut function_name= String::new();

    let start_pos;

    // Get the first identifier. It can either be a module name (followed by a colon) or the name of the function (followed by a '(')
    let current = iterator.current().clone();
    if let Some((Token::Identifier(s), pos)) = current {
        start_pos = pos;

        match iterator.peek(1) {
            Some((Token::Separator(Separator::OpenParenthesis), _)) => function_name = s,
            Some((Token::Separator(Separator::Colon), _)) => module = Some(s),
            Some((t, p)) => {
                let err_msg = format!("Expected '(' or ':', got unexpected token '{}'", t.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)))
            },
            _ => return Err(eof_error(line!()))
        };

    }
    else {panic!("Function 'parse_functioncall' called but token iterator is not on a function call.")}



    if module.is_some() {

        // Next token must be ":"
        match iterator.next() {
            Some((token, position)) => {
                if token != Token::Separator(Separator::Colon) {
                    let err_msg = format!("Expected ':', got unexpected token '{}'", token.original_string());
                    return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(position.clone())));
                }
            }
            None => return Err(eof_error(line!()))
        };


        // Next token is the function's name
        match iterator.next() {
            Some((Token::Identifier(s), _)) => function_name = s,
            Some((t, p)) => {
                let err_msg = format!("Expected function name, got unexpected token '{}'", t.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())));
            },
            None => return Err(eof_error(line!()))
        };
    }


    // Next token must be an open parenthesis
    match iterator.next() {
        Some((token, position)) => {
            if token != Token::Separator(Separator::OpenParenthesis) {
                let err_msg = format!("Expected '(', got unexpected token '{}'", token.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(position.clone())));
            }
        }
        None => return Err(eof_error(line!()))
    };
    iterator.next();

    // Now, we parse expressions until we reach a closed parenthesis
    let mut inputs_expr_id: Vec<ExpressionID> = Vec::new();

    while match iterator.current() {
        Some((Token::Separator(Separator::CloseParenthesis), _)) => false,
        Some(_) => true,
        None => return Err(eof_error(line!()))
    } {
        inputs_expr_id.push(parse_expression(iterator, program, warning)?.0);
    };


    // Next token must be ')'
    let last_pos;
    match iterator.current() {
        Some((token, position)) => {
            last_pos = position.clone();
            if token != Token::Separator(Separator::CloseParenthesis) {
                let err_msg = format!("Expected ')', got unexpected token '{}'", token.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(position.clone())));
            }
        }
        None => return Err(eof_error(line!()))
    };

    let functioncall_pos = start_pos.until(last_pos);

    let func_id = FunctionSignature::new(module, function_name, None, None, None);

    iterator.next();
    Ok(Expression::FunctionCall(func_id, inputs_expr_id, functioncall_pos))
}




/// Parse an operation
fn parse_operation(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool) -> Result<Expression, Error> {

    // The starting token must be an operator
    let (operator, first_pos) = match iterator.current() {
        Some((Token::Operator(s), p)) => (s, p),
        _ => panic!("parse_operation called but iterator is not starting on an operator")
    };


    // determine the number of operands
    let mut nb_operands = 2;

    let operator = match operator.as_str() {
        // 2 elements operators
        "+" => Operator::Add,
        "-" => Operator::Sub,
        "*" => Operator::Mul,
        "/" => Operator::Div,
        "==" => Operator::Eq,
        ">" => Operator::Gr,
        "<" => Operator::Lw,
        ">=" => Operator::Ge,
        "<=" => Operator::Le,
        "?" => Operator::Or,
        "&" => Operator::And,

        // 1 element operators
        _ => {
            nb_operands = 1;
            match operator.as_str() {
                "!" => Operator::Inv,
                t => {
                    let err_msg = format!("Unimplemented operator {}", t);
                    return Err(Error::new(ErrorMessage::OperationErrror(err_msg), Some(first_pos)))
                }
            }
        }
    };

    // get the first and potential second expression
    iterator.next();
    let (first_expr_id, mut last_pos) = parse_expression(iterator, program, warning)?;

    // Get second expression, if needed
    let second_expr_id = match nb_operands > 1 {
        true => {
            let (expr_id, pos) = parse_expression(iterator, program, warning)?;
            last_pos = pos;
            Some(expr_id)
        }, 
        false => None,
    };

    let op_pos = first_pos.until(last_pos);
    Ok(Expression::Operation(operator, Some(first_expr_id), second_expr_id, op_pos))
}








/// In the case of a ParameterCall or a MethodCall (expr.attribute or expr.method()), this function parses the second part (after the period)
/// It is given the ExpressionID and ElementPosition of the first expression
fn parse_second_expr(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool, first_expr: (ExpressionID, ElementPosition)) -> Result<(ExpressionID, ElementPosition), Error> {
    // name of the variable or function to use
    let ident = match iterator.next() {
        Some((Token::Identifier(s), _)) => s,
        Some((t, p)) => {
            let err_msg = format!("Expected identifier, got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)));
        },
        None => return Err(eof_error(line!()))
    };



    // Check whether the call is a method call or a parameter call
    match iterator.peek(1) {
        // method call
        Some((Token::Separator(Separator::OpenParenthesis), _)) | Some((Token::Separator(Separator::Colon), _)) => {
            let function = parse_functioncall(iterator, program, warning)?;
            // Transforms the FunctionCall expression given by the parse_functioncall function into a MethodCall
            if let Expression::FunctionCall(signature, input_exprs, pos) = function {
                let expr_pos = first_expr.1.until(pos);
                let method_call = Expression::MethodCall(first_expr.0, signature, input_exprs, expr_pos.clone());
                return Ok((program.push_expr(method_call), expr_pos));
            }
            else {panic!("Function 'parse_functioncall' did not return an Expression::Functioncall value")}
        },
        
        // Parameter call
        Some((t, p)) => {
            let expr_pos = first_expr.1.until(p);
            let param_call = Expression::ParameterCall(first_expr.0, ident, expr_pos.clone());
            return Ok((program.push_expr(param_call), expr_pos));
        },

        None => return Err(eof_error(line!()))
    }



}






















/// Parse an expression, push it to the program's expression stack and return its id
fn parse_expression(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool) -> Result<(ExpressionID, ElementPosition), Error> {
    // we use the first token of the expression to find its type
    let (expr, expr_pos) = match iterator.current() {

        // The expression starts with a Literal, so it's only this literal
        Some((Token::Literal(s), first_position)) => {
            iterator.next();
            (Expression::Literal(Value::from_raw_token(s.clone()), first_position.clone()), first_position.clone())
        },

        // TODO: lists

        // The token is an identifier. CHeck the next token to see if its a function call, a variable call or a list access (lists not implemented yet)
        Some((Token::Identifier(s), first_position)) =>  {
            match iterator.peek(1) {
                Some((Token::Separator(Separator::OpenParenthesis), _)) | Some((Token::Separator(Separator::Colon), _)) => {
                    let func_call = parse_functioncall(iterator, program, warning)?;
                    if let Expression::FunctionCall(_, _, p) = func_call.clone() {(func_call, p)}
                    else {panic!("parse_functioncall did not return an Expression::FunctionCall enum")}
                },
                _ => {
                    iterator.next();
                    (Expression::VariableCall(s.clone(), first_position.clone()), first_position.clone())
                }
            }
        },

        // The token is an operator, so it's an operation
        Some((Token::Operator(_), _)) => {
            let operation = parse_operation(iterator, program, warning)?;
            if let Expression::Operation(_, _, _, p) = operation.clone() {(operation, p)}
            else {panic!("parse_operation did not return an Expression::Operation enum")}
        },

        Some((t, p)) => {
            let err_msg = format!("Unexpected expression start '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)))
        }

        None => return Err(eof_error(line!()))
    };



    let first_expr = (program.push_expr(expr), expr_pos);


    // If after parsing the expression, the iterator is on a Separator::Period, the expression is in fact not finished here.
    // It is a variable call or a method call on the result of that expression/the value stored in the variable forming this expression
    match iterator.current() {
        Some((Token::Separator(Separator::Period), _)) => parse_second_expr(iterator, program, warning, first_expr),
        None => Err(eof_error(line!())),
        _ => Ok(first_expr)
    }
}










/// Parse an assignment statement
fn parse_assignment(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool) -> Result<Statement, Error> {
    let var_name: String;
    let start_pos: ElementPosition;

    // Get variable name and starting position of the assignment
     match iterator.current() {
        Some((token, position)) => {
            // Get the name of the variable to be assigned to
            if let Token::Identifier(s) = token {
                var_name = s.clone();
                start_pos = position.clone();
            }
            else {
                let err_msg = format!("Expected variable name, got unexpected token '{}'", token.original_string());
                return Err(Error::new(ErrorMessage::InvalidIdentifier(err_msg), Some(position.clone())));
            };
        },
        None => return Err(eof_error(line!()))
    };
    

    // The next token must be '='
    match iterator.next() {
        Some((token, position)) => {
            if token.original_string() != "=".to_string() {
                let err_msg = format!("Expected '=', got unexpected token '{}'", token.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(position.clone())));
            }
        }
        None => return Err(eof_error(line!()))
    };

    // Rest of the assignment is an expression
    iterator.next();
    let (expression_id, expr_pos) = parse_expression(iterator, program, warning)?;
    let assignment_position = start_pos.until(expr_pos);
    Ok(Statement::Assignment(var_name, expression_id, assignment_position))
}












/// Parse and return a Statement from the iterator.
/// Each statement SHOULD end with a semicolon. However the way the syntax works makes them unecessary, so not
/// putting them will raise a warning
fn parse_statement(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool) -> Result<Statement, Error> {

    let statement = match iterator.current() {
        // Variable assignment or expression call. We'll need the next token to find out
        Some((Token::Identifier(_), _)) => {
            match iterator.peek(1) {
                Some((next_token, _)) => {
                    // This is a variable assignment (IDENTIFIER = EXPRESSION;)
                    if next_token.original_string() == "=".to_string() {
                        parse_assignment(iterator, program, warning)?
                    }

                    // This must be an expression call
                    else {
                        let (expr_id, pos) = parse_expression(iterator, program, warning)?;
                        Statement::ExpressionCall(expr_id, pos)
                    }
                },

                None => return Err(eof_error(line!()))
            }
        },


        Some((Token::Keyword(s), p)) => match s.as_str() {
            "if" => parse_if(iterator, program, warning)?,
            "while" => parse_while(iterator, program, warning)?,
            kw => {
                let err_msg = format!("Unexpected keyword '{}'. Outside a function, you can only define structures or functions", kw);
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)))
            }
        }

        Some((t, p)) => {
            let err_msg = format!("Unexpected token '{}'. Outside a function, you can only define structures or functions", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)))
        }

        None => return Err(eof_error(line!()))
    };



    // Check for the presence of a semicolon (;)
    match iterator.current() {
        Some((Token::Separator(Separator::SemiColon), _)) => {
            // semicolon is here, we can pass it
            iterator.next();
        },
        Some((_, _)) => {
            if warning {
                let warning = Warning::new("Using semicolons at the end of statements is highly recommended".to_string(), Some(statement.get_pos()));
                warning.warn();
            }
        }
        None => {
            return Err(eof_error(line!()))
        },
    };

    Ok(statement)
}















fn parse_if(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool) -> Result<Statement, Error> {
    let first_pos;
    let last_pos;

    // first token must be 'if'. parse_if should however only be called in a way so that it's true
    if let Some((Token::Keyword(s), p)) = iterator.current() {
        first_pos = p;
        if s != "if".to_string() {panic!("Called parse_if but iterator is not a on if statement")}
    } else {panic!("Called parse_if but iterator is not a on if statement")}

    iterator.next();

    let (cond_expr_id, _) = parse_expression(iterator, program, warning)?;
    let current = iterator.current();

    // next token must be a '{'
    if let Some((Token::Separator(Separator::OpenBracket), p)) = current {
        last_pos = p;
    }
    else if let Some((t, p)) = current {
        let err_msg = format!("Expected '{{', got unexpected token '{}'", t.original_string());
        return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)));
    }
    else {return Err(eof_error(line!()))}

    iterator.next();

    // parse the succession of statements until a closed bracket is reached
    let mut statements: Vec<Statement> = Vec::new();

    while match iterator.current() {
        Some((Token::Separator(Separator::CloseBracket), _)) => false,
        Some(_) => true,
        None => return Err(eof_error(line!()))
    } {
        statements.push(parse_statement(iterator, program, warning)?);
    };

    iterator.next();


    Ok(Statement::If(cond_expr_id, statements, first_pos.until(last_pos)))
    
}







fn parse_while(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool) -> Result<Statement, Error> {
    let first_pos;
    let last_pos;

    // first token must be 'while'. parse_while should however only be called in a way so that it's true
    if let Some((Token::Keyword(s), p)) = iterator.current() {
        first_pos = p;
        if s != "while".to_string() {panic!("Called parse_while but iterator is not a on if statement")}
    } else {panic!("Called parse_while but iterator is not a on if statement")}

    iterator.next();

    let (cond_expr_id, _) = parse_expression(iterator, program, warning)?;
    let current = iterator.current();

    // next token must be a '{'
    if let Some((Token::Separator(Separator::OpenBracket), p)) = current {
        last_pos = p;
    }
    else if let Some((t, p)) = current {
        let err_msg = format!("Expected '{{', got unexpected token '{}'", t.original_string());
        return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)));
    }
    else {return Err(eof_error(line!()))}

    iterator.next();

    // parse the succession of statements until a closed bracket is reached
    let mut statements: Vec<Statement> = Vec::new();

    while match iterator.current() {
        Some((Token::Separator(Separator::CloseBracket), _)) => false,
        Some(_) => true,
        None => return Err(eof_error(line!()))
    } {
        statements.push(parse_statement(iterator, program, warning)?);
    };

    iterator.next();

    Ok(Statement::While(cond_expr_id, statements, first_pos.until(last_pos)))
}

















fn get_type_from_str(str: &str) -> Result<Type, String> {
    match str {
        "num" => Ok(Type::Number),
        "bool" => Ok(Type::Boolean),
        "string" => Ok(Type::String),
        s => {
            let identifier_re = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)$").unwrap();
            if identifier_re.is_match(s) {Ok(Type::Struct(s.to_string()))}
            else {Err(format!("Expected structure name, got '{}'", s))}
        }
    }
}














/// Parse a function and add it to the program's function stack
fn parse_function(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool) -> Result<(), Error> {
    // must start with the "define" keyword
    match iterator.current() {
        Some((t, p)) => {
            if t.original_string() != "define".to_string() {
                let err_msg = format!("Expected 'define' keyword, got '{}'", t.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)));
            }
        }
        None => return Err(eof_error(line!())),
    };


    // Next token must be the name of the function
    let f_name = match iterator.next() {
        Some((Token::Identifier(s), _)) => s.clone(),
        Some((t, p)) => {
            let err_msg = format!("Expected function name, got '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)));
        }
        None => return Err(eof_error(line!())),
    };



    // If the next token is "for", the function is a method of a given type
    let owner_type = match iterator.peek(1) {
        Some((Token::Keyword(kw), _)) => {
            if kw == "for".to_string() {
                iterator.next();

                // next token must be the type name
                Some(match iterator.next() {
                    Some((t, p)) => match get_type_from_str(&t.original_string()) {
                        Ok(t) => t,
                        Err(s) => return Err(Error::new(ErrorMessage::TypeError(s), Some(p)))
                    },

                    None => return Err(eof_error(line!())),
                })
            }
            else {None}
        },
        _ => None
    };





    // Next token must be a colon
    match iterator.next() {
        Some(t) => {
            if let (Token::Separator(Separator::Colon), _) = t {}
            else {
                let err_msg = format!("Expected ':', got '{}'", t.0.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(t.1.clone())));
            }
        },
        None => return Err(eof_error(line!())),
    }


    // Parse the input types of the function
    let mut input_types: Vec<Type> = Vec::new();

    while match iterator.peek(1) {
        Some((Token::Keyword(kw), _)) => {kw != "->".to_string()},
        Some(_) => true,
        None => return Err(eof_error(line!())),
    } {
        // Check the value of the keyword
        match iterator.next() {
            Some((t, p)) => match get_type_from_str(t.original_string().as_str()) {
                Ok(t) => input_types.push(t),
                Err(e) => return Err(Error::new(ErrorMessage::TypeError(e), Some(p)))
            },
            // This should not happen as it's already checked with peek(). But just in case
            None => return Err(eof_error(line!()))
        }

    }


    // The next token must be '->'
    match iterator.next() {
        Some((t, p)) => {
            if t.original_string() != "->".to_string() {
                let err_msg = format!("Expected '->', got '{}'", t.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)));
            }
        },
        None => return Err(eof_error(line!())),
    }


    // The next token is the return value
    let output_type = match iterator.next() {
        Some((t, p)) => match get_type_from_str(t.original_string().as_str()) {
            Ok(t) => t,
            Err(e) => return Err(Error::new(ErrorMessage::TypeError(e), Some(p)))
        },
        None => return Err(eof_error(line!()))
    };


    // next token must be an open bracket
    let next = iterator.next();
    if let Some((Token::Separator(Separator::OpenBracket), _)) = next {}
    else if let Some((t, p)) = next {
        let err_msg = format!("Expected '{{', got unexpected token '{}'", t.original_string());
        return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)));
    }
    else {return Err(eof_error(line!()))}



    // Now we need to parse each statements until we reach a closed bracket
    let mut statements: Vec<Statement> = Vec::new();

    iterator.next();
    while match iterator.current() {
        Some((Token::Separator(Separator::CloseBracket), _)) => false,
        Some(_) => true,
        None => return Err(eof_error(line!()))
    } {
        statements.push(parse_statement(iterator, program, warning)?);
    };


    // Create the function and push it to the program
    let function = CustomFunction {
        signature: FunctionSignature::new(
            None,
            f_name.clone(),
            owner_type,
            Some(input_types),
            Some(output_type)
        ),

        instructions: statements
    };
    program.push_function(Box::new(function));


    iterator.next();
    Ok(())
}









/// Parse a "builtin" statement and add the requested import to the program's list of imports.
fn parse_builtin(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool) -> Result<(), Error> {
    let first_pos;

    // must start with the "builtin" keyword
    match iterator.current() {
        Some((t, p)) => {
            first_pos = p.clone();
            if t.original_string() != "builtin".to_string() {
                let err_msg = format!("Expected 'builtin' keyword, got '{}'", t.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())));
            }
        }
        None => return Err(eof_error(line!())),
    };

    let submodule: String;

    //Next is the name of the submodule
    match iterator.next() {
        Some((Token::Identifier(s), _)) => {submodule = s},
        Some((t, p)) => {
            let err_msg = format!("Expected builtin submodule name, got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())));
        },
        None => return Err(eof_error(line!()))
    }


    // At this point, there is 2 possible tokens: ':' if one or more names are specified, or ';' if the user import everything from the submodule
    let next = iterator.next().clone();
    match next {
        Some((Token::Separator(Separator::SemiColon), p)) => {
            let pos = first_pos.until(p);
            let import = BuiltInImport::new(submodule, None);

            match import.is_valid() {
                Ok(()) => {
                    program.add_import(import);
                },
                Err(e) => {return Err(Error::new(ErrorMessage::ImportError(e), Some(pos)));}
            };
        },

        Some((Token::Separator(Separator::Colon), _)) => {
            let mut builtins: Vec<String> = Vec::new();

            // Next token is the name of the built in to import
            match iterator.next() {
                Some((Token::Identifier(n), p)) => {
                    builtins.push(n);

                    let pos = first_pos.until(p);
                    let import = BuiltInImport::new(submodule, Some(builtins));

                    match import.is_valid() {
                        Ok(()) => {
                            program.add_import(import);
                        },
                        Err(e) => {return Err(Error::new(ErrorMessage::ImportError(e), Some(pos)));}
                    };

                    iterator.next();
                },

                Some((t, p)) => {
                    let err_msg = format!("Expected built-in name, got unexpected token '{}'", t.original_string());
                    return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)));
                },

                None => return Err(eof_error(line!()))
            }
        },

        Some((_, p)) => {
            let import = BuiltInImport::new(submodule, None);

            match import.is_valid() {
                Ok(()) => {
                    program.add_import(import);
                },
                Err(e) => {return Err(Error::new(ErrorMessage::ImportError(e), Some(first_pos.until(p))));}
            };
        },

        None => return Err(eof_error(line!()))
    };


    // Check for the presence of a semicolon (;)
    match iterator.current() {
        Some((Token::Separator(Separator::SemiColon), _)) => {
            // semicolon is here, we can pass it
            iterator.next();
            Ok(())
        },
        Some((_, _)) => {
            if warning {
                let (_, last_token_pos) = iterator.peek(-1).unwrap();
                let warning = Warning::new("Using semicolons at the end of statements is highly recommended".to_string(), Some(first_pos.until(last_token_pos)));
                warning.warn();
            }
            Ok(())
        }
        None => {
            Err(eof_error(line!()))
        },
    }
}














pub fn build(tokens: TokenizedProgram, warning: bool) -> Result<SlothProgram, Error> {
    let filename = tokens.filename.clone();
    let mut iterator = TokenIterator::new(tokens);

    let mut program = SlothProgram::new(filename);


    


    // main building loop, going over each tokens
    loop {
        let token = iterator.current();

        match token {
            None => break,
            Some(v) => {
                if v.0.original_string() == "define".to_string() {
                    parse_function(&mut iterator, &mut program, warning)?;
                }
                else if v.0.original_string() == "builtin".to_string() {
                    parse_builtin(&mut iterator, &mut program, warning)?;
                }
                else if v.0.original_string() == "structure".to_string()  {
                    unimplemented!()
                }
                else {
                    let error_msg = format!("Expected function or structure definition, got unexpected token '{}'", v.0.original_string());
                    return Err(Error::new(ErrorMessage::SyntaxError(error_msg), Some(v.1.clone())));
                }

            }
        }
    }

    match program.import_builtins() {
        Ok(()) => Ok(program),
        Err(e) => Err(Error::new(ErrorMessage::ImportError(e), None))
    }
}