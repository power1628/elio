use crate::array::ArrayImpl;
use crate::scalar::Row;

#[derive(Clone)]
pub struct DataChunk {
    columns: Vec<ArrayImpl>,
    // TODO(pgao): selection vector
    cardinality: usize,
}

impl DataChunk {
    // return one empty row
    pub fn unit() -> Self {
        Self::new(vec![], 1)
    }

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

impl DataChunk {
    pub fn iter(&self) -> impl Iterator<Item = Row> + '_ {
        (0..self.cardinality).map(|idx| self.get_row_by_idx(idx))
    }

    pub fn get_row_by_idx(&self, idx: usize) -> Row {
        let mut row = Row::new();
        for col in &self.columns {
            row.push(col.get(idx).map(|x| x.to_owned_scalar()));
        }
        row
    }
}
