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

        // find the highest line error n° to determine the space required at the left of the backtrace line to fit line numbers
        let max_n = self.position_trace.iter()
                                                      .map(|p| p.line + 1)
                                                      .max();

        let max_n = match max_n {
            None => 0,
            Some(n) => n.to_string().len() + 2
        };


        let mut raw_space = String::new();
        for _ in 0..max_n {raw_space.push(' ')}

        for (i, p) in self.position_trace.iter().enumerate() {
            let filepath = std::path::Path::new(&p.filename);
            let file_string = std::fs::read_to_string(filepath).expect(format!("Unable to read file {:?}", filepath.as_os_str()).as_str());
            let lines: Vec<&str> = file_string.split('\n').collect();

            if i == 0 {println!("\x1b[31m{raw_space}■ Error backtrace\x1b[90m {CRATE_NAME} v{CRATE_VERSION}\x1b[0m");}
            else {println!("\x1b[31m{raw_space}v\x1b[0m");}
            println!("\x1b[31m{raw_space}|\x1b[0m");
            
            let nb_space_before_line_n = max_n - ((p.line + 1).to_string().len() + 1);
            let mut spaces = String::new();
            for _ in 0..nb_space_before_line_n {spaces.push(' ')}
            println!("\x1b[31m{}{} |\x1b[0m {}", spaces, p.line + 1, lines[p.line]);

            print!("\x1b[31m{raw_space}| \x1b[91m");
            for _ in 0..p.first_column {print!(" ")}

            match p.last_column {
                Some(n) => for _ in 0..(n - p.first_column + 1) {print!("^")},
                None => for _ in 0..(lines[p.line].len() - p.first_column + 2) {print!("^")}
            }

            println!("\x1b[90m         ({}:{})\x1b[0m", p.filename, p.line + 1);
        }

        if !self.position_trace.is_empty() {
            println!("\x1b[31m{raw_space}|\x1b[0m");
            print!("\x1b[31m{raw_space}=>");
        }

        // Print the error
        println!("\x1b[91m {}\x1b[0m", self.message);
        

        std::process::exit(1)
    }


    /// Return a copy of the error with the given position added.
    /// If the last error is on the same line as the new one,
    /// we keep the last one (the one more in-depth)
    pub fn with(self, pos: &Position) -> Error {
        let mut new_err = self.clone();

        // don't add the new error if on the same line but more "general" than the last one
        if new_err.position_trace.len() > 0 {
            if new_err.position_trace[0].line == pos.line && new_err.position_trace[0].filename == pos.filename {
                return self
            }
        }

        new_err.position_trace.insert(0, pos.clone());
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
            Err(e) => {
                println!("Propagating from {}", line!());
                return Err(e.with($err))
            }
        }
    };
}
