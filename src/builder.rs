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