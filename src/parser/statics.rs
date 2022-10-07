use std::rc::Rc;

use crate::lexer::{Token, TokenStream, Keyword, Separator};
use crate::sloth::program::SlothProgram;
use crate::errors::{Error, Warning, ErrMsg};
use crate::sloth::statement::Statement;

use super::expression::parse_expression;



/// Push the static exprs to the program
pub fn parse_static_expr(stream: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<(), Error> {
    // static keyword
    let (_, first_pos) = super::expect_token(stream, Token::Keyword(Keyword::Static))?;

    // the static's name
    let (name, _) = match stream.current() {
        Some((Token::Identifier(n), p)) => {
            // raise a warning if the identifier is not in full caps
            if warning && n.to_uppercase() != n {
                let warn_msg = format!("It is recommended to set in full uppercase the name of static expressions ('{}')", n.to_uppercase());
                Warning::new(warn_msg, Some(p.clone())).warn()
            }
            (n, p)
        },
        o => return Err(super::wrong_token(o, "static expr. name"))
    };

    // the =
    super::expect_token(stream, Token::Keyword(Keyword::Equal))?;

    // next is the expression
    let expr = parse_expression(stream, program, warning)?;
    let full_pos = first_pos.until(expr.get_pos());

    // ; recommended here
    super::check_semicolon(stream, warning, &full_pos)?;

    // add the expression to the program's statics
    match program.push_static(&name, expr) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(ErrMsg::RuntimeError(e), Some(full_pos))),
    }
}