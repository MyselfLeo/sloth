// The tokenizer (TokenisedProgram) takes a .slo file and convert it into a list of tokens,
// to be used by the Builder to generate a Program Tree

use crate::errors::{Error, ErrorMessage};
use regex::Regex;


const KEYWORDS: [&str; 10] = ["define", "none", "->", "=", "if", "while", "builtin", "for", "new", "import"];
const OPERATORS: [&str; 12] = ["+", "-", "*", "/", "<=", ">=", "==", "<", ">", "&", "?", "!"];                                  // The '<=' and '>=' must be before '<' and '>' so the parsing works
const SEPARATORS: [&str; 12] = ["(", ")", "{", "}", "[", "]", ";", ":", ",", "|", ".", "~"];

// Unlike SEPARATORS, those do not have a semantic meaning (only used for separating tokens)
const DEFAULT_SEPARATORS: [char; 2] = [' ', '"'];

// Comments starts with this str and ends at the end of the line
const COMMENT_CHAR: char = '#';



#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // Each token has a (line, column) parameter
    Keyword(String),
    Identifier(String),
    Separator(Separator),
    Operator(String),
    Literal(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Separator {
    OpenParenthesis,
    CloseParenthesis,
    OpenBracket,
    CloseBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    SemiColon,
    Colon,
    Comma,
    Line,
    Period,
    Tilde
}

impl Separator {
    pub fn to_string(&self) -> String {
        match self {
            Separator::OpenParenthesis => "(",
            Separator::CloseParenthesis => ")",
            Separator::OpenBracket => "{",
            Separator::CloseBracket => "}",
            Separator::OpenSquareBracket => "[",
            Separator::CloseSquareBracket => "]",
            Separator::SemiColon => ";",
            Separator::Colon => ":",
            Separator::Comma => ",",
            Separator::Line => "|",
            Separator::Period => ".",
            Separator::Tilde => "~"
        }.to_string()
    }
}





impl Token {
    /// Return the token corresponding to the given text. Will test for keyword, operator and separator.
    pub fn from_str(string: &str) -> Result<Token, String> {
        let identifier_re = Regex::new(r"^(@[0-9]+|@[a-zA-Z]+|[a-zA-Z_][a-zA-Z0-9_]*)$").unwrap();

        if KEYWORDS.contains(&string) {Ok(Token::Keyword(string.to_string()))}
        else if OPERATORS.contains(&string) {Ok(Token::Operator(string.to_string()))}
        else if SEPARATORS.contains(&string) {
            match string {
                "(" => Ok(Token::Separator(Separator::OpenParenthesis)),
                ")" => Ok(Token::Separator(Separator::CloseParenthesis)),
                "{" => Ok(Token::Separator(Separator::OpenBracket)),
                "}" => Ok(Token::Separator(Separator::CloseBracket)),
                "[" => Ok(Token::Separator(Separator::OpenSquareBracket)),
                "]" => Ok(Token::Separator(Separator::CloseSquareBracket)),
                ";" => Ok(Token::Separator(Separator::SemiColon)),
                ":" => Ok(Token::Separator(Separator::Colon)),
                "," => Ok(Token::Separator(Separator::Comma)),
                "|" => Ok(Token::Separator(Separator::Line)),
                "." => Ok(Token::Separator(Separator::Period)),
                "~" => Ok(Token::Separator(Separator::Tilde)),
                &_ => Err(format!("Unimplemented separator '{}'", string))
            }
        }

        // literals (strings, numbers or booleans)
        else if string.starts_with('"') || string.parse::<f64>().is_ok() || string == "true" || string == "false" {
            Ok(Token::Literal(string.to_string()))
        }

        // Identifiers can only have letters, numbers (not at the start) and _
        else if identifier_re.is_match(string) {
            Ok(Token::Identifier(string.to_string()))
        }

        // raise error as the token is not identified
        else {
            Err(format!("Invalid token '{}'. Note: identifiers can only be made of letters, numbers (not at the start) and '_'", string))
        }
    }


    pub fn to_string_formatted(&self) -> String {
        format!("{:?}", self)
    }

    pub fn original_string(&self) -> String {
        match self {
            Token::Keyword(x) => x.clone(),
            Token::Identifier(x) => x.clone(),
            Token::Literal(x) => x.clone(),
            Token::Operator(x) => x.clone(),
            Token::Separator(x) => x.to_string()
        }
    }
}




#[derive(Clone, Debug)]
/// Represents the position of an element (token, expression, etc.) in a file.
/// An element can't be on 2 line at the same time
pub struct ElementPosition {
    pub filename: String,
    pub line: usize,

    // column index of the first and last character of the element
    pub first_column: usize,
    pub last_column: Option<usize>
}

impl std::fmt::Display for ElementPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // + 1 to every indices so it starts at 1
        let last_column = match self.last_column {Some(n) => (n + 1).to_string(), None => "?".to_string()};
        write!(f, "({}, line {}, {}-{})", self.filename, self.line + 1, self.first_column + 1, last_column)
    }
}

impl ElementPosition {
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    /// Return a new ElementPosition starting from the start of self until the end of other.
    /// They both needs to be on the same line
    pub fn until(&self, other: ElementPosition) -> ElementPosition {
        if self.filename != other.filename {panic!("Tried to link two tokens from different files")}
        if self.line != other.line {
            ElementPosition {
                filename: self.filename.clone(),
                line: self.line, first_column: self.first_column,
                last_column: other.last_column
            }
        }
        else {
            ElementPosition {
                filename: self.filename.clone(),
                line: self.line, first_column: self.first_column,
                last_column: other.last_column
            }
        }
    }
}




/// list of tokens and their respective position generated from a program file (.slo)
#[derive(Clone)]
pub struct TokenizedProgram {
    pub filename: String,
    pub tokens: Vec<Token>,
    pub positions: Vec<ElementPosition>
}


impl TokenizedProgram {

    pub fn from_file(filename: &str) -> Result<TokenizedProgram, Error> {
        let filepath = std::path::Path::new(filename);
        if !filepath.exists() {
            let err_msg = format!("File {:?} does not exists", filepath.as_os_str());
            return Err(Error::new(ErrorMessage::FileNotFound(err_msg), None));
        }

        let mut token_list: Vec<Token> = Vec::new();
        let mut position_list: Vec<ElementPosition> = Vec::new();

        let file_string = std::fs::read_to_string(filepath).expect(format!("Unable to read file {:?}", filepath.as_os_str()).as_str());
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

                    let position = ElementPosition {
                        filename: filename.to_string(),
                        line: string_start.0,
                        first_column: string_start.1,
                        last_column: Some(c_index)
                    };

                    match Token::from_str(&string_buffer) {
                        Ok(s) => {
                            token_list.push(s);
                            position_list.push(position);
                        },
                        Err(e) => {
                            return Err(Error::new(ErrorMessage::SyntaxError(e), Some(position)));
                        },
                    }

                    string_buffer.clear();
                }



                // If we are not in a string, and we find the COMMENT_START pattern, we can skip the rest of the line
                if string_buffer.is_empty() && c == COMMENT_CHAR {
                    // Skip the rest of the line and push the current token to the vec
                    if !token_buffer.is_empty() {
                        let position = ElementPosition {
                            filename: filename.to_string(),
                            line: token_start.0,
                            first_column: token_start.1,
                            last_column: Some(c_index - 1)
                        };

                        match Token::from_str(&token_buffer) {
                            Ok(s) => {
                                token_list.push(s);
                                position_list.push(position);
                            },
                            Err(e) => {
                                return Err(Error::new(ErrorMessage::SyntaxError(e), Some(position)));
                            },
                        }

                        token_buffer.clear();
                    }

                    line_index += 1; // increment line_index here as "continue 'lines" won't call the last statement of the loop 'lines' 
                    continue 'lines;
                }




                


                // Check if the previous token is terminated by another token, or a default separator
                // example: "fibonacci_rec:" (2 tokens: Identifier(fibonacci_rec) and Colon)
                if string_buffer.is_empty() && (SEPARATORS.contains(&c.to_string().as_str()) || DEFAULT_SEPARATORS.contains(&c)) {

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

                        if !KEYWORDS.contains(&token_buffer.as_str()) {
                            for op in OPERATORS {
                                if token_buffer.starts_with(op) {
                                    let op_pos = ElementPosition {
                                        filename: filename.to_string(),
                                        line: token_start.0,
                                        first_column: token_start.1,
                                        last_column: Some(token_start.1 + op.len())
                                    };
    
                                    token_start.1 += op.len();
                                    token_buffer = token_buffer.strip_prefix(op).unwrap_or(&token_buffer).to_string();
    
                                    // push the OP token
                                    match Token::from_str(op) {
                                        Ok(s) => {
                                            token_list.push(s);
                                            position_list.push(op_pos);
                                        },
                                        Err(e) => {
                                            return Err(Error::new(ErrorMessage::SyntaxError(e), Some(op_pos)));
                                        },
                                    };
                                }
                            }
                        }


                        // Push previous token buffer to the list (if not empty), along with its position.
                        if !token_buffer.is_empty() {
                            let position = ElementPosition {
                                filename: filename.to_string(),
                                line: token_start.0,
                                first_column: token_start.1,
                                last_column: Some(c_index - 1)
                            };

                            match Token::from_str(&token_buffer) {
                                Ok(s) => {
                                    token_list.push(s);
                                    position_list.push(position);
                                },
                                Err(e) => {
                                    return Err(Error::new(ErrorMessage::SyntaxError(e), Some(position)));
                                },
                            };

                            token_buffer.clear();
                        }

                        // Push the separator as a token, only if SEPARATORS contains the character
                        if SEPARATORS.contains(&c.to_string().as_str()) {
                            let position = ElementPosition {
                                filename: filename.to_string(),
                                line: line_index,
                                first_column: c_index,
                                last_column: Some(c_index)
                            };

                            match Token::from_str(&c.to_string()) {
                                Ok(s) => {
                                    token_list.push(s);
                                    position_list.push(position);
                                },
                                Err(e) => {
                                    return Err(Error::new(ErrorMessage::SyntaxError(e), Some(position)));
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
                let position = ElementPosition {
                    filename: filename.to_string(),
                    line: token_start.0,
                    first_column: token_start.1,
                    last_column: Some(line.len() - 1)
                };

                match Token::from_str(&token_buffer) {
                    Ok(s) => {
                        token_list.push(s);
                        position_list.push(position);
                    },
                    Err(e) => {
                        return Err(Error::new(ErrorMessage::SyntaxError(e), Some(position)));
                    },
                }

                token_buffer.clear();
            }



            line_index += 1;
        }


        Ok(TokenizedProgram{filename: filename.to_string(), tokens: token_list, positions: position_list})
    }



    /// Print to the console the list of tokens
    pub fn print_tokens(&self) {
        for i in 0..self.tokens.len() {
            println!("{:<10}{:40}{:40}", i+1, self.tokens[i].to_string_formatted(), self.positions[i].to_string());
        }
    }
}