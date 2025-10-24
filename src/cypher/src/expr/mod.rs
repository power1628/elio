use std::f32::consts::E;

use mojito_common::data_type::DataType;

/// Logical expr
pub mod func_call;
pub mod property_access;
pub mod subquery;
pub mod value;
pub mod variable_ref;
use func_call::*;
use property_access::*;
use subquery::*;
use value::*;
use variable_ref::*;
pub mod utils;
pub use utils::*;
pub mod label;
pub use label::*;

use crate::variable::{Variable, VariableName};

#[derive(Debug, Clone)]
pub enum Expr {
    VariableRef(VariableRef),
    PropertyAccess(PropertyAccess),
    Constant(Constant),
    FuncCall(FuncCall),
    Subquery(Subquery),
    Label(LabelExpr),
}

pub trait ExprNode: std::fmt::Debug + Clone {
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

impl_expr_node_for_enum!(Expr, VariableRef, PropertyAccess, Constant, FuncCall, Subquery, Label);

impl Expr {
    pub fn boxed(self) -> Box<Expr> {
        Box::new(self)
    }

    pub fn from_variable(var: &Variable) -> Self {
        Expr::VariableRef(VariableRef::new(var.name.clone(), var.typ.clone()))
    }
}

impl Expr {
    pub fn boolean(b: bool) -> Self {
        Expr::Constant(Constant::boolean(b))
    }

    pub fn and(self, rhs: Self) -> Self {
        Expr::FuncCall(FuncCall::new("AND".to_string(), vec![self, rhs], DataType::Boolean))
    }

    pub fn or(self, rhs: Self) -> Self {
        Expr::FuncCall(FuncCall::new("AND".to_string(), vec![self, rhs], DataType::Boolean))
    }

    pub fn equal(self, rhs: Self) -> Self {
        Expr::FuncCall(FuncCall::new("EQ".to_string(), vec![self, rhs], DataType::Boolean))
    }

    pub fn property(self, prop: &IrToken, typ: &DataType) -> Self {
        Expr::PropertyAccess(PropertyAccess::new_unchecked(self.boxed(), prop, typ))
    }
}
