use crate::lexer::{Token, TokenStream, Separator};
use crate::sloth::program::SlothProgram;
use crate::sloth::types::Type;
use crate::position::Position;
use crate::errors::Error;


/// Parse a type (ex: num, string, list[num], Struct, list[list[string]], etc.)
pub fn parse_type(stream: &mut TokenStream, program: &mut SlothProgram, module_name: &Option<String>, warning: bool) -> Result<(Type, Position), Error> {
    println!("parsing type");

    let first_pos;
    let mut last_pos;

    let first_type_name = match stream.current() {
        Some((Token::Identifier(n), p)) => {
            first_pos = p.clone();
            last_pos = p;
            n
        },
        o => return Err(super::wrong_token(o, "type"))
    };

    let return_type = match first_type_name.as_str() {
        "any" => Type::Any,
        "num" => Type::Number,
        "bool" => Type::Boolean,
        "string" => Type::String,
        "list" => {
            
            // [
            match stream.next() {
                Some((Token::Separator(Separator::OpenSquareBracket), _)) => stream.next(),
                o => return Err(super::wrong_token(o, "'['"))
            };

            // parse the inner-type of the list
            let (list_type, _) = parse_type(stream, program, module_name, warning)?;
            
            // ]
            match stream.current() {
                Some((Token::Separator(Separator::CloseSquareBracket), p)) => last_pos = p,
                o => return Err(super::wrong_token(o, "']'"))
            };

            Type::List(Box::new(list_type))
        },
        _ => {Type::Object(first_type_name)}
    };

    stream.next();

    Ok((return_type, first_pos.until(last_pos)))
}