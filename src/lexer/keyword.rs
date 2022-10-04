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
}