use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::built_in::BuiltInImport;
use crate::sloth::expression::{ExpressionID, Expression};
use crate::sloth::function::{CustomFunction, FunctionSignature};
use crate::sloth::operator::{Operator};
use crate::sloth::program::SlothProgram;
use crate::sloth::statement::{Statement, IdentifierWrapper, IdentifierElement};
use crate::sloth::structure::{StructDefinition, StructSignature};
use crate::sloth::types::Type;
use crate::sloth::value::Value;
use crate::tokenizer::{TokenizedProgram, Token, ElementPosition, Separator, self};
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








/// Parse a list
fn parse_list(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool) -> Result<(Expression, ElementPosition), Error> {

    let starting_pos;
    let last_pos;
    // The starting token must be an open square bracket
    if let Some((Token::Separator(Separator::OpenSquareBracket), p)) = iterator.current() {starting_pos = p;}
    else {panic!("Called parse_list but iterator is not a on an open square bracket")}


    let mut exprs: Vec<ExpressionID> = Vec::new();

    
    iterator.next();


    // Until we meet a closed square bracket, we parse each expressions
    while match iterator.current() {Some((Token::Separator(Separator::CloseSquareBracket), _)) => false, Some(_) => true, None => return Err(eof_error(line!()))} {
        let (expr_id, _) = parse_expression(iterator, program, warning)?;
        exprs.push(expr_id);
    }

    // At this point, the iterator should be on a closed square bracket
    if let Some((Token::Separator(Separator::CloseSquareBracket), p)) = iterator.current() {last_pos = p;}
    else {panic!("parse_list do not finish on a ']'")}

    iterator.next();
    let pos = starting_pos.until(last_pos);

    Ok((Expression::ListInit(exprs, pos.clone()), pos))
}













/// In the case of a ParameterCall or a MethodCall (expr.attribute or expr.method()), this function parses the second part (after the period/)
/// It is given the ExpressionID and ElementPosition of the first expression
fn parse_second_expr(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool, first_expr: (ExpressionID, ElementPosition), is_parenthesied: bool) -> Result<(ExpressionID, ElementPosition), Error> {
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
    let expr = match iterator.peek(1) {
        // method call
        Some((Token::Separator(Separator::OpenParenthesis), _)) | Some((Token::Separator(Separator::Colon), _)) => {
            let function = parse_functioncall(iterator, program, warning)?;
            // Transforms the FunctionCall expression given by the parse_functioncall function into a MethodCall
            if let Expression::FunctionCall(signature, input_exprs, pos) = function {
                let expr_pos = first_expr.1.until(pos);
                let method_call = Expression::MethodCall(first_expr.0, signature, input_exprs, expr_pos.clone());
                (program.push_expr(method_call), expr_pos)
            }
            else {panic!("Function 'parse_functioncall' did not return an Expression::Functioncall value")}
        },
        
        // Parameter call
        Some((_, p)) => {
            let expr_pos = first_expr.1.until(p);
            let param_call = Expression::ParameterCall(first_expr.0, ident, expr_pos.clone());
            iterator.next();
            (program.push_expr(param_call), expr_pos)
        },

        None => return Err(eof_error(line!()))
    };



    // determines whether the expression if finished here or not.
    match iterator.current() {
        Some((Token::Separator(Separator::CloseParenthesis), _)) => {
            if is_parenthesied {iterator.next(); Ok(expr)}
            else {Ok(expr)}
        },
        Some((Token::Separator(Separator::Period), _)) => {
            parse_second_expr(iterator, program, warning, first_expr, is_parenthesied)
        },
        Some((t, p)) => {
            if !is_parenthesied {Ok(expr)}
            else {
                let err_msg = format!("Expected ')', got unexpected token '{}'", t.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)));
            }
        },
        None => Err(eof_error(line!())),
    }



}















/// Parse the construction of an object
fn parse_object_construction(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool) -> Result<(Expression, ElementPosition), Error> {
    iterator.next();

    let mut pos = None;

    let mut module_name: Option<String> = None;
    // If the peek(2) token is a colon, then the module name is given
    if let Some((Token::Separator(Separator::Colon), _)) = iterator.peek(1) {

        match iterator.current() {
            Some((Token::Identifier(n), p)) => {
                pos = Some(p);
                module_name = Some(n)
            },
            Some((t, p)) => {
                let err_msg = format!("Expected module name, got unexpected token '{}'", t.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())))
            },
            None => return Err(eof_error(line!()))
        }
        // go over the colon, set the iterator to the structure name
        iterator.next();
        iterator.next();
    }


    // Next token is the struct's name
    let struct_name = match iterator.current() {
        Some((Token::Identifier(n), p)) => {
            if pos.is_none() {pos = Some(p);}
            n
        },
        Some((t, p)) => {
            let err_msg = format!("Expected structure name, got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())))
        },
        None => return Err(eof_error(line!()))
    };

    // Next is an open parenthesis
    match iterator.next() {
        Some((Token::Separator(Separator::OpenParenthesis), _)) => (),
        Some((t, p)) => {
            let err_msg = format!("Expected '(', got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())))
        },
        None => return Err(eof_error(line!()))
    };


    let mut expr_ids = Vec::new();

    // Next is a sequence of expressions, until a closed parenthesis is met
    iterator.next();
    loop {
        match iterator.current() {
            Some((Token::Separator(Separator::CloseParenthesis), p)) => {
                pos = Some(pos.unwrap().until(p));
                break
            },
            _ => {
                let (expr_id, _) = parse_expression(iterator, program, warning)?;
                expr_ids.push(expr_id);
            }
        };
    }

    iterator.next();

    Ok((Expression::ObjectConstruction(StructSignature::new(module_name, struct_name), expr_ids, pos.clone().unwrap()), pos.unwrap()))
}
























/// Parse an expression, push it to the program's expression stack and return its id
fn parse_expression(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool) -> Result<(ExpressionID, ElementPosition), Error> {
    // If the first token is an open parenthesis, we expect the expression to end on a closed parenthesis.
    let is_parenthesied = match iterator.current() {
        Some((Token::Separator(Separator::OpenParenthesis), _)) => {
            iterator.next();
            true
        }
        _ => false
    };
    


    // we use the first token of the expression to find its type
    let (expr, expr_pos) = match iterator.current() {

        // The expression starts with a Literal, so it's only this literal
        Some((Token::Literal(s), first_position)) => {
            iterator.next();
            (Expression::Literal(Value::from_raw_token(s.clone()), first_position.clone()), first_position.clone())
        },

    
        // The token is an open square bracket. It's the start of a list
        Some((Token::Separator(Separator::OpenSquareBracket), _)) => parse_list(iterator, program, warning)?,



        // The token is an identifier. CHeck the next token to see if its a function call, or variable call
        Some((Token::Identifier(_), _)) =>  {
            match iterator.peek(1) {
                Some((Token::Separator(Separator::OpenParenthesis), _)) | Some((Token::Separator(Separator::Colon), _)) => {
                    let func_call = parse_functioncall(iterator, program, warning)?;
                    if let Expression::FunctionCall(_, _, p) = func_call.clone() {(func_call, p)}
                    else {panic!("parse_functioncall did not return an Expression::FunctionCall enum")}
                },
                _ => {
                    let wrapper = parse_identifierwrapper(iterator, program, warning)?;
                    (Expression::VariableCall(wrapper.0, wrapper.1.clone()), wrapper.1)
                }
            }
        },

        // The token is an operator, so it's an operation
        Some((Token::Operator(_), _)) => {
            let operation = parse_operation(iterator, program, warning)?;
            if let Expression::Operation(_, _, _, p) = operation.clone() {(operation, p)}
            else {panic!("parse_operation did not return an Expression::Operation enum")}
        },



        // The token is the "new" keyword: it's the construction of a struct
        Some((Token::Keyword(n), p)) => {
            match n.as_str() {
                "new" => parse_object_construction(iterator, program, warning)?,
                _ => {
                    let err_msg = format!("Unexpected keyword '{}'", n);
                    return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)))
                }
            }
        },




        Some((t, p)) => {
            let err_msg = format!("Unexpected expression start '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)))
        }

        None => return Err(eof_error(line!()))
    };

    let first_expr = (program.push_expr(expr.clone()), expr_pos);


    // determines whether the expression if finished here or not.
    match iterator.current() {
        Some((Token::Separator(Separator::CloseParenthesis), _)) => {
            if is_parenthesied {iterator.next();}
            
            if let Some((Token::Separator(Separator::Period), _)) = iterator.current() {
                parse_second_expr(iterator, program, warning, first_expr, false)
            }
            else {Ok(first_expr)}
        },
        Some((Token::Separator(Separator::Period), _)) => {
            parse_second_expr(iterator, program, warning, first_expr, is_parenthesied)
        },
        Some((t, p)) => {
            if !is_parenthesied {Ok(first_expr)}
            else {
                let err_msg = format!("Expected ')', got unexpected token '{}'", t.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)));
            }
        },
        None => Err(eof_error(line!())),
    }
}










/// Parse an assignment statement
fn parse_assignment(wrapper: (IdentifierWrapper, ElementPosition), iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool) -> Result<Statement, Error> {
    
    // Get identifier wrapper
    let (id_wrapper, start_pos) = wrapper;
    
    // The next token must be '='
    match iterator.current() {
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

    Ok(Statement::Assignment(id_wrapper, expression_id, assignment_position))
}









/// Parse an identifier chaine, like "var1.field1.field2[value]"
fn parse_identifierwrapper(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool) -> Result<(IdentifierWrapper, ElementPosition), Error> {
    let first_pos;
    let mut last_pos;
    let mut sequence: Vec<IdentifierElement> = Vec::new();


    // first token must be an identifier
    match iterator.current() {
        Some((Token::Identifier(n), p)) => {
            sequence.push(IdentifierElement::Identifier(n));
            first_pos = p.clone();
            last_pos = p;
        },
        Some((t, p)) => {
            let err_msg = format!("Expected identifier, got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)))
        },
        None => return Err(eof_error(line!()))
    }

    // loop until the end of the wrapper. Each step is either a field access (.fieldname) or an indexation ([x])
    loop {

        // If the third next token is an open parenthesis, we can stop here as the rest is not part of the IdentifierWrapper, but is
        // instead a method call.
        match iterator.peek(3) {
            Some((Token::Separator(Separator::OpenParenthesis), _)) => {
                iterator.next();
                break
            },
            _ => ()
        }

        match iterator.next() {
            Some((Token::Separator(Separator::Period), _)) => {
                // next token must be an identifier
                match iterator.next() {
                    Some((Token::Identifier(n), p)) => {
                        sequence.push(IdentifierElement::Identifier(n));
                        last_pos = p;
                    },
                    Some((t, p)) => {
                        let err_msg = format!("Expected identifier, got unexpected token '{}'", t.original_string());
                        return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)))
                    },
                    None => return Err(eof_error(line!()))
                }
            },


            Some((Token::Separator(Separator::OpenSquareBracket), _)) => {
                // next is the expression that will be used as an index
                iterator.next();
                let expr_id = parse_expression(iterator, program, warning)?;
                // next token MUST be a closed parenthesis
                match iterator.current() {
                    Some((Token::Separator(Separator::CloseSquareBracket), p)) => {
                        sequence.push(IdentifierElement::Indexation(expr_id.0));
                        last_pos = p;
                    },
                    Some((t, p)) => {
                        let err_msg = format!("Expected ']', got unexpected token '{}'", t.original_string());
                        return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)))
                    },
                    None => return Err(eof_error(line!()))
                }
            },

            Some((_, _)) => {break;},
            None => return Err(eof_error(line!()))
        };
    };

    Ok((IdentifierWrapper::new(sequence), first_pos.until(last_pos)))
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
                    // Check if it's a simple function call
                    if next_token.original_string() == ":".to_string() || next_token.original_string() == "(".to_string() {
                        let func_call = parse_functioncall(iterator, program, warning)?;
                        Statement::ExpressionCall(program.push_expr(func_call.clone()), func_call.get_pos())
                    }

                    else {
                        // At this point we can parse the wrapper
                        let wrapper = parse_identifierwrapper(iterator, program, warning)?;
                        
                        match iterator.current() {
                            Some((token, _)) => {
                                // Assignment
                                if token.original_string() == "=".to_string() {
                                    parse_assignment(wrapper, iterator, program, warning)?
                                }

                                // Expression call
                                else {
                                    Statement::ExpressionCall(program.push_expr(Expression::VariableCall(wrapper.0, wrapper.1.clone())), wrapper.1)
                                }
                            },

                            None => return Err(eof_error(line!()))
                        }
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










fn parse_type(iterator: &mut TokenIterator, program: &mut SlothProgram, module_name: &Option<String>, warning: bool) -> Result<(Type, ElementPosition), Error> {
    let first_pos;
    let mut last_pos;

    let first_type_name = match iterator.current() {
        Some((Token::Identifier(n), p)) => {
            first_pos = p.clone();
            last_pos = p;
            n
        },
        Some((t, p)) => {
            let err_msg = format!("Expected type, got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())))
        },
        None => return Err(eof_error(line!()))
    };

    let return_type = match first_type_name.as_str() {
        "any" => Type::Any,
        "num" => Type::Number,
        "bool" => Type::Boolean,
        "string" => Type::String,
        "list" => {
            // parse the list type
            match iterator.next() {
                Some((Token::Separator(Separator::OpenSquareBracket), _)) => (),
                Some((t, p)) => {
                    let err_msg = format!("Expected '[', got unexpected token '{}'", t.original_string());
                    return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())))
                },
                None => return Err(eof_error(line!()))
            };

            iterator.next();
            let (list_type, _) = parse_type(iterator, program, module_name, warning)?;
            
            match iterator.current() {
                Some((Token::Separator(Separator::CloseSquareBracket), p)) => last_pos = p,
                Some((t, p)) => {
                    let err_msg = format!("Expected ']', got unexpected token '{}'", t.original_string());
                    return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())))
                },
                None => return Err(eof_error(line!()))
            };

            Type::List(Box::new(list_type))
        },
        _ => {Type::Struct(first_type_name)}
    };


    iterator.next();

    Ok((return_type, first_pos.until(last_pos)))
}






















/// Parse a function
fn parse_function(iterator: &mut TokenIterator, program: &mut SlothProgram, module_name: &Option<String>, warning: bool) -> Result<(), Error> {
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
                iterator.next();

                // next token must be the type name
                Some(parse_type(iterator, program, module_name, warning)?.0)
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

    iterator.next();

    while match iterator.current() {
        Some((Token::Keyword(kw), _)) => {kw != "->".to_string()},
        Some(_) => true,
        None => return Err(eof_error(line!())),
    } {
        input_types.push(parse_type(iterator, program, module_name, warning)?.0)
    }


    // The next token must be '->'
    match iterator.current() {
        Some((t, p)) => {
            if t.original_string() != "->".to_string() {
                let err_msg = format!("Expected '->', got '{}'", t.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p)));
            }
        },
        None => return Err(eof_error(line!())),
    }


    // The next token is the return value
    iterator.next();
    let (output_type, _) = parse_type(iterator, program, module_name, warning)?;


    // next token must be an open bracket
    let next = iterator.current();
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



    // At this stage, if the function is named "main" and module_name is Some(...), we don't push the function
    // as an imported module can't have a main function. Raise a warning

    if !(f_name == "main" && module_name.is_some()) {
         // Create the function and push it to the program
        let function = CustomFunction {
            signature: FunctionSignature::new(
                module_name.clone(),
                f_name.clone(),
                owner_type,
                Some(input_types),
                Some(output_type)
            ),

            instructions: statements
        };
        program.push_function(Box::new(function));
    }
    else if warning {
        let warn = Warning::new(format!("Ignoring 'main' function of imported module '{}'. You may want to remove it", module_name.clone().unwrap()), None);
        warn.warn()
    }


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















/// Parse a structure definition, push it to the program
fn parse_structure_def(iterator: &mut TokenIterator, program: &mut SlothProgram, module_name: &Option<String>, warning: bool) -> Result<(), Error> {
    // must start with the "structure" keyword
    match iterator.current() {
        Some((t, p)) => {
            if t.original_string() != "structure".to_string() {
                let err_msg = format!("Expected 'structure' keyword, got '{}'", t.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())));
            }
        }
        None => return Err(eof_error(line!())),
    };


    let mut definition_pos;


    // Next is the name of the structure. It must be an identifier
    let struct_name = match iterator.next() {
        Some((Token::Identifier(n), p)) => {
            definition_pos = p;
            n
        },
        Some((t, p)) => {
            let err_msg = format!("Expected structure name (an identifier), got '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())));
        },
        None => return Err(eof_error(line!())),
    };


    // Next is an open bracket
    match iterator.next() {
        Some((Token::Separator(Separator::OpenBracket), p)) => {
            definition_pos = definition_pos.until(p);
        },
        Some((t, p)) => {
            let err_msg = format!("Expected '{{', got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())));
        },
        None => return Err(eof_error(line!())),
    }


    let mut fields_name: Vec<String> = Vec::new();
    let mut fields_types: Vec<Box<Type>> = Vec::new();

    iterator.next();

    // Next is each fields of this structure, until we met a closed bracket
    loop {
        match iterator.current() {
            None => return Err(eof_error(line!())),
            Some((Token::Separator(Separator::CloseBracket), _)) => {
                iterator.next();
                break
            },

            // name of the field, as an identifier
            Some((Token::Identifier(name), first_pos)) => {

                // check that the name is not already used
                if fields_name.contains(&name) {
                    let err_msg = format!("The name '{}' is already used for a field of the structure '{}'", name, struct_name);
                    return Err(Error::new(ErrorMessage::DefinitionError(err_msg), Some(first_pos.clone())))
                }


                fields_name.push(name);

                
                // next token must be a colon
                match iterator.next() {
                    Some((Token::Separator(Separator::Colon), _)) => (),
                    Some((t, p)) => {
                        let err_msg = format!("Expected ':', got unexpected token '{}'", t.original_string());
                        return Err(Error::new(ErrorMessage::DefinitionError(err_msg), Some(p.clone())))
                    }
                    None => return Err(eof_error(line!()))
                }

                // the type of the field
                iterator.next();
                let (field_type, type_pos) = parse_type(iterator, program, module_name, warning)?;
                fields_types.push(Box::new(field_type));


                // A semicolon here is strongly recommended, but not necessary
                match iterator.current() {
                    Some((Token::Separator(Separator::SemiColon), _)) => {iterator.next();},
                    Some((_, _)) => {
                        if warning {
                            let warning = Warning::new("Use of a semicolon at the end of each field definition is highly recommended".to_string(), Some(first_pos.until(type_pos)));
                            warning.warn();
                        }
                    },
                    None => return Err(eof_error(line!()))
                }
            },

            Some((t, p)) => {
                let err_msg = format!("Expected field name or '}}', got unexpected token '{}'", t.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())))
            }
        }
    }

    match program.push_struct(StructDefinition::new(struct_name, fields_name, fields_types, None), module_name.clone()) {
        // warning raised by the program
        Some(w) => {
            if warning {
                let warning = Warning::new(w, Some(definition_pos));
                warning.warn();
            }
        },
        None => ()
    };

    Ok(())
}








/// Parse an "import" statement, i.e the import of another .slo file. Different from the "builtin" statement which '''imports''' builtin functions and structures
fn parse_import(iterator: &mut TokenIterator, program: &mut SlothProgram, warning: bool, origin_path: PathBuf) -> Result<(), Error> {
    let first_pos = match iterator.current() {
        Some((_, p)) => p.clone(),
        None => return Err(eof_error(line!()))
    };

    // Next token is a literal string with the name of the file to import
    let (path, last_pos) = match iterator.next() {
        Some((Token::Literal(s), p)) => {
            // get path from literal
            match Value::from_raw_token(s.clone()) {
                Value::String(_) => {
                    // Cleans the literal, as it will have ' " 'before and after
                    let mut name = s.strip_prefix("\"").unwrap();
                    name = name.strip_suffix("\"").unwrap();

                    let working_dir = origin_path.parent().unwrap().to_path_buf();

                    let mut file = working_dir.clone();
                    file.push(name);

                    if origin_path == file {
                        let err_msg = format!("File '{}' imports itself", iterator.current().unwrap().1.filename);
                        return Err(Error::new(ErrorMessage::ImportError(err_msg), Some(p.clone())))
                    }

                    (file, p)
                },
                _ => {
                    let err_msg = format!("Expected filename, got unexpected token '{}'", s);
                    return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())))
                }
            }
        },
        Some((t, p)) => {
            let err_msg = format!("Expected filename, got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())))
        },
        None => return Err(eof_error(line!()))
    };

    // parse the file for the program
    parse_file(path, program, warning, false)?;


    // A semicolon here is strongly recommended, but not necessary
    match iterator.next() {
        Some((Token::Separator(Separator::SemiColon), _)) => {iterator.next();},
        Some((_, _)) => {
            if warning {
                let warning = Warning::new("Use of a semicolon at the end of each field definition is highly recommended".to_string(), Some(first_pos.until(last_pos)));
                warning.warn();
            }
        },
        None => return Err(eof_error(line!()))
    }

    Ok(())
}











/// Parse a whole file, populating the program object
pub fn parse_file(filename: PathBuf, program: &mut SlothProgram, warning: bool, is_main: bool) -> Result<(), Error> {
    let tokens = tokenizer::TokenizedProgram::from_file(filename.to_str().unwrap())?;
    let mut iterator = TokenIterator::new(tokens);

    let module_name = match is_main {
        true => None,
        false => Some(filename.file_stem().unwrap().to_str().unwrap().to_string()),
    };

    // main building loop, going over each tokens
    loop {
        let token = iterator.current();

        match token {
            None => break,
            Some(v) => {
                if v.0.original_string() == "define".to_string() {
                    parse_function(&mut iterator, program, &module_name, warning)?;
                }
                else if v.0.original_string() == "builtin".to_string() {
                    parse_builtin(&mut iterator, program, warning)?;
                }
                else if v.0.original_string() == "structure".to_string()  {
                    parse_structure_def(&mut iterator, program, &module_name, warning)?;
                }
                else if v.0.original_string() == "import".to_string() {
                    parse_import(&mut iterator, program, warning, filename.clone())?;
                }
                else {
                    let error_msg = format!("Expected function or structure definition, got unexpected token '{}'", v.0.original_string());
                    return Err(Error::new(ErrorMessage::SyntaxError(error_msg), Some(v.1.clone())));
                }

            }
        }
    };

    Ok(())
}











pub fn from(filename: String, warning: bool, import_default_builtins: bool) -> Result<SlothProgram, Error> {
    let path = PathBuf::from(&filename);
    let mut program = SlothProgram::new(path.file_stem().unwrap().to_str().unwrap().to_string(), import_default_builtins);
    parse_file(path, &mut program, warning, true)?;

    match program.import_builtins() {
        Ok(()) => (),
        Err(e) => return Err(Error::new(ErrorMessage::ImportError(e), None))
    };

    Ok(program)
}