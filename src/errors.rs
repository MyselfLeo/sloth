use std::fmt::Display;

use crate::tokenizer::{ElementPosition};


const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");


#[derive(Debug)]
pub enum ErrorMessage {
    SyntaxError(String),
    NoEntryPoint(String),
    FileNotFound(String),
    InvalidArguments(String),
    UnexpectedExpression(String),
    RuntimeError(String),
    UnexpectedEOF(String),
    TypeError(String),
    RustError(String),
    OperationErrror(String),
    ReturnValueError(String),
    ImportError(String),
    DefinitionError(String)
}


impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorMessage::SyntaxError(e) => write!(f, "SYNTAX ERROR: {}", e),
            ErrorMessage::NoEntryPoint(e) => write!(f, "NO ENTRY POINT: {}", e),
            ErrorMessage::FileNotFound(e) => write!(f, "FILE NOT FOUND: {}", e),
            ErrorMessage::InvalidArguments(e) => write!(f, "INVALID ARGUMENTS: {}", e),
            ErrorMessage::UnexpectedExpression(e) => write!(f, "UNEXPECTED EXPRESSION: {}", e),
            ErrorMessage::RuntimeError(e) => write!(f, "RUNTIME ERROR: {}", e),
            ErrorMessage::UnexpectedEOF(e) => write!(f, "UNEXPECTED EOF: {}", e),
            ErrorMessage::TypeError(e) => write!(f, "TYPE ERROR: {}", e),
            ErrorMessage::RustError(e) => write!(f, "RUST ERROR: {} (this error is from Rust, it is unlikely your fault)", e),
            ErrorMessage::OperationErrror(e) => write!(f, "OPERATION ERROR: {}", e),
            ErrorMessage::ReturnValueError(e) => write!(f, "RETURN VALUE ERROR: {}", e),
            ErrorMessage::ImportError(e) => write!(f, "IMPORT ERROR: {}", e),
            ErrorMessage::DefinitionError(e) => write!(f, "DEFINITION ERROR: {}", e),
        }
    }
}



#[derive(Debug)]
pub struct Error {
    pub message: ErrorMessage,
    pub position: Option<ElementPosition>
}


impl Error {
    pub fn new(message: ErrorMessage, position: Option<ElementPosition>) -> Error {
        
        // if dummy pos, consider no pos was given
        let pos = match position {
            None => None,
            Some(p) => {
                if p.filename == "" {None}
                else {Some(p)}
            }
        };

        Error { message: message, position: pos }
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

                match p.last_column {
                    Some(n) => for _ in 0..(n - p.first_column + 1) {print!("^")},
                    None => for _ in 0..(lines[p.line].len() - p.first_column + 2) {print!("^")}
                }
                
                println!("\x1b[0m");
            }
        }
        std::process::exit(1)
    }


    /// Set the position of the error if none is already set
    pub fn clog_pos(&mut self, pos: ElementPosition) {
        if self.position.is_none() {
            self.position = Some(pos)
        }
    }
}




pub struct Warning {
    pub text: String,
    pub position: Option<ElementPosition>
}

impl Warning {
    pub fn new(text: String, position: Option<ElementPosition>) -> Warning {
        Warning {
            text: text,
            position: position
        }
    }

    pub fn warn(&self) {
        match &self.position {
            None => println!("\x1b[93mWarning: {}\x1b[0m", self.text),
            Some(p) => {
                let filepath = std::path::Path::new(&p.filename);
                let file_string = std::fs::read_to_string(filepath).expect(format!("Unable to read file {:?}", filepath.as_os_str()).as_str());
                let lines: Vec<&str> = file_string.split('\n').collect();

                let line_index_str_len = (p.line + 1).to_string().len();

                println!("\x1b[93mWarning: {}\x1b[0m", self.text);
                println!("\x1b[90m{}:{}  ({} v{})\x1b[0m", p.filename, p.line + 1, CRATE_NAME, CRATE_VERSION);

                println!("\x1b[33m|\x1b[0m");
                println!("\x1b[33m| {}\x1b[0m {}", p.line + 1, lines[p.line]);
                print!("\x1b[33m| \x1b[93m");
                for _ in 0..p.first_column + line_index_str_len + 1 {print!(" ")}
                
                match p.last_column {
                    Some(n) => for _ in 0..(n - p.first_column + 1) {print!("^")},
                    None => for _ in 0..(lines[p.line].len() - p.first_column + 2) {print!("^")}
                }
                
                println!("\x1b[0m");
            }
        }
    }
}








/// Return as a String the elements of the Vec, separated by the given caracter
pub fn formatted_vec_string<T: Display>(vec: &Vec<T>, sep: char) -> String {
    let mut res = String::new();
    for (i, e) in vec.iter().enumerate() {
        res = format!("{} {}", res, e);
        if i < vec.len() - 1 {res.push(sep)}
    }
    res
}