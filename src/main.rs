#[allow(dead_code)]
mod tokenizer;
#[allow(dead_code)]
mod errors;
#[allow(dead_code)]
mod sloth;
#[allow(dead_code)]
mod built_in;
#[allow(dead_code)]
mod builder;


use clap::Parser;
use sloth::{program::SlothProgram, value::Value};



/// Interpreter for the Sloth Programming Language 
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the .slo file to execute
    #[clap(value_parser)]
    file: String,

    /// Display the tokens of the file instead of running it$
    #[clap(long, value_parser)]
    tokens: bool,

    /// Arguments for the Sloth program
    #[clap(value_parser)]
    arguments: Vec<String>
}







fn main() {
    let args = Args::parse();

    let filename = args.file;
    let tokens = tokenizer::TokenizedProgram::from_file(&filename);

    match tokens {
        Err(e) => e.abort(),
        Ok(tokens) => {

            if args.tokens {tokens.print_tokens()}
            else {
                // build the program
                let mut program: SlothProgram = match builder::build(tokens) {
                    Err(e) => {e.abort(); return},
                    Ok(p) => p,
                };

                unsafe {
                    let return_value = program.run(args.arguments);

                    match return_value {
                        Err(e) => e.abort(),
                        Ok(v) => match v {
                            Value::Number(x) => std::process::exit(x as i32),
                            _ => panic!("The main function must return a Number value")
                        }
                    }
                }

            }

        }
    }
}
