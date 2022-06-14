use crate::tokenizer::{ElementPosition};




pub fn syntax_error(e: &str, position: &ElementPosition) {
    let filepath = std::path::Path::new(&position.filename);
    let file_string = std::fs::read_to_string(filepath).expect(format!("Unable to read file {:?}", filepath.as_os_str()));
    let lines: Vec<&str> = file_string.split('\n').collect();

    println!("SYNTAX ERROR");
    println!("{}", e);
    println!();
    println!("{}", lines[position.line]);

    for i in 0..position.first_column {print!(" ")}
    for i in 0..(position.last_column - position.first_column + 1) {print!("^")}
    println!(" here");
    println!("{}, line {}", position.filename, position.line);

    std::process::exit(1)
}