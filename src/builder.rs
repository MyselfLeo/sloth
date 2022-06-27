use std::iter;

use crate::sloth::expression::{ExpressionID, Expression};
use crate::sloth::function::SlothFunction;
use crate::sloth::program::SlothProgram;
use crate::sloth::statement::Statement;
use crate::sloth::types::Type;
use crate::sloth::value::Value;
use crate::tokenizer::{TokenizedProgram, Token, ElementPosition, Separator};
use crate::errors::{Error, ErrorMessage};



const UNEXPECTED_EOF_ERR_MSG: &str = "Unexpected End Of File";





fn eof_error() -> Error {
    Error::new(ErrorMessage::UnexpectedEOF(UNEXPECTED_EOF_ERR_MSG.to_string()), None)
}







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
    pub fn peek(&mut self, nth: usize) -> Option<(&Token, &ElementPosition)> {
        self.nth(self.current + nth)
    }
}






















/// Parse a function call
fn parse_functioncall(iterator: &mut TokenIterator, program: &mut SlothProgram) -> Result<Expression, Error> {
    todo!()
}




/// Parse an operation
fn parse_operation(iterator: &mut TokenIterator, program: &mut SlothProgram) -> Result<Expression, Error> {
    todo!()
}






/// Parse an expression, push it to the program's expression stack and return its id
fn parse_expression(iterator: &mut TokenIterator, program: &mut SlothProgram) -> Result<ExpressionID, Error> {

    // we use the first token of the expression to find its type
    let expr = match iterator.current() {

        // The expression starts with a Literal, so it's only this literal
        Some((Token::Literal(s), first_position)) => {
            Expression::Literal(Value::from_string(s.clone()), first_position.clone())
        },

        // TODO: lists

        // The token is an identifier. CHeck the next token to see if its a function call, a variable call or a list access (lists not implemented yet)
        Some((Token::Identifier(s), first_position)) =>  {
            match iterator.peek(1) {
                Some((Token::Separator(Separator::OpenParenthesis), _)) => parse_functioncall(iterator, program)?,
                _ => Expression::VariableCall(s.clone(), first_position.clone())
            }
        },

        // The token is an operator, so it's an operation
        Some((Token::Operator(_), _)) => parse_operation(iterator, program)?,

        Some((t, p)) => {
            let err_msg = format!("Unexpected expression start '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())))
        }

        None => return Err(eof_error())
    };


    Ok(program.push_expr(expr))
}










/// Parse an assignment statement
fn parse_assignment(iterator: &mut TokenIterator, program: &mut SlothProgram) -> Result<Statement, Error> {
    let var_name: String;
    let start_pos: ElementPosition;

    // Get variable name and starting position of the assignment
     match iterator.current() {
        Some((token, position)) => {
            // Get the name of the variable to be assigned to
            if let Token::Identifier(s) = token {
                var_name = s.clone();
                start_pos = position.clone();
            }
            else {
                let err_msg = format!("Expected variable name, got unexpected token '{}'", token.original_string());
                return Err(Error::new(ErrorMessage::InvalidIdentifier(err_msg), Some(position.clone())));
            };
        },
        None => return Err(eof_error())
    };
    

    // The next token must be '='
    match iterator.next() {
        Some((token, position)) => {
            if token.original_string() != "=".to_string() {
                let err_msg = format!("Expected '=', got unexpected token '{}'", token.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(position.clone())));
            }
        }
        None => return Err(eof_error())
    };

    // Rest of the assignment is an expression
    let expression_id = parse_expression(iterator, program)?;

    // last position
    let last_pos = match iterator.current() {
        Some((_, p)) => p.clone(),
        None => return Err(eof_error())
    };

    let assignment_position = start_pos.until(last_pos);


    Ok(Statement::Assignment(var_name, expression_id, assignment_position))
}












/// Parse and return a Statement from the iterator.
/// Each statement SHOULD end with a semicolon. However the way the syntax works makes them unecessary, so not
/// putting them will raise a warning
fn parse_statement(iterator: &mut TokenIterator, program: &mut SlothProgram) -> Result<Statement, Error> {

    let statement = match iterator.current() {
        // Variable assignment or expression call. We'll need the next token to find out
        Some((Token::Identifier(_), p)) => {

            match iterator.peek(1) {
                Some((next_token, p)) => {
                    // This is a variable assignment (IDENTIFIER = EXPRESSION;)
                    if next_token.original_string() == "=".to_string() {
                        parse_assignment(iterator, program)?
                    }

                    // This must be an expression call
                    else {
                        let expr_id = parse_expression(iterator, program)?;
                        // TODO: parse_expression should return the ElementPosition of the Expression
                        Statement::ExpressionCall(expr_id, p.clone())
                    }
                },

                None => return Err(eof_error())
            }
        },


        Some((Token::Keyword(s), p)) => match s.as_str() {
            "if" => todo!(),
            "while" => todo!(),
            kw => {
                let err_msg = format!("Unexpected keyword '{}'. Outside a function, you can only define structures or functions", kw);
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())))
            }
        }

        Some((t, p)) => {
            let err_msg = format!("Unexpected token '{}'. Outside a function, you can only define structures or functions", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())))
        }

        None => return Err(eof_error())
    };


    Ok(statement)
}












































/// Parse a function and add it to the program's function stack
fn parse_function(iterator: &mut TokenIterator, program: &mut SlothProgram) -> Result<(), Error> {
    // must start with the "define" keyword
    match iterator.current() {
        Some((t, p)) => {
            if t.original_string() != "define".to_string() {
                let err_msg = format!("Expected 'define' keyword, got '{}'", t.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())));
            }
        }
        None => return Err(eof_error()),
    };


    // Next token must be the name of the function
    let f_name = match iterator.next() {
        Some((Token::Identifier(s), _)) => s.clone(),
        Some((t, p)) => {
            let err_msg = format!("Expected function name, got '{}'", t.original_string());
            return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())));
        }
        None => return Err(eof_error()),
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
        None => return Err(eof_error()),
    }


    // Parse the input types of the function
    let mut input_types: Vec<Type> = Vec::new();

    while match iterator.peek(1) {
        Some((Token::Keyword(kw), _)) => {kw != &"->".to_string()},
        Some(_) => true,
        None => return Err(eof_error()),
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
            None => return Err(eof_error())
        }

    }


    // The next token must be '->'
    match iterator.next() {
        Some((t, p)) => {
            if t.original_string() != "->".to_string() {
                let err_msg = format!("Expected '->', got '{}'", t.original_string());
                return Err(Error::new(ErrorMessage::SyntaxError(err_msg), Some(p.clone())));
            }
        },
        None => return Err(eof_error()),
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
        None => return Err(eof_error())
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
                    let function = parse_function(&mut iterator, &mut program)?;
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