use std::sync::atomic::AtomicUsize;

use mojito_common::variable::VariableName;

#[derive(Debug, Default)]
pub struct VariableGenerator {
    next_id: AtomicUsize,
}

impl VariableGenerator {
    pub fn named(&self, hint: &str) -> VariableName {
        let id = self.next_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("{hint}@{}", id).into()
    }

    pub fn unnamed(&self) -> VariableName {
        let id = self.next_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("anon@{}", id).into()
    }
}
