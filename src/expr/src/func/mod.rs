pub mod operator;
pub mod sig;

pub mod temporal;

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::func::sig::FuncDef;

// Global Function Registry

pub static FUNCTION_REGISTRY: LazyLock<HashMap<String, FuncDef>> = LazyLock::new(|| {
    let mut registry = HashMap::new();
    // register scalar functions
    temporal::register(&mut registry);

    // register agg functions
    HashMap::new()
});
