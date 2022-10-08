mod lexer;
mod parser;
mod sloth;
mod builtins;
mod errors;
mod position;

use clap::Parser;
use sloth::program::SlothProgram;
use sloth::value::Value;

use std::time::{Instant, Duration};



/// Interpreter for the Sloth Programming Language 
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of the .slo file to execute
    #[clap(value_parser)]
    file: String,

    /// Print the return code after execution
    #[clap(short, long, value_parser)]
    code: bool,

    /// Compute and display the time of building and execution of the program
    #[clap(short, long, value_parser)]
    time: bool,

    /// Display the tokens of the file instead of running it
    #[clap(long, value_parser)]
    tokens: bool,

    /// Display the list of the functions generated from the file instead of running it
    #[clap(long, value_parser)]
    functions: bool,

    /// Display the list of the expressions generated from the file instead of running it
    #[clap(long, value_parser)]
    expr: bool,

    /// Disable warnings
    #[clap(long, value_parser)]
    nowarn: bool,

    /// Don't import default builtins
    #[clap(long, value_parser)]
    nodefault: bool,

    /// Arguments for the Sloth program
    #[clap(value_parser)]
    arguments: Vec<String>,
}




fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let build_time: Duration;
    let exec_time: Duration;
    let filename = args.file;

    if args.tokens {
        let tokens = match lexer::get_token_stream(&filename) {
            Ok(t) => t,
            Err(e) => {
                e.abort();
                return
            }
        };
        tokens.print_tokens()
    }

    else {
        // build the program
        let mut program: SlothProgram = match parser::build_program(filename.clone(), !args.nowarn, !args.nodefault) {
            Err(e) => {e.abort(); return},
            Ok(p) => p,
        };

        build_time = start_time.elapsed();

        if args.functions {program.print_functions()}
        else if args.expr {program.print_exprs()}
        else {
            unsafe {
                let return_value = program.run(args.arguments);
                exec_time = start_time.elapsed();
                match return_value {
                    Err(e) => e.abort(),
                    Ok(v) => match v {
                        Value::Number(x) => {
                            if args.code || args.time {println!()}
                            if args.code {println!("\x1b[94mExited\x1b[0m with return code {}", x)};
                            if args.time {
                                println!("\x1b[94mBuilt\x1b[0m in {}ms", build_time.as_millis());
                                println!("\x1b[94mExecuted\x1b[0m in {}ms", exec_time.as_millis());
                            }
                            std::process::exit(x as i32)
                        },
                        _ => panic!("The main function must return a Number value")
                    }
                }
            }
        }
    }
}
