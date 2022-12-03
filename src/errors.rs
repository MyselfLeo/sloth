use crate::position::Position;


const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");


#[derive(Debug, Clone)]
pub enum ErrMsg {
    SyntaxError(String),
    NoEntryPoint(String),
    FileError(String),
    InvalidArguments(String),
    UnexpectedExpression(String),
    RuntimeError(String),
    UnexpectedEOF(String),
    TypeError(String),
    RustError(String),
    ReturnValueError(String),
    ImportError(String),
    DefinitionError(String),
    FunctionError(String),
}


impl std::fmt::Display for ErrMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrMsg::SyntaxError(e) => write!(f, "SYNTAX ERROR: {}", e),
            ErrMsg::NoEntryPoint(e) => write!(f, "NO ENTRY POINT: {}", e),
            ErrMsg::FileError(e) => write!(f, "FILE ERROR {}", e),
            ErrMsg::InvalidArguments(e) => write!(f, "INVALID ARGUMENTS: {}", e),
            ErrMsg::UnexpectedExpression(e) => write!(f, "UNEXPECTED EXPRESSION: {}", e),
            ErrMsg::RuntimeError(e) => write!(f, "RUNTIME ERROR: {}", e),
            ErrMsg::UnexpectedEOF(e) => write!(f, "UNEXPECTED EOF: {}", e),
            ErrMsg::TypeError(e) => write!(f, "TYPE ERROR: {}", e),
            ErrMsg::RustError(e) => write!(f, "RUST ERROR: {} (this error is from Rust, it is unlikely your fault)", e),
            ErrMsg::ReturnValueError(e) => write!(f, "RETURN VALUE ERROR: {}", e),
            ErrMsg::ImportError(e) => write!(f, "IMPORT ERROR: {}", e),
            ErrMsg::DefinitionError(e) => write!(f, "DEFINITION ERROR: {}", e),
            ErrMsg::FunctionError(e) => write!(f, "FUNCTION ERROR: {}", e),
        }
    }
}



#[derive(Debug, Clone)]
pub struct Error {
    pub message: ErrMsg,
    pub position_trace: Vec<Position>
}


impl Error {
    pub fn new(message: ErrMsg, position: Option<Position>) -> Error {
        // if dummy pos, consider no pos was given
        let pos = match position {
            None => vec![],
            Some(p) => {
                if p.filename == "" {vec![]}
                else {vec![p]}
            }
        };

        Error {message: message, position_trace: pos}
    }


    pub fn abort(&self) {

        // print the trace from the deeper to the shallower
        for p in &self.position_trace {
            let filepath = std::path::Path::new(&p.filename);
            let file_string = std::fs::read_to_string(filepath).expect(format!("Unable to read file {:?}", filepath.as_os_str()).as_str());
            let lines: Vec<&str> = file_string.split('\n').collect();

            let line_index_str_len = (p.line + 1).to_string().len();

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

        // Print the error
        println!("\x1b[91m{}\x1b[0m", self.message);

        std::process::exit(1)
    }


    /// Return a copy of the error with the given position added
    pub fn with(self, pos: &Position) -> Error {
        let mut new_err = self.clone();
        new_err.position_trace.push(pos.clone());

        new_err
    }
}




pub struct Warning {
    pub text: String,
    pub position: Option<Position>
}

impl Warning {
    pub fn new(text: String, position: Option<Position>) -> Warning {
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





/// Evaluate the expression if it returns Result<_, Error>.
/// if it returns an Error, add the given position to it and propagate it
#[macro_export]
macro_rules! propagate {
    ($expr:expr, $err:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => return Err(e.with($err))
        }
    };
}
