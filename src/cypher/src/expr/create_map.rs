use mojito_common::data_type::DataType;

use crate::expr::{BoxedExpr, Expr, ExprNode, IrToken};

/// Create Property Map
#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct CreateMap {
    pub properties: Vec<(IrToken, BoxedExpr)>,
}

impl CreateMap {
    pub fn new(properties: Vec<(IrToken, BoxedExpr)>) -> Self {
        Self { properties }
    }
}

impl ExprNode for CreateMap {
    fn typ(&self) -> DataType {
        DataType::PropertyMap
    }
}

impl From<CreateMap> for Expr {
    fn from(value: CreateMap) -> Self {
        Expr::CreateMap(value)
    }
}
