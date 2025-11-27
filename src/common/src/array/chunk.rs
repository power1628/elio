use crate::array::ArrayImpl;

#[derive(Clone)]
pub struct DataChunk {
    columns: Vec<ArrayImpl>,
    // TODO(pgao): selection vector
    cardinality: usize,
}

impl DataChunk {
    pub fn new(columns: Vec<ArrayImpl>, cardinality: usize) -> Self {
        Self { columns, cardinality }
    }

    pub fn len(&self) -> usize {
        self.cardinality
    }

    pub fn is_empty(&self) -> bool {
        self.cardinality == 0
    }

    pub fn column(&self, idx: usize) -> &ArrayImpl {
        &self.columns[idx]
    }

    pub fn add_column(&mut self, col: ArrayImpl) {
        self.columns.push(col);
    }
}
