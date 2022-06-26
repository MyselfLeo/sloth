use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::types::Type;
use crate::tokenizer::{TokenizedProgram, Token, ElementPosition, Separator};
use crate::errors::{Error, ErrorMessage};



const UNEXPECTED_EOF_ERR_MSG: &str = "Unexpected End Of File";





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










/// Parse a function and add it to the program's function stack
fn parse_function(iterator: &mut TokenIterator, program: &mut SlothProgram) -> Result<(), Error> {
    // must start with the "define" keyword
    match iterator.next() {
        Some((t, p)) => {

            if let Token::Keyword(s) = t {
                
            }

            let err_msg = format!("Expected 'define' keyword, got '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())));
        }
        None => return Err(Error::new(ErrorMessage::UnexpectedEOF(UNEXPECTED_EOF_ERR_MSG.to_string()), None)),
    };


    // Next token must be the name of the function
    let f_name = match iterator.next() {
        Some((&Token::Identifier(s), _)) => s.clone(),
        Some((t, p)) => {
            let err_msg = format!("Expected function name, got '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())));
        }
        None => return Err(Error::new(ErrorMessage::UnexpectedEOF(UNEXPECTED_EOF_ERR_MSG.to_string()), None)),
    };


    // Next token must be a colon
    match iterator.next() {
        Some(t) => {
            if let (&Token::Separator(Separator::Colon), _) = t {}
            else {
                let err_msg = format!("Expected ':', got '{}'", t.0.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(t.1.clone())));
            }
        },
        None => return Err(Error::new(ErrorMessage::UnexpectedEOF(UNEXPECTED_EOF_ERR_MSG.to_string()), None)),
    }



    // Parse the input types of the function
    let mut input_types: Vec<Type> = Vec::new();

    while match iterator.peek(1) {
        Some((Token::Keyword(kw), _)) => {kw != &"->".to_string()},
        Some(_) => true,
        None => return Err(Error::new(ErrorMessage::UnexpectedEOF(UNEXPECTED_EOF_ERR_MSG.to_string()), None)),
    } {
        // Check the value of the keyword
        match iterator.next() {
            Some(e) => match e.0.original_string().as_str() {
                "num" => input_types.push(Type::Number),
                "bool" => input_types.push(Type::Boolean),
                "string" => input_types.push(Type::String),
                s => {
                    let err_msg = format!("Unexpected input type '{}'", s);
                    return Err(Error::new(ErrorMessage::TypeError(err_msg), Some(e.1.clone())));
                }
            }

            // This should not happen as it's already checked with peek(). But just in case
            None => return Err(Error::new(ErrorMessage::UnexpectedEOF(UNEXPECTED_EOF_ERR_MSG.to_string()), None))
        }

    }


    // The next token must be '->'
    match iterator.next() {
        Some(t) => {
            if let (&LIGHT_ARROW_TOKEN, _) = t {}
            else {
                let err_msg = format!("Expected '->', got '{}'", t.0.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(t.1.clone())));
            }
        },
        None => return Err(Error::new(ErrorMessage::UnexpectedEOF(UNEXPECTED_EOF_ERR_MSG.to_string()), None)),
    }


    // The next token is the return value
    let output_type = match iterator.next() {
        Some(e) => match e.0.original_string().as_str() {
            "num" => Type::Number,
            "bool" => Type::Boolean,
            "string" => Type::String,
            s => {
                let err_msg = format!("Unexpected output type '{}'", s);
                return Err(Error::new(ErrorMessage::TypeError(err_msg), Some(e.1.clone())));
            }
        }
        None => return Err(Error::new(ErrorMessage::UnexpectedEOF(UNEXPECTED_EOF_ERR_MSG.to_string()), None))
    };


    // next token must be an open bracket
    let next = iterator.next();
    if let Some((&Token::Separator(Separator::OpenBracket), _)) = next {}
    else if let Some((t, p)) = next {
        let err_msg = format!("Expected '{{', got unexpected token '{}'", t.original_string());
        return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())));
    }


    // Now we need to parse each statements
    unimplemented!()
}















pub fn build(tokens: TokenizedProgram) -> Result<SlothProgram, Error> {
    let mut iterator = TokenIterator::new(tokens);

    let mut program = SlothProgram::new();


    


    // main building loop, going over each tokens
    loop {
        let token = iterator.current();

        match token {
            None => break,
            Some(v) => {


                if v.0.original_string() == "define".to_string() {
                    let function = parse_function(&mut iterator, &mut program);
                }
                else if v.0.original_string() == "structure".to_string()  {
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