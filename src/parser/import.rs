use std::path::PathBuf;
use regex::Regex;

use crate::lexer::{Token, TokenStream, Keyword, Separator};
use crate::sloth::function::{CustomFunction, FunctionSignature};
use crate::sloth::program::SlothProgram;
use crate::errors::{Error, ErrMsg};
use crate::sloth::statement::Statement;
use crate::sloth::types::Type;




/// Parse an "import" statement, i.e the import of another .slo file. Different from the "builtin" statement which '''imports''' builtin functions and structures
pub fn parse_import(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool, origin_path: PathBuf) -> Result<(), Error> {
    // import keyword
    let (_, first_pos) = super::expect_token(stream, Token::Keyword(Keyword::Import))?;
    
    // name of the file, as a literal (NOT an identifier)
    let filename = match stream.current() {
        Some((Token::Literal(s), p)) => {
            let re = Regex::new(r#"^"(.*)""#).unwrap();
            let file_name = match re.captures(&s) {
                Some(cap) => cap.get(1),
                None => None,
            };

            match file_name {
                Some(v) => v.as_str().to_string(),
                None => {
                    let err_msg = format!("Expected filename, got '{}'", s);
                    return Err(Error::new(ErrMsg::ImportError(err_msg), Some(p)))
                }
            }
        },

        o => return Err(super::wrong_token(o, "file name"))
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