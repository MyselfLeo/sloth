use std::path::PathBuf;

use crate::builtins::BuiltInImport;
use crate::sloth::expression::{ExpressionID, Expression};
use crate::sloth::function::{CustomFunction, FunctionSignature};
use crate::sloth::operator::{Operator};
use crate::sloth::program::SlothProgram;
use crate::sloth::statement::{Statement};
use crate::sloth::structure::{CustomDefinition, StructSignature};
use crate::sloth::types::Type;
use crate::sloth::value::Value;
use crate::position::Position;
use crate::lexer::{Token, TokenStream, Keyword, Separator, get_token_stream};
use crate::errors::{Error, ErrMsg, Warning};



































/// In the case of a MethodCall (expr.method()), this function parses the second part (after the period)
/// It is given the ExpressionID and Position of the first expression
fn parse_second_expr(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool, first_expr: (ExpressionID, Position), is_parenthesied: bool) -> Result<(ExpressionID, Position), Error> {
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





































/// Parse an expression, push it to the program's expression stack and return its id
fn parse_expression(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<(ExpressionID, Position), Error> {
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