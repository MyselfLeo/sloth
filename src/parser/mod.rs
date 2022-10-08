use std::path::PathBuf;

use crate::position::Position;
use crate::lexer::{Token, TokenStream, Separator, Keyword};
use crate::errors::{Error, ErrMsg, Warning};
use crate::sloth::program::SlothProgram;
use crate::sloth::structure::ObjectBlueprint;

mod types;
mod structure;
mod varcall;
mod builtin;
mod expression;
mod literal;
mod list;
mod operation;
mod object_construction;
mod functioncall;
mod bracket;
mod statement;
mod flow_control;
mod statics;
mod function;
mod import;


/*
Each function using the TokenStream to parse a particular token structure must be implemented so:
- they start expecting the TokenStream to be on the FIRST token of their structure
- they end on the first token AFTER their structure
*/




/// Return the EOF error
fn eof_error() -> Error {
    Error::new(ErrMsg::UnexpectedEOF("Unexpected end of file".to_string()), None)
}







/// Generate the error to be raised when an expected token is not correct
pub fn wrong_token(given: Option<(Token, Position)>, expected: &str) -> Error {
    match given {
        Some((t, p)) => {
            let mut err_msg = format!("Expected {expected}, got unexpected ");
            err_msg.push_str(match t {
                Token::Keyword(..) => "keyword",
                Token::Separator(..) => "separator",
                Token::Identifier(..) => "identifier",
                Token::Operator(..) => "operator",
                Token::Literal(..) => "literal",
            });
            err_msg.push_str(&format!(" '{}'", t.original_string()));
            Error::new(ErrMsg::SyntaxError(err_msg), Some(p))
        },
        None => eof_error()
    }
}




/// Raise an error if the current token of the stream is not the required token.
/// Else, return the token and set the stream to the next token
pub fn expect_token(stream: &mut TokenStream, token: Token) -> Result<(Token, Position), Error> {
    match stream.current() {
        Some((t, p)) => {
            if t == token {stream.next(); Ok((t, p))}                                                            // token is correct
            else {Err(wrong_token(Some((t, p)), &format!("'{}'", token.original_string())))}     // token is not correct, generate error msg
        },
        None => Err(eof_error())
    }
}




/// Return whether the current token of the stream is the same as the given token.
/// Raise an error in case of EOF
pub fn current_equal(stream: &mut TokenStream, token: Token) -> Result<bool, Error> {
    match stream.current() {
        Some((t, _)) => Ok(t == token),
        None => Err(eof_error())
    }
}




/// Check if the current token is a semicolon:
/// - if yes, go to the next
/// - if no, doesn't move the stream cursor and warn if requested
pub fn check_semicolon(stream: &mut TokenStream, warn: bool, statement_pos: &Position) -> Result<(), Error> {
    match stream.current() {
        Some((Token::Separator(Separator::SemiColon), _)) => {stream.next(); Ok(())},
        Some(..) => {
            if warn {
                let warning = Warning::new("Use of a semicolon here is highly recommended".to_string(), Some(statement_pos.clone()));
                warning.warn();
            };
            Ok(())
        },
        None => return Err(eof_error())
    }
}



/// Check if the user defines a module (module:)
pub fn module_check(stream: &mut TokenStream) -> Result<Option<(String, Position)>, Error> {
    if let Some((Token::Separator(Separator::Colon), _)) = stream.peek(1) {
        let res = match stream.current() {
            Some((Token::Identifier(n), p)) => Some((n, p)),
            o => return Err(wrong_token(o, "module")),
        };
        // go over the colon
        stream.skip(2);
        Ok(res)
    }
    else {Ok(None)}
}


















/// Parse a whole file, populating the program object
pub fn parse_file(filename: String, program: &mut SlothProgram, warning: bool, is_main: bool) -> Result<(), Error> {
    let mut stream = crate::lexer::get_token_stream(&filename)?;

    let module_name = match is_main {
        true => None,
        false => Some(PathBuf::from(&filename).file_stem().unwrap().to_str().unwrap().to_string()),
    };

    // main building loop, going over each tokens
    loop {
        let token = stream.current();

        match token {
            None => break,

            Some((Token::Keyword(n), p)) => {
                match n {
                    Keyword::Builtin => {
                        let import = builtin::parse_builtin(&mut stream, program, warning)?;
                        program.add_import(import);
                    },
                    Keyword::Import => {
                        import::parse_import(&mut stream, program, warning, filename.clone())?;
                    },
                    Keyword::Static => {
                        statics::parse_static_expr(&mut stream, program, warning)?;
                    },
                    Keyword::Structure => {
                        let structure = structure::parse_structure(&mut stream, program, &module_name, warning)?;
                        let res = program.push_struct(
                            structure.get_signature().name,
                            structure.get_signature().module,
                            Box::new(structure)
                        );

                        // raise warning if the struct is overwritten
                        match res {
                            None => (),
                            Some(w) => Warning::new(w, None).warn()
                        }
                    },
                    Keyword::Define => {
                        let function = function::parse_function(&mut stream, program, &module_name, warning)?;
                        let res = program.push_function(Box::new(function));

                        // raise warning if the function is overwritten
                        match res {
                            None => (),
                            Some(w) => Warning::new(w, None).warn()
                        }
                    },

                    t => {
                        let error_msg = format!("Expected 'builtin', 'import', 'static', 'structure' or 'define', got unexpected keyword '{}'", t.to_string());
                        return Err(Error::new(ErrMsg::SyntaxError(error_msg), Some(p)));
                    }
                }
            },

            o => return Err(wrong_token(o, "keyword"))
        }
    };
    Ok(())
}








/// 
pub fn build_program(filename: String, warning: bool, import_default_builtins: bool) -> Result<SlothProgram, Error> {
    let mut program = SlothProgram::new(PathBuf::from(&filename).file_stem().unwrap().to_str().unwrap().to_string(), import_default_builtins);
    parse_file(filename, &mut program, warning, true)?;

    match program.import_builtins() {
        Ok(()) => (),
        Err(e) => return Err(Error::new(ErrMsg::ImportError(e), None))
    };

    Ok(program)
}