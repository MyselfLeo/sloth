pub const KEYWORDS: [&str; 11] = ["define", "->", "=", "if", "while", "builtin", "for", "new", "import", "structure", "static"];


#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Builtin,
    Import,
    Static,
    Structure,
    Define,
    For,
    LeftArrow,
    New,
    Equal,
    If,
    While,
}


impl Keyword {
    pub fn to_string(&self) -> String {
        match self {
            Keyword::Builtin => "builtin",
            Keyword::Import => "import",
            Keyword::Static => "static",
            Keyword::Structure => "structure",
            Keyword::Define => "define",
            Keyword::For => "for",
            Keyword::LeftArrow => "->",
            Keyword::New => "new",
            Keyword::Equal => "=",
            Keyword::If => "if",
            Keyword::While => "while",
        }.to_string()
    }


    pub fn from_str(str: &str) -> Result<Keyword, String> {
        let val = match str {
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
            _ => return Err(format!("Unimplemented keyword '{}'", str))
        };
        Ok(val)
    }
}