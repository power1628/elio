use crate::{catalog::func::FunctionCatalog, error::PlanError};
use mojito_common::LabelId;
use mojito_expr::func::sig::FuncDef;
pub mod func;

pub trait CatalogReader {
    fn resolve_function(&self, name: &str) -> Result<Option<FunctionCatalog>, PlanError>;
    /// operator should be resolved in separate logic.
    fn resolve_operator(&self, op: &str) -> Result<Option<FuncDef>, PlanError>;
}
