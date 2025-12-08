use std::sync::Arc;

use bitvec::vec::BitVec;

use crate::array::ArrayImpl;

pub struct DataChunk {
    columns: Vec<Arc<ArrayImpl>>,
    visibility: Arc<BitVec>,
}

impl DataChunk {
    pub fn new(columns: Vec<ArrayImpl>, visibility: BitVec) -> Self {
        Self {
            columns: columns.into_iter().map(Arc::new).collect(),
            visibility: Arc::new(visibility),
        }
    }

    pub fn column(&self, idx: usize) -> &ArrayImpl {
        &self.columns[idx]
    }

    pub fn visibility(&self) -> &BitVec {
        &self.visibility
    }
}
