use std::{collections::HashMap, ops::Deref, sync::Arc};

use mojito_common::TokenId;
use mojito_storage::{codec::TokenKind, meta::TokenStore};

pub mod func;
pub use func::FunctionCatalog;

/// Catalog contains
///  - Registered functions
///  - Token to TokenId Mapping
///  - #TODO(pgao): Constraints
///  - #TODO(pgao): index

pub struct Catalog {
    // token store with cache
    token: Arc<TokenStore>,
    // functions
    functions: HashMap<String, FunctionCatalog>,
}

impl std::fmt::Debug for Catalog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Catalog").finish()
    }
}

impl Catalog {
    pub fn new(token: Arc<TokenStore>) -> Self {
        Self {
            token,
            // TODO(pgao): register builtin functions here
            functions: HashMap::new(),
        }
    }

    pub fn get_function_by_name(&self, name: &str) -> Option<&FunctionCatalog> {
        self.functions.get(name)
    }
}

impl Deref for Catalog {
    type Target = TokenStore;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}
