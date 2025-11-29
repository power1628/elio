use mojito_common::IrToken;
use mojito_common::data_type::DataType;

use crate::expr::{Expr, ExprNode};

/// Create Property Map
/// TODO(pgao): should guaranteee Expr return type is only can be viewed as propety value types
#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct CreateMap {
    pub properties: Vec<(IrToken, Expr)>,
}

impl CreateMap {
    pub fn new(properties: Vec<(IrToken, Expr)>) -> Self {
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
