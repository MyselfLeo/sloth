use crate::errors;


const KEYWORDS: [&str; 11] = ["define", "num", "bool", "string", "list", "none", "->", "=", "if", "while", "use"];
const OPERATORS: [&str; 12] = ["+", "-", "*", "/", "<", ">", "<=", ">=", "==", "&", "?", "!"];
const SEPARATORS: [&str; 9] = ["(", ")", "{", "}", "[", "]", ";", ":", ","];

// Unlike SEPARATORS, those do not have a semantic meaning (only used for separating tokens)
const DEFAULT_SEPARATORS: [char; 1] = [' '];

// Comments starts with this str and ends at the end of the line
const COMMENT_START: &str = "//";



#[derive(Debug, PartialEq)]
pub enum Token {
    // Each token has a (line, column) parameter
    Keyword(String),
    Identifier(String),
    Separator(Separator),
    Operator(String),
    Literal(String),
}

#[derive(Debug, PartialEq)]
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
}


impl Token {
    /// Return the token corresponding to the given text. Will test for keyword, operator and separator.
    pub fn from_str(string: &str) -> Result<Token, String> {
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
                &_ => Err(format!("Unimplemented separator {}", string))
            }
        }

        else if string.starts_with('"') || string.parse::<f64>().is_ok() {
            Ok(Token::Literal(string.to_string()))
        }

        else {
            Ok(Token::Identifier(string.to_string()))
        }
    }
}



/// Represents the position of an element (token, expression, etc.) in a file.
/// An element can't be on 2 line at the same time
pub struct ElementPosition {
    pub filename: String,
    pub line: usize,

    // column index of the first and last character of the element
    pub first_column: usize,
    pub last_column: usize
}




/// list of tokens and their respective position generated from a program file (.slo)
pub struct TokenizedProgram {
    tokens: Vec<Token>,
    positions: Vec<ElementPosition>
}


impl TokenizedProgram {

    pub fn from_file(filename: &str) -> Result<TokenizedProgram, String> {
        let filepath = std::path::Path::new(filename);
        if !filepath.exists() {return Err(format!("File {:?} does not exists", filepath.as_os_str()));}

        let mut token_list: Vec<Token> = Vec::new();
        let mut position_list: Vec<ElementPosition> = Vec::new();

        let file_string = std::fs::read_to_string(filepath).expect(format!("Unable to read file {:?}", filepath.as_os_str()));
        let lines = file_string.split('\n');

        // parse each line one by one, as a token can't be between 2 lines
        let line_index: usize = 0;
        for line in lines {
            

            let mut token_buffer = String::new();
            let mut string_buffer = String::new();

            let mut token_start;


            // Iterate over each characters
            for (c_index, c) in line.chars().enumerate() {

                if token_buffer.is_empty() {
                    token_start = (line_index, c_index);
                }


                // If we are not in a string, and we find the COMMENT_START pattern, we can skip the rest of the line
                if string_buffer.is_empty() {
                    
                }
                


                // Check if the previous token is terminated by another token, or a default separator
                // example: "fibonacci_rec:" (2 tokens: Identifier(fibonacci_rec) and Colon)
                if SEPARATORS.contains(&c.to_string().as_str()) || DEFAULT_SEPARATORS.contains(&c){
                    // Push previous token buffer to the list (if not empty), along with its position.
                    if !token_buffer.is_empty() {
                        let position = ElementPosition {
                            filename: filename.to_string(),
                            line: token_start.0,
                            first_column: token_start.1,
                            last_column: c_index - 1
                        };

                        match Token::from_str(&token_buffer) {
                            Ok(s) => {
                                token_list.push(s);
                                position_list.push(position);
                            },
                            Err(e) => errors::syntax_error(&e, &position),
                        }

                        token_buffer.clear();
                    }

                    // Push the separator as a token, only if SEPARATORS contains the character
                    if SEPARATORS.contains(&c.to_string().as_str()) {
                        let position = ElementPosition {
                            filename: filename.to_string(),
                            line: line_index,
                            first_column: c_index,
                            last_column: c_index
                        };
                        match Token::from_str(&c.to_string()) {
                            Ok(s) => {
                                token_list.push(s);
                                position_list.push(position);
                            },
                            Err(e) => errors::syntax_error(&e, &position),
                        }
                    }
                }



                


            }


            line_index += 1;
        }


        Ok(TokenizedProgram{tokens: token_list, positions: position_list})
    }
}