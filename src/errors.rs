use crate::tokenizer::{ElementPosition};


const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");



pub fn syntax_error(e: &str, position: &ElementPosition) {
    let filepath = std::path::Path::new(&position.filename);
    let file_string = std::fs::read_to_string(filepath).expect(format!("Unable to read file {:?}", filepath.as_os_str()).as_str());
    let lines: Vec<&str> = file_string.split('\n').collect();

    let line_index_str_len = (position.line + 1).to_string().len();

    println!("\x1b[91mSYNTAX ERROR: {}\x1b[0m", e);
    println!("\x1b[90m{}:{}  ({} v{})\x1b[0m", position.filename, position.line + 1, CRATE_NAME, CRATE_VERSION);

    println!("\x1b[31m|\x1b[0m");
    println!("\x1b[31m| {}\x1b[0m {}", position.line + 1, lines[position.line]);
    print!("\x1b[31m| \x1b[91m");
    for _ in 0..position.first_column + line_index_str_len + 1 {print!(" ")}
    for _ in 0..(position.last_column - position.first_column + 1) {print!("^")}
    println!("\x1b[0m");

    std::process::exit(1)
}