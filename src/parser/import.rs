use regex::Regex;

use crate::lexer::{Token, TokenStream, Keyword};
use crate::sloth::program::SlothProgram;
use crate::errors::{Error, ErrMsg};


/// Parse an "import" statement, i.e the import of another .slo file. Different from the "builtin" statement which '''imports''' builtin functions and structures
pub fn parse_import(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool, _: String) -> Result<(), Error> {
    println!("parsing import");
    // import keyword
    let (_, first_pos) = super::expect_token(stream, Token::Keyword(Keyword::Import))?;
    
    // name of the file, as a literal (NOT an identifier)
    let (filename, last_pos) = match stream.current() {
        Some((Token::Literal(s), p)) => {
            let re = Regex::new(r#"^"(.*)""#).unwrap();
            let file_name = match re.captures(&s) {
                Some(cap) => cap.get(1),
                None => None,
            };

            match file_name {
                Some(v) => (v.as_str().to_string(), p),
                None => {
                    let err_msg = format!("Expected filename, got '{}'", s);
                    return Err(Error::new(ErrMsg::ImportError(err_msg), Some(p)))
                }
            }
        },

        o => return Err(super::wrong_token(o, "file name"))
    };

    // parse the file for the program
    super::parse_file(filename, program, warning, false)?;

    super::check_semicolon(stream, warning, &first_pos.until(last_pos))?;

    Ok(())
}