use mojito_storage::error::GraphStoreError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlanError {
    #[error("meta access error")]
    MetaError(#[from] GraphStoreError),
    #[error("{}", _0.message)]
    SemanticError(#[from] SemanticError),
    #[error("{}", _0)]
    NotSupported(String),
}

impl PlanError {
    pub fn semantic_err<T: ToString>(msg: T) -> Self {
        Self::SemanticError(SemanticError::new(msg.to_string()))
    }

    pub fn not_supported<T: ToString>(msg: T) -> Self {
        Self::NotSupported(msg.to_string())
    }
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

impl SemanticError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
