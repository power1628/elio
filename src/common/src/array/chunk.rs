use crate::array::ArrayImpl;

#[derive(Clone)]
pub struct DataChunk {
    pub columns: Vec<ArrayImpl>,
    // TODO(pgao): selection vector
}

impl DataChunk {
    pub fn new(columns: Vec<ArrayImpl>) -> Self {
        Self { columns }
    }
}
