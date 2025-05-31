use derive_more::Display;

#[derive(Debug, Display)]
#[display("{}", _0)]
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

#[derive(Debug, Display)]
pub enum Literal {
    #[display("{}", _0)]
    Boolean(bool),
    #[display("{}", _0)]
    Integer(String), // un-parsed integer
    #[display("{}", _0)]
    Float(String), // un-parsed float
    #[display("{}", _0)]
    String(String),
    #[display("NULL")]
    Null,
}
