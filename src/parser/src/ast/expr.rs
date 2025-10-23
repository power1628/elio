use derive_more::Display;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    Literal {
        lit: Literal,
    },
    Varaible {
        name: String,
    },
    Parameter {
        name: String,
    },
    MapExpression {
        keys: Vec<String>,
        values: Vec<Expr>,
    },
    PropertyAccess {
        map: Box<Expr>,
        key: String,
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
}

impl Expr {
    pub fn new_boolean(value: bool) -> Self {
        Expr::Literal {
            lit: Literal::Boolean(value),
        }
    }
    pub fn new_integer(value: String) -> Self {
        Expr::Literal {
            lit: Literal::Integer(value),
        }
    }
    pub fn new_float(value: String) -> Self {
        Expr::Literal {
            lit: Literal::Float(value),
        }
    }
    pub fn new_string(value: String) -> Self {
        Expr::Literal {
            lit: Literal::String(value),
        }
    }
    pub fn new_null() -> Self {
        Expr::Literal { lit: Literal::Null }
    }
    pub fn new_variable(name: String) -> Self {
        Expr::Varaible { name }
    }
    pub fn new_parameter(name: String) -> Self {
        Expr::Parameter { name }
    }
    pub fn new_map_expression(keys: Vec<String>, values: Vec<Expr>) -> Self {
        Expr::MapExpression { keys, values }
    }
    pub fn new_property_access(map: Expr, key: String) -> Self {
        Expr::PropertyAccess {
            map: Box::new(map),
            key,
        }
    }
    pub fn new_unary(op: UnaryOperator, oprand: Expr) -> Self {
        Expr::Unary {
            op,
            oprand: Box::new(oprand),
        }
    }
    pub fn new_binary(left: Expr, op: BinaryOperator, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
    pub fn new_function_call(name: String, args: Vec<Expr>) -> Self {
        Expr::FunctionCall { name, args }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal { lit } => write!(f, "{lit}"),
            Expr::Varaible { name } => write!(f, "{name}"),
            Expr::MapExpression { keys, values } => {
                write!(
                    f,
                    "{{{}}}",
                    keys.iter()
                        .zip(values)
                        .map(|(k, v)| format!("{k}: {v}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Expr::Parameter { name } => write!(f, "${name}"),
            Expr::PropertyAccess { map, key } => write!(f, "{map}.{key}"),
            Expr::Unary { op, oprand } => match op.associativity() {
                Associativity::Prefix => write!(f, "{op}({oprand})"),
                Associativity::Postfix => write!(f, "({oprand}){op}"),
            },
            Expr::Binary { left, op, right } => {
                write!(f, "({left}) {op} ({right})")
            }
            Expr::FunctionCall { name, args } => {
                write!(
                    f,
                    "{}({})",
                    name,
                    args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>().join(", ")
                )
            }
        }
    }
}

#[derive(Debug, Display, PartialEq, Eq, Hash)]
pub enum Literal {
    #[display("{}", if *_0 { "TRUE" } else { "FALSE" })]
    Boolean(bool),
    #[display("{}", _0)]
    Integer(String), // un-parsed integer
    #[display("{}", _0)]
    Float(String), // un-parsed float
    #[display("\'{}\'", _0)]
    String(String),
    #[display("NULL")]
    Null,
    #[display("INF")]
    Inf,
}

enum Associativity {
    Prefix,
    Postfix,
}

macro_rules! unary_operator {
    ($($variant:ident => $sym:expr, $assoc:expr),* $(,)?) => {
        #[derive(Debug, PartialEq, Eq, Hash)]
        pub enum UnaryOperator {
            $($variant),*
        }

        impl UnaryOperator {
            fn associativity(&self) -> Associativity {
                match self {
                    $(Self::$variant => $assoc),*
                }
            }
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
        #[derive(Debug, PartialEq, Eq, Hash)]
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
    UnaryAdd => "+", Associativity::Prefix,
    UnarySubtract => "-", Associativity::Prefix,
    Not => "NOT", Associativity::Prefix,
    IsNull => "IS NULL",   Associativity::Postfix,
    IsNotNull => "IS NOT NULL",   Associativity::Postfix,
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

#[derive(Debug)]
pub enum LabelExpr {
    Label(String),
    Or(Box<LabelExpr>, Box<LabelExpr>),
}

impl LabelExpr {
    pub fn new_label(label: String) -> Self {
        Self::Label(label)
    }
    pub fn new_or(left: Self, right: Self) -> Self {
        Self::Or(Box::new(left), Box::new(right))
    }
}

impl std::fmt::Display for LabelExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Label(label) => write!(f, "{label}"),
            Self::Or(left, right) => write!(f, "{left} | {right}"),
        }
    }
}
