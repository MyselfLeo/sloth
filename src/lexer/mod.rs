use crate::errors::Error;

pub mod token;
pub mod tokenstream;
pub mod tokeniser;
pub mod separator;
pub mod keyword;
pub mod operator;

pub use tokenstream::TokenStream;
pub use token::Token;
pub use separator::Separator;
pub use keyword::Keyword;
pub use operator::Operator;


// Unlike SEPARATORS, those do not have a semantic meaning (only used for separating tokens)
pub const DEFAULT_SEPARATORS: [char; 3] = [' ', '"', '\t'];

// Comments starts with this str and ends at the end of the line
pub const COMMENT_STR: [char; 2] = ['/', '/'];







/// Generate a TokenStream from the given file
pub fn get_token_stream(filename: &str) -> Result<TokenStream, Error> {
    let tokens = tokeniser::from_file(filename)?;
    let length = tokens.len();

    let stream = TokenStream::new(
        filename.to_string(),
        tokens,
        length,
        0
    );

    Ok(stream)
}