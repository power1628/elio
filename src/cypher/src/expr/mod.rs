use enum_as_inner::EnumAsInner;
use mojito_common::IrToken;
use mojito_common::data_type::DataType;
use mojito_common::schema::Variable;

pub mod agg_call;
pub mod create_map;
pub mod filters;
/// Logical expr
pub mod func_call;
pub mod label;
pub mod property_access;
pub mod subquery;
pub mod utils;
pub mod value;
pub mod variable_ref;
pub use agg_call::*;
pub use create_map::*;
pub use filters::*;
pub use func_call::*;
pub use label::*;
pub use property_access::*;
pub use subquery::*;
pub use value::*;
pub use variable_ref::*;

#[derive(Debug, Hash, Clone, Eq, PartialEq, EnumAsInner)]
pub enum Expr {
    VariableRef(VariableRef),
    PropertyAccess(PropertyAccess),
    Constant(Constant),
    FuncCall(FuncCall),
    AggCall(AggCall),
    Subquery(Subquery),
    Label(LabelExpr),
    CreateMap(CreateMap),
}

pub type BoxedExpr = Box<Expr>;

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

impl_expr_node_for_enum!(
    Expr,
    VariableRef,
    PropertyAccess,
    Constant,
    FuncCall,
    AggCall,
    Subquery,
    Label,
    CreateMap
);

impl Expr {
    #[inline]
    pub fn boxed(self) -> Box<Expr> {
        Box::new(self)
    }

    #[inline]
    pub fn from_variable(var: &Variable) -> Self {
        Expr::VariableRef(VariableRef::new_unchecked(var.name.clone(), var.typ.clone()))
    }
}

impl Expr {
    pub fn boolean(b: bool) -> Self {
        Expr::Constant(Constant::boolean(b))
    }

    pub fn and(self, rhs: Self) -> Self {
        Expr::FuncCall(FuncCall::new_unchecked(
            "AND".to_string(),
            vec![self, rhs],
            DataType::Bool,
        ))
    }

    pub fn or(self, rhs: Self) -> Self {
        Expr::FuncCall(FuncCall::new_unchecked(
            "OR".to_string(),
            vec![self, rhs],
            DataType::Bool,
        ))
    }

    pub fn equal(self, rhs: Self) -> Self {
        Expr::FuncCall(FuncCall::new_unchecked(
            "EQ".to_string(),
            vec![self, rhs],
            DataType::Bool,
        ))
    }

    pub fn property(self, prop: &IrToken, typ: &DataType) -> Self {
        Expr::PropertyAccess(PropertyAccess::new_unchecked(self.boxed(), prop, typ))
    }
}
