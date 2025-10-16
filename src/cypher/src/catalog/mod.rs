use crate::error::PlanError;
use mojito_common::LabelId;
pub mod func;

pub trait MetaReader {
    fn get_label(&self, label: &str) -> Result<Option<LabelId>, PlanError>;
    fn get_rel_type(&self, rel: &str) -> Result<Option<LabelId>, PlanError>;
}

pub trait CatalogReader {
    fn resolve_function(&self, name: &str) -> 
}
