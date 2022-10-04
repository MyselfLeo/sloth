use std::path::PathBuf;

use crate::built_in::BuiltInImport;
use crate::sloth::expression::{ExpressionID, Expression};
use crate::sloth::function::{CustomFunction, FunctionSignature};
use crate::sloth::operator::{Operator};
use crate::sloth::program::SlothProgram;
use crate::sloth::statement::{Statement};
use crate::sloth::structure::{CustomDefinition, StructSignature};
use crate::sloth::types::Type;
use crate::sloth::value::Value;
use crate::element::ElementPosition;
use crate::lexer::{Token, TokenStream, Keyword, Separator, get_token_stream};
use crate::errors::{Error, ErrMsg, Warning};




fn eof_error(i: u32) -> Error {
    Error::new(ErrMsg::UnexpectedEOF(format!("{} ({})",  "Unexpected End Of File", i)), None)
}










/// Parse a variable call
fn parse_variablecall(iterator: &mut TokenStream, _: &mut SlothProgram, _: bool) -> Result<Expression, Error> {
    // Get the identifier
    match iterator.current() {
        Some((Token::Identifier(n), p)) => {
            iterator.next();
            Ok(Expression::VariableAccess(None, n, p))
        }
        Some((t, p)) => {
            let err_msg = format!("Expected variable name, got unexpected token '{}'", t.original_string());
            Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())))
        },
        None => Err(eof_error(line!()))
    }
}









/// Parse a function call
fn parse_functioncall(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Expression, Error> {
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
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)))
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
                    return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(position.clone())));
                }
            }
            None => return Err(eof_error(line!()))
        };


        // Next token is the function's name
        match iterator.next() {
            Some((Token::Identifier(s), _)) => function_name = s,
            Some((t, p)) => {
                let err_msg = format!("Expected function name, got unexpected token '{}'", t.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())));
            },
            None => return Err(eof_error(line!()))
        };
    }


    // Next token must be an open parenthesis
    match iterator.next() {
        Some((token, position)) => {
            if token != Token::Separator(Separator::OpenParenthesis) {
                let err_msg = format!("Expected '(', got unexpected token '{}'", token.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(position.clone())));
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
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(position.clone())));
            }
        }
        None => return Err(eof_error(line!()))
    };

    let functioncall_pos = start_pos.until(last_pos);

    let func_id = FunctionSignature::new(module, function_name, None, None, None);

    iterator.next();
    Ok(Expression::FunctionCall(None, func_id, inputs_expr_id, functioncall_pos))
}




/// Parse an operation
fn parse_operation(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Expression, Error> {

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
                    return Err(Error::new(ErrMsg::OperationErrror(err_msg), Some(first_pos)))
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
fn parse_list(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<(Expression, ElementPosition), Error> {

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








/// Parse expression[access]
fn parse_bracket_access(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool, first_expr: (ExpressionID, ElementPosition), is_parenthesied: bool) -> Result<(ExpressionID, ElementPosition), Error> {
    // must start on an open square bracket
    match iterator.current() {
        Some((Token::Separator(Separator::OpenSquareBracket), _)) => (),
        _ => panic!("parse_bracket_access called but not on a [")
    };

    // next expression is the field identifier
    iterator.next();
    let access = parse_expression(iterator, program, warning)?;

    // next must be a closing bracket
    let final_pos = match iterator.current() {
        Some((Token::Separator(Separator::CloseSquareBracket), p)) => p,
        Some((t, p)) => {
            let err_msg = format!("Expected ']', got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())))
        },
        None => return Err(eof_error(line!()))
    };
    iterator.next();

    let expr_pos = first_expr.1.until(final_pos);
    let expr_id = program.push_expr(Expression::BracketAcces(first_expr.0, access.0, expr_pos.clone()));

    let expr = (expr_id, expr_pos);

    // determines whether the expression if finished here or not.
    match iterator.current() {
        Some((Token::Separator(Separator::CloseParenthesis), _)) => {
            if is_parenthesied {iterator.next(); Ok(expr)}
            else {Ok(expr)}
        },
        Some((Token::Separator(Separator::OpenSquareBracket), _)) => {
            parse_bracket_access(iterator, program, warning, expr, is_parenthesied)
        },
        Some((Token::Separator(Separator::Period), _)) => {
            parse_second_expr(iterator, program, warning, expr, is_parenthesied)
        },
        Some((t, p)) => {
            if !is_parenthesied {Ok(expr)}
            else {
                let err_msg = format!("Expected ')', got unexpected token '{}'", t.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)));
            }
        },
        None => Err(eof_error(line!())),
    }
}











/// In the case of a MethodCall (expr.method()), this function parses the second part (after the period)
/// It is given the ExpressionID and ElementPosition of the first expression
fn parse_second_expr(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool, first_expr: (ExpressionID, ElementPosition), is_parenthesied: bool) -> Result<(ExpressionID, ElementPosition), Error> {
    // name of the variable or function to use
    let (ident, ident_pos) = match iterator.next() {
        Some((Token::Identifier(n), p)) => (n, p),
        Some((t, p)) => {
            let err_msg = format!("Expected identifier, got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)));
        },
        None => return Err(eof_error(line!()))
    };



    // Check whether the call is a method call or a field access
    let expr = match iterator.peek(1) {
        // method call
        Some((Token::Separator(Separator::OpenParenthesis), _)) | Some((Token::Separator(Separator::Colon), _)) => {
            let function = parse_functioncall(iterator, program, warning)?;
            // Transforms the FunctionCall expression given by the parse_functioncall function into a MethodCall
            if let Expression::FunctionCall(_, signature, input_exprs, pos) = function {
                let expr_pos = first_expr.1.until(pos);
                let method_call = Expression::FunctionCall(Some(first_expr.0), signature, input_exprs, expr_pos.clone());
                (program.push_expr(method_call), expr_pos)
            }
            else {panic!("Function 'parse_functioncall' did not return an Expression::Functioncall value")}
        },
        
        // Field
        Some((_, _)) => {
            iterator.next();
            let expr_pos = first_expr.1.until(ident_pos);

            let field_access = Expression::VariableAccess(Some(first_expr.0), ident, expr_pos.clone());
            (program.push_expr(field_access), expr_pos)
        },

        None => return Err(eof_error(line!()))
    };



    // determines whether the expression if finished here or not.
    match iterator.current() {
        Some((Token::Separator(Separator::CloseParenthesis), _)) => {
            if is_parenthesied {iterator.next(); Ok(expr)}
            else {Ok(expr)}
        },
        Some((Token::Separator(Separator::OpenSquareBracket), _)) => {
            parse_bracket_access(iterator, program, warning, expr, is_parenthesied)
        },
        Some((Token::Separator(Separator::Period), _)) => {
            parse_second_expr(iterator, program, warning, expr, is_parenthesied)
        },
        Some((t, p)) => {
            if !is_parenthesied {Ok(expr)}
            else {
                let err_msg = format!("Expected ')', got unexpected token '{}'", t.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)));
            }
        },
        None => Err(eof_error(line!())),
    }



}















/// Parse the construction of an object
fn parse_object_construction(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<(Expression, ElementPosition), Error> {
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
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())))
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
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())))
        },
        None => return Err(eof_error(line!()))
    };

    // Next is an open parenthesis
    match iterator.next() {
        Some((Token::Separator(Separator::OpenParenthesis), _)) => (),
        Some((t, p)) => {
            let err_msg = format!("Expected '(', got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())))
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
fn parse_expression(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<(ExpressionID, ElementPosition), Error> {
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



        // The token is an identifier. Check the next token to see if its a function call, or field access
        Some((Token::Identifier(_), _)) =>  {
            match iterator.peek(1) {
                Some((Token::Separator(Separator::OpenParenthesis), _)) | Some((Token::Separator(Separator::Colon), _)) => {
                    let func_call = parse_functioncall(iterator, program, warning)?;
                    if let Expression::FunctionCall(_, _, _, p) = func_call.clone() {(func_call, p)}
                    else {panic!("parse_functioncall did not return an Expression::FunctionCall enum")}
                },
                _ => {
                    let var_call = parse_variablecall(iterator, program, warning)?;
                    if let Expression::VariableAccess(_, _, p) = var_call.clone() {(var_call, p)}
                    else {panic!("pare_variablecall did not return an Expression::VariableAccess")}
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
        Some((Token::Keyword(Keyword::New), _)) => {
            parse_object_construction(iterator, program, warning)?
        },




        Some((t, p)) => {
            let err_msg = format!("Unexpected expression start '{}'", t.original_string());
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)))
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
        Some((Token::Separator(Separator::OpenSquareBracket), _)) => {
            parse_bracket_access(iterator, program, warning, first_expr, is_parenthesied)
        },
        Some((t, p)) => {
            if !is_parenthesied {Ok(first_expr)}
            else {
                let err_msg = format!("Expected ')', got unexpected token '{}'", t.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)));
            }
        },
        None => Err(eof_error(line!())),
    }
}










/// Parse an assignment statement
fn parse_assignment(left_expr: (ExpressionID, ElementPosition), iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Statement, Error> {
    
    let (left_expr, left_pos) = left_expr;
    
    // The next token must be '='
    match iterator.current() {
        Some((token, position)) => {
            if token.original_string() != "=".to_string() {
                let err_msg = format!("Expected '=', got unexpected token '{}'", token.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(position.clone())));
            }
        }
        None => return Err(eof_error(line!()))
    };

    // Rest of the assignment is an expression
    iterator.next();
    let (expression_id, expr_pos) = parse_expression(iterator, program, warning)?;
    let assignment_position = left_pos.until(expr_pos);

    Ok(Statement::Assignment(left_expr, expression_id, assignment_position))
}













/// Parse and return a Statement from the iterator.
/// Each statement SHOULD end with a semicolon. However the way the syntax works makes them unecessary, so not
/// putting them will raise a warning
fn parse_statement(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Statement, Error> {

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
                        // Get the expression of the left part
                        let expr = parse_expression(iterator, program, warning)?;
                        
                        if iterator.current().is_none() {return Err(eof_error(line!()))}

                        // assignment or expr call
                        if let Some((Token::Keyword(Keyword::Equal), _)) = iterator.current() {
                            parse_assignment(expr, iterator, program, warning)?
                        }
                        else {
                            Statement::ExpressionCall(expr.0, expr.1)
                        }
                    }
                },

                None => return Err(eof_error(line!()))
            }
        },

        Some((Token::Keyword(Keyword::If), _)) => parse_if(iterator, program, warning)?,
        Some((Token::Keyword(Keyword::While), _)) => parse_while(iterator, program, warning)?,


        Some((t, p)) => {
            let err_msg = format!("Unexpected token '{}'. Outside a function, you can only define structures or functions", t.original_string());
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)))
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















fn parse_if(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Statement, Error> {
    let first_pos;
    let last_pos;

    // first token must be 'if'. parse_if should however only be called in a way so that it's true
    if let Some((Token::Keyword(Keyword::If), p)) = iterator.current() {first_pos = p}
    else {panic!("Called parse_if but iterator is not a on if statement")}

    iterator.next();

    let (cond_expr_id, _) = parse_expression(iterator, program, warning)?;
    let current = iterator.current();

    // next token must be a '{'
    if let Some((Token::Separator(Separator::OpenBracket), p)) = current {
        last_pos = p;
    }
    else if let Some((t, p)) = current {
        let err_msg = format!("Expected '{{', got unexpected token '{}'", t.original_string());
        return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)));
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







fn parse_while(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Statement, Error> {
    let first_pos;
    let last_pos;

    // first token must be 'while'. parse_while should however only be called in a way so that it's true
    if let Some((Token::Keyword(Keyword::While), p)) = iterator.current() {first_pos = p}
    else {panic!("Called parse_while but iterator is not a on while statement")}

    iterator.next();

    let (cond_expr_id, _) = parse_expression(iterator, program, warning)?;
    let current = iterator.current();

    // next token must be a '{'
    if let Some((Token::Separator(Separator::OpenBracket), p)) = current {
        last_pos = p;
    }
    else if let Some((t, p)) = current {
        let err_msg = format!("Expected '{{', got unexpected token '{}'", t.original_string());
        return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)));
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










fn parse_type(iterator: &mut TokenStream, program: &mut SlothProgram, module_name: &Option<String>, warning: bool) -> Result<(Type, ElementPosition), Error> {
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
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())))
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
                    return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())))
                },
                None => return Err(eof_error(line!()))
            };

            iterator.next();
            let (list_type, _) = parse_type(iterator, program, module_name, warning)?;
            
            match iterator.current() {
                Some((Token::Separator(Separator::CloseSquareBracket), p)) => last_pos = p,
                Some((t, p)) => {
                    let err_msg = format!("Expected ']', got unexpected token '{}'", t.original_string());
                    return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())))
                },
                None => return Err(eof_error(line!()))
            };

            Type::List(Box::new(list_type))
        },
        _ => {Type::Object(first_type_name)}
    };


    iterator.next();

    Ok((return_type, first_pos.until(last_pos)))
}






















/// Parse a function
fn parse_function(iterator: &mut TokenStream, program: &mut SlothProgram, module_name: &Option<String>, warning: bool) -> Result<(), Error> {
    // must start with the "define" keyword
    match iterator.current() {
        Some((t, p)) => {
            if t.original_string() != "define".to_string() {
                let err_msg = format!("Expected 'define' keyword, got '{}'", t.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)));
            }
        }
        None => return Err(eof_error(line!())),
    };


    // Next token must be the name of the function
    let f_name = match iterator.next() {
        Some((Token::Identifier(s), _)) => s.clone(),
        Some((t, p)) => {
            let err_msg = format!("Expected function name, got '{}'", t.original_string());
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)));
        }
        None => return Err(eof_error(line!())),
    };



    // If the next token is "for", the function is a method of a given type
    let owner_type = match iterator.peek(1) {
        Some((Token::Keyword(kw), _)) => {
            if kw == Keyword::For {
                iterator.next();
                iterator.next();

                // next token must be the type name
                Some(parse_type(iterator, program, module_name, warning)?.0)
            }
            else {iterator.next(); None}
        },
        _ => {iterator.next(); None}
    };





    // Next token must be a colon
    match iterator.current() {
        Some(t) => {
            if let (Token::Separator(Separator::Colon), _) = t {}
            else {
                let err_msg = format!("Expected ':', got '{}'", t.0.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(t.1.clone())));
            }
        },
        None => return Err(eof_error(line!())),
    }


    // Parse the input types of the function
    let mut input_types: Vec<(Type, bool)> = Vec::new();

    iterator.next();

    while match iterator.current() {
        Some((Token::Keyword(Keyword::LeftArrow), _)) => false,
        Some(_) => true,
        None => return Err(eof_error(line!())),
    } {
        let mut referenced = false;
        if let Some((Token::Separator(Separator::Tilde), _)) = iterator.current() {
            referenced = true;
            iterator.next();
        }
        input_types.push((parse_type(iterator, program, module_name, warning)?.0, referenced))
    }


    // The next token must be '->'
    match iterator.current() {
        Some((t, p)) => {
            if t.original_string() != "->".to_string() {
                let err_msg = format!("Expected '->', got '{}'", t.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)));
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
        return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)));
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
fn parse_builtin(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<(), Error> {
    let first_pos;

    // must start with the "builtin" keyword
    match iterator.current() {
        Some((t, p)) => {
            first_pos = p.clone();
            if t.original_string() != "builtin".to_string() {
                let err_msg = format!("Expected 'builtin' keyword, got '{}'", t.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())));
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
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())));
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
                Err(e) => {return Err(Error::new(ErrMsg::ImportError(e), Some(pos)));}
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
                        Err(e) => {return Err(Error::new(ErrMsg::ImportError(e), Some(pos)));}
                    };

                    iterator.next();
                },

                Some((t, p)) => {
                    let err_msg = format!("Expected built-in name, got unexpected token '{}'", t.original_string());
                    return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p)));
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
                Err(e) => {return Err(Error::new(ErrMsg::ImportError(e), Some(first_pos.until(p))));}
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
fn parse_structure_def(iterator: &mut TokenStream, program: &mut SlothProgram, module_name: &Option<String>, warning: bool) -> Result<(), Error> {
    // must start with the "structure" keyword
    match iterator.current() {
        Some((t, p)) => {
            if t.original_string() != "structure".to_string() {
                let err_msg = format!("Expected 'structure' keyword, got '{}'", t.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())));
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
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())));
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
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())));
        },
        None => return Err(eof_error(line!())),
    }


    let mut struct_fields: (Vec<String>, Vec<Type>) = (Vec::new(), Vec::new());

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
            Some((Token::Identifier(field_name), first_pos)) => {

                // check that the name is not already used
                if struct_fields.0.contains(&field_name) {
                    let err_msg = format!("The name '{}' is already used for a field of the structure '{}'", field_name, struct_name);
                    return Err(Error::new(ErrMsg::DefinitionError(err_msg), Some(first_pos.clone())))
                }



                
                // next token must be a colon
                match iterator.next() {
                    Some((Token::Separator(Separator::Colon), _)) => (),
                    Some((t, p)) => {
                        let err_msg = format!("Expected ':', got unexpected token '{}'", t.original_string());
                        return Err(Error::new(ErrMsg::DefinitionError(err_msg), Some(p.clone())))
                    }
                    None => return Err(eof_error(line!()))
                }

                // the type of the field
                iterator.next();
                let (field_type, type_pos) = parse_type(iterator, program, module_name, warning)?;

                struct_fields.0.push(field_name);
                struct_fields.1.push(field_type);

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
                return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())))
            }
        }
    }

    let signature = StructSignature::new(module_name.clone(), struct_name.clone());

    let mut fields: Vec<(String, Type)> = Vec::new();

    for (n, t) in std::iter::zip(struct_fields.0, struct_fields.1) {
        fields.push((n, t));
    }


    let definition = CustomDefinition::new(signature, fields);


    match program.push_struct(struct_name, module_name.clone(), Box::new(definition)) {
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
fn parse_import(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool, origin_path: PathBuf) -> Result<(), Error> {
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
                        return Err(Error::new(ErrMsg::ImportError(err_msg), Some(p.clone())))
                    }

                    (file, p)
                },
                _ => {
                    let err_msg = format!("Expected filename, got unexpected token '{}'", s);
                    return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())))
                }
            }
        },
        Some((t, p)) => {
            let err_msg = format!("Expected filename, got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())))
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







/// Parse a static expression definition
fn parse_static_expr(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<(), Error> {
    // position of the 'static' keyword
    let first_pos = match iterator.current() {
        Some((_, p)) => p,
        None => return Err(eof_error(line!()))
    };

    // the static's name
    let (name, _) = match iterator.next() {
        Some((Token::Identifier(n), p)) => {
            // raise a warning if the identifier is not in full caps
            if warning && n.to_uppercase() != n {
                let warn_msg = format!("It is recommended to set in full uppercase the name of static expressions ('{}')", n.to_uppercase());
                Warning::new(warn_msg, Some(p.clone())).warn()
            }

            (n, p)
        },
        Some((t, p)) => {
            let err_msg = format!("Expected static name, got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())));
        },
        None => return Err(eof_error(line!()))
    };



    // next should be a =
    match iterator.next() {
        Some((Token::Keyword(Keyword::Equal), _)) => (),
        Some((t, p)) => {
            let err_msg = format!("Expected '=', got unexpected token '{}'", t.original_string());
            return Err(Error::new(ErrMsg::SyntaxError(err_msg), Some(p.clone())));
        },
        None => return Err(eof_error(line!()))
    }


    // next is the expression
    iterator.next();
    let (expr, expr_pos) = parse_expression(iterator, program, warning)?;
    let full_pos = first_pos.until(expr_pos);


    // A semicolon here is strongly recommended, but not necessary
    match iterator.current() {
        Some((Token::Separator(Separator::SemiColon), _)) => {iterator.next();},
        Some((_, _)) => {
            if warning {
                let warning = Warning::new("Using semicolons at the end of statements is highly recommended".to_string(), Some(full_pos.clone()));
                warning.warn();
            }
        },
        None => return Err(eof_error(line!()))
    }

    // add the expression to the program's statics
    match program.push_static(&name, expr) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(ErrMsg::RuntimeError(e), Some(full_pos))),
    }
}








/// Parse a whole file, populating the program object
pub fn parse_file(filename: PathBuf, program: &mut SlothProgram, warning: bool, is_main: bool) -> Result<(), Error> {
    let mut iterator = get_token_stream(&filename.to_string_lossy())?;

    let module_name = match is_main {
        true => None,
        false => Some(filename.file_stem().unwrap().to_str().unwrap().to_string()),
    };

    // main building loop, going over each tokens
    loop {
        let token = iterator.current();

        match token {
            None => break,

            Some((Token::Keyword(n), p)) => {
                match n {
                    Keyword::Builtin => parse_builtin(&mut iterator, program, warning)?,
                    Keyword::Import => parse_import(&mut iterator, program, warning, filename.clone())?,
                    Keyword::Static => parse_static_expr(&mut iterator, program, warning)?,
                    Keyword::Structure => parse_structure_def(&mut iterator, program, &module_name, warning)?,
                    Keyword::Define => parse_function(&mut iterator, program, &module_name, warning)?,

                    t => {
                        let error_msg = format!("Expected 'builtin', 'import', 'static', 'structure' or 'define', got unexpected keyword '{}'", t.to_string());
                        return Err(Error::new(ErrMsg::SyntaxError(error_msg), Some(p)));
                    }
                }
            },

            Some((v, p)) => {
                let error_msg = format!("Expected keyword, got unexpected token '{}'", v.original_string());
                return Err(Error::new(ErrMsg::SyntaxError(error_msg), Some(p)));
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
        Err(e) => return Err(Error::new(ErrMsg::ImportError(e), None))
    };

    Ok(program)
}