// The '<=' and '>=' must be before '<' and '>' so the parsing works
pub const OPERATORS: [&str; 14] = ["+", "-", "*", "/", "%", "<=", ">=", "==", "<", ">", "&", "?", "!", "#"];


#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,    // modulo
    Eq,     // Equal
    Gr,     // Greater than
    Lw,     // Lower than
    Ge,     // Greater or Equal
    Le,     // Lower or Equal
    And,
    Or,
    Inv,    // Inverse of boolean
    Len,    // length operator
}


impl Operator {
    pub fn to_string(&self) -> String {
        match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Mod => "%",
            Operator::Eq => "==",
            Operator::Gr => ">",
            Operator::Lw => "<",
            Operator::Ge => ">=",
            Operator::Le => "<=",
            Operator::And => "&",
            Operator::Or => "?",
            Operator::Inv => "!",
            Operator::Len => "#",
        }.to_string()
    }

    /// Return the unique name of this operator (add, mul, div, etc.)
    pub fn get_name(&self) -> String {
        match self {
            Operator::Add => "add",
            Operator::Sub => "sub",
            Operator::Mul => "mul",
            Operator::Div => "div",
            Operator::Mod => "mod",
            Operator::Eq => "eq",
            Operator::Gr => "gr",
            Operator::Lw => "lw",
            Operator::Ge => "ge",
            Operator::Le => "le",
            Operator::And => "and",
            Operator::Or => "or",
            Operator::Inv => "inv",
            Operator::Len => "len",
        }.to_string()
    }


    pub fn from_str(str: &str) -> Result<Operator, String> {
        let val = match str {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            "%" => Operator::Mod,
            "==" => Operator::Eq,
            ">" => Operator::Gr,
            "<" => Operator::Lw,
            ">=" => Operator::Ge,
            "<=" => Operator::Le,
            "&" => Operator::And,
            "?" => Operator::Or,
            "!" => Operator::Inv,
            "#" => Operator::Len,
            _ => return Err(format!("Unimplemented operator '{}'", str))
        };
        Ok(val)
    }
}