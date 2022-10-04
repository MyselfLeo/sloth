use crate::errors::Error;

use self::tokenstream::TokenStream;

pub mod token;
pub mod tokenstream;
pub mod tokeniser;
pub mod separator;
pub mod keyword;



pub const KEYWORDS: [&str; 11] = ["define", "->", "=", "if", "while", "builtin", "for", "new", "import", "structure", "static"];
pub const OPERATORS: [&str; 12] = ["+", "-", "*", "/", "<=", ">=", "==", "<", ">", "&", "?", "!"];                                  // The '<=' and '>=' must be before '<' and '>' so the parsing works
pub const SEPARATORS: [&str; 12] = ["(", ")", "{", "}", "[", "]", ";", ":", ",", "|", ".", "~"];

// Unlike SEPARATORS, those do not have a semantic meaning (only used for separating tokens)
pub const DEFAULT_SEPARATORS: [char; 2] = [' ', '"'];
// Comments starts with this str and ends at the end of the line
pub const COMMENT_CHAR: char = '#';








pub fn get_token_stream(filename: &str) -> Result<TokenStream, Error> {
    
}