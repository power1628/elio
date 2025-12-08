use core::error;
use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub enum EvalError {
    #[error("get or create token failed {0}")]
    GetOrCreateTokenError(String),
    #[error("mapb error {0}")]
    MapbError(String),
    #[error("type error {0}")]
    TypeError(String),
    #[error("field not found {0}")]
    FieldNotFound(String),
}

impl EvalError {
    pub fn get_or_create_token_error(key: &str) -> Self {
        Self::GetOrCreateTokenError(key.to_string())
    }

    pub fn mapb_error(msg: &str) -> Self {
        Self::MapbError(msg.to_string())
    }

    pub fn type_error<T: Display>(msg: T) -> Self {
        Self::TypeError(msg.to_string())
    }
}
