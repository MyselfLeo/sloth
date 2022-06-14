#[allow(dead_code)]
mod tokenizer;
#[allow(dead_code)]
mod errors;

fn main() {

    let program = tokenizer::TokenizedProgram::from_file("factorial.slo").unwrap();

    println!("{}", program);
}
