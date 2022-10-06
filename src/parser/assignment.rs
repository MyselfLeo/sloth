use crate::lexer::{Token, TokenStream, Keyword};
use crate::position::Position;
use crate::sloth::expression::ExpressionID;
use crate::sloth::program::SlothProgram;
use crate::sloth::statement::Statement;
use crate::errors::Error;

use super::expression::parse_expression;



/// Parse an assignment statement
/// Starts at the '=' sign
fn parse_assignment(target: (ExpressionID, Position), stream: &mut TokenStream, program: &mut SlothProgram, warning: bool) -> Result<Statement, Error> {
    let (target_expr, target_pos) = target;
    super::expect_token(stream, Token::Keyword(Keyword::Equal))?;

    // Rest of the assignment is an expression
    let source = parse_expression(stream, program, warning)?;
    let assignment_position = target_pos.until(source.get_pos());

    Ok(Statement::Assignment(target_expr, source, assignment_position))
}