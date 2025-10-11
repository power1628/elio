use mojito_storage::error::GraphStoreError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlanError {
    #[error("meta access error")]
    MetaError(#[from] GraphStoreError),
    #[error("{}", _0.message)]
    SemanticError(#[from] SemanticError),
}

#[derive(Error, Debug)]
pub struct SemanticError {
    // TODO(pgao): add gql status here
    pub message: String,
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl SemanticError {}
