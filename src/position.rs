#[derive(Clone, Debug)]
/// Represents the position of an element (token, expression, etc.) in a file.
/// An element can't be on 2 line at the same time
pub struct Position {
    pub filename: String,
    pub line: usize,

    // column index of the first and last character of the element
    pub first_column: usize,
    pub last_column: Option<usize>
}



impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // + 1 to every indices so it starts at 1
        let last_column = match self.last_column {Some(n) => (n + 1).to_string(), None => "?".to_string()};
        write!(f, "({}, line {}, {}-{})", self.filename, self.line + 1, self.first_column + 1, last_column)
    }
}




impl Position {
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    /// Return a new Position starting from the start of self until the end of other.
    /// They both needs to be on the same line
    pub fn until(&self, other: Position) -> Position {
        if self.filename != other.filename {panic!("Tried to link two tokens from different files")}
        if self.line != other.line {
            Position {
                filename: self.filename.clone(),
                line: self.line, first_column: self.first_column,
                last_column: other.last_column
            }
        }
        else {
            Position {
                filename: self.filename.clone(),
                line: self.line, first_column: self.first_column,
                last_column: other.last_column
            }
        }
    }
}