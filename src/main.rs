#[allow(dead_code)]
mod tokenizer;
#[allow(dead_code)]
mod errors;


use clap::Parser;



/// Interpreter for the Sloth Programming Language 
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the .slo file to execute
    #[clap(value_parser)]
    file: String,

    /// Display the tokens of the file instead of running it$
    #[clap(long, value_parser)]
    tokens: bool
}





fn main() {
    let args = Args::parse();

    let filename = args.file;
    let program = tokenizer::TokenizedProgram::from_file(&filename).unwrap();

    if args.tokens {program.print_tokens()}
}
