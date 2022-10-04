use regex::Regex;

use super::keyword::Keyword;
use super::separator::Separator;


#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Separator(Separator),
    Identifier(String),
    Operator(String),
    Literal(String),
}


impl Token {
    /// Return the token corresponding to the given text. Will test for keyword, operator and separator.
    pub fn from_str(string: &str) -> Result<Token, String> {
        let identifier_re = Regex::new(r"^(@[0-9]+|@[a-zA-Z]+|[a-zA-Z_][a-zA-Z0-9_]*)$").unwrap();

        if super::keyword::KEYWORDS.contains(&string) {
            let val = match string {
                "builtin" => Keyword::Builtin,
                "import" => Keyword::Import,
                "static" => Keyword::Static,
                "structure" => Keyword::Structure,
                "define" => Keyword::Define,
                "for" => Keyword::For,
                "->" => Keyword::LeftArrow,
                "new" => Keyword::New,
                "=" => Keyword::Equal,
                "if" => Keyword::If,
                "while" => Keyword::While,
                _ => return Err(format!("Unimplemented keyword '{}'", string))
            };
            Ok(Token::Keyword(val))
        }

        else if super::separator::SEPARATORS.contains(&string) {
            let val = match string {
                "(" => Separator::OpenParenthesis,
                ")" => Separator::CloseParenthesis,
                "{" => Separator::OpenBracket,
                "}" => Separator::CloseBracket,
                "[" => Separator::OpenSquareBracket,
                "]" => Separator::CloseSquareBracket,
                ";" => Separator::SemiColon,
                ":" => Separator::Colon,
                "," => Separator::Comma,
                "|" => Separator::Line,
                "." => Separator::Period,
                "~" => Separator::Tilde,
                &_ => return Err(format!("Unimplemented separator '{}'", string))
            };
            Ok(Token::Separator(val))
        }

        else if super::OPERATORS.contains(&string) {
            Ok(Token::Operator(string.to_string()))
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
            Token::Keyword(x) => x.to_string(),
            Token::Identifier(x) => x.clone(),
            Token::Literal(x) => x.clone(),
            Token::Operator(x) => x.clone(),
            Token::Separator(x) => x.to_string()
        }
    }
}