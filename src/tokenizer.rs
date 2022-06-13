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

        else {
            Err(format!("Unexpected token {}", string))
        }
    }
}



/// Represents the position of a token in a file.
/// A token can't be on 2 line at the same time
pub struct TokenPosition {
    filename: String,
    line: usize,

    // column index of the first and last character of the token
    first_column: usize,
    last_column: usize
}




/// list of tokens and their respective position generated from a program file (.slo)
pub struct TokenizedProgram {
    tokens: Vec<Token>,
    positions: Vec<TokenPosition>
}


impl TokenizedProgram {

    pub fn from_file(filename: &str) -> Result<TokenizedProgram, String> {
        let filepath = std::path::Path::new(filename);
        if !filepath.exists() {return Err(format!("File {:?} does not exists", filepath.as_os_str()));}

        let token_list: Vec<Token> = Vec::new();
        let position_list: Vec<TokenPosition> = Vec::new();

        let file_string = std::fs::read_to_string(filepath).expect(format!("Unable to read file {:?}", filepath.as_os_str()));
        let lines = file_string.split('\n');

        // parse each line one by one, as a token can't be between 2 lines
        let line_index: usize = 0;
        for raw_line in lines {
            
            // Go until first comment separating
            let line_shards: Vec<&str> = raw_line.split(COMMENT_START).collect();
            let line = line_shards[0];



            



            line_index += 1;
        }


        Ok(TokenizedProgram{tokens: token_list, positions: position_list})
    }
}