use std::sync::Arc;

use bitvec::vec::BitVec;

use crate::array::{ArrayImpl, ArrayRef};

pub struct DataChunk {
    columns: Vec<Arc<ArrayImpl>>,
    visibility: Arc<BitVec>,
}

impl DataChunk {
    pub fn new(columns: impl Iterator<Item = ArrayImpl>, visibility: BitVec) -> Self {
        Self {
            columns: columns.map(Arc::new).collect(),
            visibility: Arc::new(visibility),
        }
    }

    pub fn column(&self, idx: usize) -> ArrayRef {
        self.columns[idx].clone()
    }

    pub fn visibility(&self) -> &BitVec {
        &self.visibility
    }

    // valid row len
    pub fn row_len(&self) -> usize {
        self.visibility.count_ones()
    }
}
