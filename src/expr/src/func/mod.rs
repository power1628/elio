pub mod operator;
pub mod sig;
// pub mod string;

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::func::sig::FuncDef;

// Global Function Registry

pub static FUNCTION_REGISTRY: LazyLock<HashMap<String, FuncDef>> = LazyLock::new(|| {
    // register scalar functions

    // string::register(&mut registry);
    // register agg functions
    HashMap::new()
});
