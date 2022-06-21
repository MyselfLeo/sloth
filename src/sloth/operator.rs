#[derive(Clone, Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Eq,     // Equal
    Gr,     // Greater than
    Lw,     // Lower than
    Ge,     // Greater or Equal
    Le,     // Lower or Equal
    And,
    Or,
    Inv,    // Inverse of boolean
}