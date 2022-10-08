use crate::lexer::{Token, TokenStream, Separator};
use crate::sloth::program::SlothProgram;
use crate::sloth::structure::{CustomDefinition, StructSignature};
use crate::sloth::types::Type;
use crate::errors::Error;

use super::types::parse_type;



/// Parse a structure definition (starting with keyword "structure")
pub fn parse_structure(stream: &mut TokenStream, program: &mut SlothProgram, module_name: &Option<String>, warning: bool) -> Result<CustomDefinition, Error> {
    // name of the structure
    let (struct_name, first_pos) = match stream.next() {
        Some((Token::Identifier(n), p)) => (n, p),
        o => return Err(super::wrong_token(o, "structure name"))
    };

    // Next is an open bracket
    match stream.next() {
        Some((Token::Separator(Separator::OpenBracket), _)) => {stream.next();},
        o => return Err(super::wrong_token(o, "'{{'"))
    }

    let mut fields: Vec<(String, Type)> = Vec::new();

    // Next is each fields of this structure, until we met a closed bracket
    while !super::current_equal(stream, Token::Separator(Separator::CloseBracket))? {
        // name of the field
        let (field_name, ..) = match stream.current() {
            Some((Token::Identifier(f), p)) => {stream.next(); (f, p)},
            o => return Err(super::wrong_token(o, "field name or '}}'"))
        };

        // colon
        super::expect_token(stream, Token::Separator(Separator::Colon))?;

        // the type of the field
        let (field_type, type_pos) = parse_type(stream, program, module_name, warning)?;

        fields.push((field_name, field_type));

        // A semicolon here is strongly recommended, but not necessary
        super::check_semicolon(stream, warning, &first_pos.until(type_pos))?;
    }
    stream.next();

    // return the definition
    let signature = StructSignature::new(module_name.clone(), struct_name.clone());
    Ok(CustomDefinition::new(signature, fields))
}