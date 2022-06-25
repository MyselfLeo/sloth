use crate::sloth::program::SlothProgram;
use crate::tokenizer::{TokenizedProgram, Token, ElementPosition};
use crate::errors::{Error, ErrorMessage};





struct TokenIterator {
    tokens: TokenizedProgram,
    nb_tokens: usize,
    current: usize,
}

impl TokenIterator {
    pub fn new(tokens: TokenizedProgram) -> TokenIterator {
        TokenIterator {
            tokens: tokens.clone(),
            nb_tokens: tokens.tokens.len(),
            current: 0
        }
    }

    /// return the nth value of the iterator
    pub fn nth(&self, index: usize) -> Option<(&Token, &ElementPosition)> {
        if index >= self.nb_tokens {None}
        else {
            Some((&self.tokens.tokens[index], &self.tokens.positions[index]))
        }
    }

    /// return current value of the iterator
    pub fn current(&self) -> Option<(&Token, &ElementPosition)> {
        self.nth(self.current)
    }

    /// switch to the next value of the iterator and returns it
    pub fn next(&mut self) -> Option<(&Token, &ElementPosition)> {
        self.current += 1;
        self.nth(self.current)
    }

    /// return the nth next value without switching to it
    pub fn peek(&self, nth: usize) -> Option<(&Token, &ElementPosition)> {
        self.nth(self.current + nth)
    }
}








pub fn build(tokens: TokenizedProgram) -> Result<SlothProgram, Error> {
    todo!()
}