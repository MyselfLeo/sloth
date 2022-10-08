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