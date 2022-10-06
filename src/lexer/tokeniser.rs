use crate::position::Position;
use crate::errors::{Error, ErrMsg};
use super::token::Token;





/// Convert the given file into a list of Tokens
pub fn from_file(filename: &str) -> Result<Vec<(Token, Position)>, Error> {
    let filepath = std::path::Path::new(filename);

    if !filepath.exists() {
        let err_msg = format!("File {:?} does not exists", filepath.as_os_str());
        return Err(Error::new(ErrMsg::FileError(err_msg), None));
    }

    let mut tokens: Vec<(Token, Position)> = Vec::new();


    let file_string = match std::fs::read_to_string(filepath) {
        Ok(v) => v,
        Err(e) => return Err(Error::new(ErrMsg::FileError(e.to_string()), None))
    };

    let lines = file_string.split('\n');

    // parse each line one by one, as a token can't be between 2 lines
    let mut line_index: usize = 0;
    'lines: for line in lines {


        let mut token_buffer = String::new();
        let mut string_buffer = String::new();

        let mut token_start = (0, 0);
        let mut string_start = (0, 0);


        // Iterate over each characters
        'chars: for (c_index, c) in line.chars().enumerate() {

            if token_buffer.is_empty() {
                token_start = (line_index, c_index);
            }


            // start of a string
            if string_buffer.is_empty() && c == '"' {
                string_buffer.push('"');
                string_start = (line_index, c_index);

                continue 'chars;
            }


            // We reach the end of a string
            if !string_buffer.is_empty() && c == '"' {
                string_buffer.push('"');

                let position = Position {
                    filename: filename.to_string(),
                    line: string_start.0,
                    first_column: string_start.1,
                    last_column: Some(c_index)
                };

                match Token::from_str(&string_buffer) {
                    Ok(s) => tokens.push((s, position)),
                    Err(e) => return Err(Error::new(ErrMsg::SyntaxError(e), Some(position))),
                }

                string_buffer.clear();
            }



            // If we are not in a string, and we find the COMMENT_START pattern, we can skip the rest of the line
            if string_buffer.is_empty() && c == super::COMMENT_CHAR {
                // Skip the rest of the line and push the current token to the vec
                if !token_buffer.is_empty() {
                    let position = Position {
                        filename: filename.to_string(),
                        line: token_start.0,
                        first_column: token_start.1,
                        last_column: Some(c_index - 1)
                    };

                    match Token::from_str(&token_buffer) {
                        Ok(s) => tokens.push((s, position)),
                        Err(e) => {
                            return Err(Error::new(ErrMsg::SyntaxError(e), Some(position)));
                        },
                    }

                    token_buffer.clear();
                }

                line_index += 1; // increment line_index here as "continue 'lines" won't call the last statement of the loop 'lines' 
                continue 'lines;
            }




            


            // Check if the previous token is terminated by another token, or a default separator
            // example: "fibonacci_rec:" (2 tokens: Identifier(fibonacci_rec) and Colon)
            if string_buffer.is_empty() && (super::separator::SEPARATORS.contains(&c.to_string().as_str()) || super::DEFAULT_SEPARATORS.contains(&c)) {

                // SPECIAL CASE: The period can be a separator, but can also be part of a number.
                // we check if the current buffer can be parsed into an integer: if so, the period is
                // part of the token

                token_buffer = token_buffer.trim().to_string();

                if token_buffer.parse::<i64>().is_ok() && c == '.' && line.chars().nth(c_index + 1).unwrap_or(' ').is_numeric() {
                    token_buffer.push('.');
                }

                else {
                    // Check if the token_buffer starts with an operator and is not a keyword, because the op can be sticked to its operands: !true, >=value, etc.
                    // if so, we separate it, create its own Token, etc. then continue with the rest of the buffer

                    if !super::keyword::KEYWORDS.contains(&token_buffer.as_str()) {
                        for op in super::operator::OPERATORS {
                            if token_buffer.starts_with(op) {
                                let op_pos = Position {
                                    filename: filename.to_string(),
                                    line: token_start.0,
                                    first_column: token_start.1,
                                    last_column: Some(token_start.1 + op.len())
                                };

                                token_start.1 += op.len();
                                token_buffer = token_buffer.strip_prefix(op).unwrap_or(&token_buffer).to_string();

                                // push the OP token
                                match Token::from_str(op) {
                                    Ok(s) => tokens.push((s, op_pos)),
                                    Err(e) => {
                                        return Err(Error::new(ErrMsg::SyntaxError(e), Some(op_pos)));
                                    },
                                };
                            }
                        }
                    }


                    // Push previous token buffer to the list (if not empty), along with its position.
                    if !token_buffer.is_empty() {
                        let position = Position {
                            filename: filename.to_string(),
                            line: token_start.0,
                            first_column: token_start.1,
                            last_column: Some(c_index - 1)
                        };

                        match Token::from_str(&token_buffer) {
                            Ok(s) => tokens.push((s, position)),
                            Err(e) => {
                                return Err(Error::new(ErrMsg::SyntaxError(e), Some(position)));
                            },
                        };

                        token_buffer.clear();
                    }

                    // Push the separator as a token, only if SEPARATORS contains the character
                    if super::separator::SEPARATORS.contains(&c.to_string().as_str()) {
                        let position = Position {
                            filename: filename.to_string(),
                            line: line_index,
                            first_column: c_index,
                            last_column: Some(c_index)
                        };

                        match Token::from_str(&c.to_string()) {
                            Ok(s) => tokens.push((s, position)),
                            Err(e) => {
                                return Err(Error::new(ErrMsg::SyntaxError(e), Some(position)));
                            },
                        };

                        token_buffer.clear();
                    }
                }

                continue 'chars;
            }




            // add current char to the buffer if we're in a token, or to the string_buffer if we're in a string
            if !string_buffer.is_empty() {
                string_buffer.push(c);
            }
            else {
                token_buffer.push(c);
            }
        }



        // Add the remaining of the buffer as a token
        if !token_buffer.is_empty() {
            let position = Position {
                filename: filename.to_string(),
                line: token_start.0,
                first_column: token_start.1,
                last_column: Some(line.len() - 1)
            };

            match Token::from_str(&token_buffer) {
                Ok(s) => tokens.push((s, position)),
                Err(e) => {
                    return Err(Error::new(ErrMsg::SyntaxError(e), Some(position)));
                },
            }

            token_buffer.clear();
        }



        line_index += 1;
    }


    Ok(tokens)
}