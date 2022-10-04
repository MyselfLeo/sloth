use crate::element::ElementPosition;
use crate::lexer::{Token, TokenStream};
use crate::errors::{Error, ErrMsg};



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
pub fn wrong_token(given: Option<(Token, ElementPosition)>, expected: &str) -> Error {
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
/// Else, set the stream to the next token
pub fn expect_token(stream: &mut TokenStream, token: Token) -> Result<(), Error> {
    match stream.current() {
        Some((t, p)) => {
            // token is correct
            if t == token {
                stream.next();
                Ok(())
            }
            // token is not correct, generate error msg
            else {
                Err(wrong_token(Some((t, p)), &token.original_string()))
            }
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