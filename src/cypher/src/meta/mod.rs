use crate::error::PlanError;
use mojito_common::LabelId;
use mojito_storage::codec::TokenKind;

pub trait MetaReader {
    fn get_label(&self, label: &str) -> Result<Option<LabelId>, PlanError>;
    fn get_rel_type(&self, rel: &str) -> Result<Option<LabelId>, PlanError>;
}
