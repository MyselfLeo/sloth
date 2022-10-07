use crate::lexer::{Token, TokenStream, Keyword, Separator};
use crate::sloth::function::{CustomFunction, FunctionSignature};
use crate::sloth::program::SlothProgram;
use crate::errors::Error;
use crate::sloth::statement::Statement;
use crate::sloth::types::Type;




/// Parse an "import" statement, i.e the import of another .slo file. Different from the "builtin" statement which '''imports''' builtin functions and structures
pub fn parse_import(iterator: &mut TokenStream, program: &mut SlothProgram, warning: bool, origin_path: PathBuf) -> Result<(), Error> {
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