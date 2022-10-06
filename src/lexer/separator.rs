pub const SEPARATORS: [&str; 12] = ["(", ")", "{", "}", "[", "]", ";", ":", ",", "|", ".", "~"];


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


    pub fn from_str(str: &str) -> Result<Separator, String> {
        let val = match str {
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
            _ => return Err(format!("Unimplemented separator '{}'", str))
        };
        Ok(val)
    }
}