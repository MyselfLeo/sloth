use crate::position::Position;
use crate::lexer::{Token, TokenStream, Separator};
use crate::errors::{Error, ErrMsg, Warning};

mod types;
mod structure;


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
pub fn expect_token(stream: &mut TokenStream, token: Token) -> Result<(), Error> {
    match stream.current() {
        Some((t, p)) => {
            if t == token {stream.next(); Ok(())}                                                // token is correct
            else {Err(wrong_token(Some((t, p)), &token.original_string()))}     // token is not correct, generate error msg
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
pub fn check_semicolon(stream: &mut TokenStream, warn: bool, statement_pos: Position) -> Result<(), Error> {
    match stream.current() {
        Some((Token::Separator(Separator::SemiColon), _)) => {stream.next(); Ok(())},
        Some(..) => {
            if warn {
                let warning = Warning::new("Use of a semicolon here is highly recommended".to_string(), Some(statement_pos));
                warning.warn();
            };
            Ok(())
        },
        None => return Err(eof_error())
    }
}