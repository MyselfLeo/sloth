use crate::tokenizer::{ElementPosition};


const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");



pub enum ErrorMessage {
    SyntaxError(String),
    NoEntryPoint(String),
    InvalidEntryPoint(String),
    FileNotFound(String),
    InvalidArguments(String),
    UnexpectedExpression(String),
    RuntimeError(String),
    UnexpectedEOF(String),
    TypeError(String),
    InvalidIdentifier(String),
    OperationErrror(String)
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorMessage::SyntaxError(e) => write!(f, "SYNTAX ERROR: {}", e),
            ErrorMessage::NoEntryPoint(e) => write!(f, "NO ENTRY POINT: {}", e),
            ErrorMessage::InvalidEntryPoint(e) => write!(f, "INVALID ENTRY POINT: {}", e),
            ErrorMessage::FileNotFound(e) => write!(f, "FILE NOT FOUND: {}", e),
            ErrorMessage::InvalidArguments(e) => write!(f, "INVALID ARGUMENTS: {}", e),
            ErrorMessage::UnexpectedExpression(e) => write!(f, "UNEXPECTED EXPRESSION: {}", e),
            ErrorMessage::RuntimeError(e) => write!(f, "RUNTIME ERROR: {} (this is most likely not caused by your code)", e),
            ErrorMessage::UnexpectedEOF(e) => write!(f, "UNEXPECTED EOF: {}", e),
            ErrorMessage::TypeError(e) => write!(f, "TYPE ERROR: {}", e),
            ErrorMessage::InvalidIdentifier(e) => write!(f, "INVALID IDENTIFIER: {}", e),
            ErrorMessage::OperationErrror(e) => write!(f, "OPERATION ERROR: {}", e),
        }
    }
}




pub struct Error {
    message: ErrorMessage,
    position: Option<ElementPosition>
}


impl Error {
    pub fn new(message: ErrorMessage, position: Option<ElementPosition>) -> Error {
        Error { message: message, position: position }
    }


    pub fn abort(&self) {
        match &self.position {
            None => println!("\x1b[91m{}\x1b[0m", self.message),
            Some(p) => {
                let filepath = std::path::Path::new(&p.filename);
                let file_string = std::fs::read_to_string(filepath).expect(format!("Unable to read file {:?}", filepath.as_os_str()).as_str());
                let lines: Vec<&str> = file_string.split('\n').collect();

                let line_index_str_len = (p.line + 1).to_string().len();

                println!("\x1b[91m{}\x1b[0m", self.message);
                println!("\x1b[90m{}:{}  ({} v{})\x1b[0m", p.filename, p.line + 1, CRATE_NAME, CRATE_VERSION);

                println!("\x1b[31m|\x1b[0m");
                println!("\x1b[31m| {}\x1b[0m {}", p.line + 1, lines[p.line]);
                print!("\x1b[31m| \x1b[91m");
                for _ in 0..p.first_column + line_index_str_len + 1 {print!(" ")}
                for _ in 0..(p.last_column - p.first_column + 1) {print!("^")}
                println!("\x1b[0m");
            }
        }
        std::process::exit(1)
    }
}