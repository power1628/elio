use std::sync::{Arc, atomic::AtomicUsize};

use mojito_common::data_type::DataType;

pub type VariableName = Arc<str>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub name: VariableName,
    pub typ: DataType,
}

impl Variable {
    pub fn new(name: &VariableName, typ: &DataType) -> Self {
        Self {
            name: name.clone(),
            typ: typ.clone(),
        }
    }
}

#[derive(Debug, Default)]
pub struct VariableGenerator {
    next_id: AtomicUsize,
}

impl VariableGenerator {
    pub fn next_name(&self) -> VariableName {
        let id = self.next_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("v{}", id).into()
    }
}
