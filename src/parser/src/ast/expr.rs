#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
}

impl Expr {
    pub fn new_boolean(value: bool) -> Self {
        Expr::Literal(Literal::Boolean(value))
    }
    pub fn new_integer(value: String) -> Self {
        Expr::Literal(Literal::Integer(value))
    }
    pub fn new_float(value: String) -> Self {
        Expr::Literal(Literal::Float(value))
    }
    pub fn new_string(value: String) -> Self {
        Expr::Literal(Literal::String(value))
    }
    pub fn new_null() -> Self {
        Expr::Literal(Literal::Null)
    }
}

#[derive(Debug)]
pub enum Literal {
    Boolean(bool),
    Integer(String), // un-parsed integer
    Float(String),   // un-parsed float
    String(String),
    Null,
}
