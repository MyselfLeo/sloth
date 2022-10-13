use crate::builtins::BuiltInImport;
use crate::lexer::{Token, TokenStream, Separator, Keyword};
use crate::sloth::program::SlothProgram;
use crate::errors::{Error, ErrMsg};




/// Parse a 'builtin' statement, return the resulting BuiltInImport
pub fn parse_builtin(stream: &mut TokenStream, _: &mut SlothProgram, warning: bool) -> Result<BuiltInImport, Error> {
    // "builtin" keyword
    let (_, mut pos) = super::expect_token(stream, Token::Keyword(Keyword::Builtin))?;

    // module name
    let module = match stream.current() {
        Some((Token::Identifier(n), p)) => {pos = pos.until(p); n},
        o => return Err(super::wrong_token(o, "module"))
    };

    stream.next();

    // If the next token is ':', the user imports a particular element in this module (not the whole thing)
    let is_particular = super::current_equal(stream, Token::Separator(Separator::Colon))?;

    // imported element name
    let builtin = {
        if is_particular {
            let v = match stream.next() {
                Some((Token::Identifier(n), p)) => {pos = pos.until(p); vec![n]},
                o => return Err(super::wrong_token(o, "function or structure name"))
            };
            stream.next();
            Some(v)
        }
        else {None}
    };

    super::check_semicolon(stream, warning, &pos)?;

    // check import. TODO maybe don't do that here ?
    let import = BuiltInImport::new(module, builtin);

    match import.is_valid() {
        Ok(..) => Ok(import),
        Err(e) => Err(Error::new(ErrMsg::ImportError(e), Some(pos)))
    }
}