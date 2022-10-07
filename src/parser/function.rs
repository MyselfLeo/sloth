use crate::lexer::{Token, TokenStream, Keyword, Separator};
use crate::sloth::function::{CustomFunction, FunctionSignature};
use crate::sloth::program::SlothProgram;
use crate::errors::Error;
use crate::sloth::statement::Statement;
use crate::sloth::types::Type;

use super::types::parse_type;
use super::statement::parse_statement;








fn parse_function(stream: &mut TokenStream, program: &mut SlothProgram, module_name: &Option<String>, warning: bool) -> Result<CustomFunction, Error> {
    // "define' keyword
    let (_, first_pos) = super::expect_token(stream, Token::Keyword(Keyword::Define))?;

    // function name
    let func_name = match stream.current() {
        Some((Token::Identifier(n), _)) => {
            stream.next();
            n
        },
        o => return Err(super::wrong_token(o, "function"))
    };

    // method 'for [type]'
    let owner_type = match stream.current() {
        Some((Token::Keyword(Keyword::For), _)) => {
            // parse the type
            stream.next();
            Some(parse_type(stream, program, module_name, warning)?.0)
        },
        _ => None
    };

    // colon
    super::expect_token(stream, Token::Separator(Separator::Colon))?;

    // input types until '->'
    let mut input_types: Vec<(Type, bool)> = Vec::new(); // bool => true = passed by reference

    while super::current_equal(stream, Token::Keyword(Keyword::LeftArrow))? {
        let by_ref = super::current_equal(stream, Token::Separator(Separator::Tilde))?;
        if by_ref {stream.next();};

        let (arg_type, _) = parse_type(stream, program, module_name, warning)?;
        input_types.push((arg_type, by_ref))
    }

    // '->'
    super::expect_token(stream, Token::Keyword(Keyword::LeftArrow))?;

    // return value
    let (output_type, _) = parse_type(stream, program, module_name, warning)?;

    // open bracket
    super::expect_token(stream, Token::Separator(Separator::OpenBracket))?;

    // each statement until '}'
    let mut statements: Vec<Statement> = Vec::new();
    while super::current_equal(stream, Token::Separator(Separator::CloseBracket))? {
        statements.push(parse_statement(stream, program, warning)?);
    };

    // '}'
    super::expect_token(stream, Token::Separator(Separator::CloseBracket))?;


    // return the function
    let func = CustomFunction {
        signature: FunctionSignature::new(
            module_name.clone(),
            func_name,
            owner_type,
            Some(input_types),
            Some(output_type)
        ),

        instructions: statements
    };
    Ok(func)
}