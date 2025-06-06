use derive_more::Display;

#[derive(Debug, Display)]
#[display("{}", _0)]
pub enum Expr {
    Literal(Literal),
    Varaible(String),
    Parameter(String),
    PropertyKey(String),
    PropertyAccess {
        variable: Box<Expr>,
        key: Box<Expr>,
    },
    Unary {
        op: UnaryOperator,
        oprand: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    Case {
        expr: Option<Box<Expr>>,
        alternatives: Vec<(Expr, Expr)>,
        default: Option<Box<Expr>>,
    },
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
    #[display("INF")]
    Inf,
}

macro_rules! unary_operator {
    ($($variant:ident => $sym:expr),* $(,)?) => {
        #[derive(Debug)]
        pub enum UnaryOperator {
            $($variant),*
        }
        impl std::fmt::Display for UnaryOperator {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant => write!(f, $sym),)*
                }
            }
        }
    };
}

macro_rules! binary_operator {
    ($($variant:ident => $sym:expr),* $(,)?) => {
        #[derive(Debug)]
        pub enum BinaryOperator {
            $($variant),*
        }

        impl std::fmt::Display for BinaryOperator {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant => write!(f, $sym),)*
                }
            }
        }
    };
}

unary_operator! {
    UnaryAdd => "+",
    UnarySubtract => "-",
    Not => "NOT",
    IsNull => "IS NULL",
    IsNotNull => "IS NOT NULL",
}

binary_operator! {
    // artimetic
    Add => "+",
    Subtract => "-",
    Multiply => "*",
    Divide => "/",
    Modulo => "%",
    Pow => "^",
    // list
    Concat => "||",
    // logical
    Or => "OR",
    Xor => "XOR",
    And => "AND",
    // comparison
    Eq => "=",
    NotEq => "<>",
    Gt => ">",
    GtEq => ">=",
    Lt => "<",
    LtEq => "<=",
    // comparasion2
    StartsWith => "STARTS WITH",
    EndsWith => "ENDS WITH",
    Contains => "CONTAINS",
    In => "IN",
}
