// The '<=' and '>=' must be before '<' and '>' so the parsing works
pub const OPERATORS: [&str; 12] = ["+", "-", "*", "/", "<=", ">=", "==", "<", ">", "&", "?", "!"];


#[derive(Clone, Debug, PartialEq)]
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


impl Operator {
    pub fn to_string(&self) -> String {
        match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Eq => "==",
            Operator::Gr => ">",
            Operator::Lw => "<",
            Operator::Ge => ">=",
            Operator::Le => "<=",
            Operator::And => "&",
            Operator::Or => "?",
            Operator::Inv => "!",
        }.to_string()
    }


    pub fn from_str(str: &str) -> Result<Operator, String> {
        let val = match str {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            "==" => Operator::Eq,
            ">" => Operator::Gr,
            "<" => Operator::Lw,
            ">=" => Operator::Ge,
            "<=" => Operator::Le,
            "&" => Operator::And,
            "?" => Operator::Or,
            "!" => Operator::Inv,
            _ => return Err(format!("Unimplemented operator '{}'", str))
        };
        Ok(val)
    }
}