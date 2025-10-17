use mojito_common::data_type::DataType;

/// Logical expr
pub mod func_call;
pub mod subquery;
pub mod value;
pub mod variable_ref;
use func_call::*;
use subquery::*;
use value::*;
use variable_ref::*;

pub enum Expr {
    VariableRef(VariableRef),
    Value(Value),
    FuncCall(FuncCall),
    Subquery(Subquery),
}

pub trait ExprNode {
    fn typ(&self) -> DataType;
}

macro_rules! impl_expr_node_for_enum {
    ($enum_name:ident, $($variant:ident),+) => {
        impl ExprNode for $enum_name {
            fn typ(&self) -> DataType {
                match self {
                    $(
                        Self::$variant(expr) => expr.typ(),
                    )+
                }
            }
        }
    };
}

impl_expr_node_for_enum!(Expr, VariableRef, Value, FuncCall, Subquery);
