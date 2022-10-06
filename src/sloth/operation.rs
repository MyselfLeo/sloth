use super::expression::Expression;



pub enum Operation {
    Addition(Expression, Expression),
    Substraction(Expression, Expression),
    Multiplication(Expression, Expression),
    Division(Expression, Expression),
    Equal(Expression, Expression),
    Greater(Expression, Expression),
    Lower(Expression, Expression),
    GreaterEqual(Expression, Expression),
    LowerEqual(Expression, Expression),
    And(Expression, Expression),
    Or(Expression, Expression),
    Inverse(Expression),
}