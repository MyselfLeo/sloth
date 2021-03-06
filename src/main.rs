mod tokenizer;
mod errors;
mod sloth;
mod built_in;
mod builder;

use clap::Parser;
use sloth::program::SlothProgram;
use sloth::value::Value;



/// Interpreter for the Sloth Programming Language 
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the .slo file to execute
    #[clap(value_parser)]
    file: String,

    /// Display the tokens of the file instead of running it
    #[clap(long, value_parser)]
    tokens: bool,

    /// Display the list of the functions generated from the file instead of running it
    #[clap(long, value_parser)]
    functions: bool,

    /// Display the list of the expressions generated from the file instead of running it
    #[clap(long, value_parser)]
    expr: bool,

    /// Print the return code after execution
    #[clap(long, value_parser)]
    code: bool,

    /// Disable the warnings
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

    let filename = args.file;
    let tokens = tokenizer::TokenizedProgram::from_file(&filename);

    match tokens {
        Err(e) => e.abort(),
        Ok(tokens) => {

            if args.tokens {tokens.print_tokens()}
            else {
                // build the program
                let mut program: SlothProgram = match builder::build(tokens, !args.nowarn, !args.nodefault) {
                    Err(e) => {e.abort(); return},
                    Ok(p) => p,
                };

                
                if args.functions {program.print_functions()}

                else if args.expr {program.print_exprs()}

                else {
                    unsafe {
                        let return_value = program.run(args.arguments);

                        match return_value {
                            Err(e) => e.abort(),
                            Ok(v) => match v {
                                Value::Number(x) => {
                                    if args.code {println!("Exited with return code {}", x)};
                                    std::process::exit(x as i32)
                                },
                                _ => panic!("The main function must return a Number value")
                            }
                        }
                    }
                }
            }
        }
    }
}
