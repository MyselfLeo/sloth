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
    let mut iterator = TokenIterator::new(tokens);

    let mut program = SlothProgram::new();


    let define_token = Token::Keyword("define".to_string());
    let structure_token = Token::Keyword("structure".to_string());


    // main building loop, going over each tokens
    loop {
        let token = iterator.current();

        match token {
            None => break,
            Some(v) => {


                if v.0 == &define_token {
                    unimplemented!()
                }
                else if v.0 == &structure_token  {
                    unimplemented!()
                }
                else {
                    let error_msg = format!("Expected function or structure definition, got unexpected token '{}'", v.0.original_string());
                    return Err(Error::new(ErrorMessage::SyntaxError(error_msg), Some(v.1.clone())));
                }

            }
        }
    }



    Ok(program)
}