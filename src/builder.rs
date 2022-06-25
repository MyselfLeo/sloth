use crate::sloth::program::SlothProgram;
use crate::tokenizer::TokenizedProgram;
use crate::errors::{Error, ErrorMessage};





struct TokenIterator {
    tokens: TokenizedProgram,
    nb_tokens: usize,
    current: usize,
}

impl TokenIterator {
    pub fn new(tokens: TokenizedProgram)
}








pub fn build(tokens: TokenizedProgram) -> Result<SlothProgram, Error> {
    todo!()
}