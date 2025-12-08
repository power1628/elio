use crate::array::ArrayImpl;
use crate::array::mask::Mask;
use crate::scalar::Row;

#[derive(Clone, Debug)]
pub struct DataChunk {
    columns: Vec<ArrayImpl>,
    // selection vector
    visibility: Mask,
    // valid number of rows
    cardinality: usize,
    capacity: usize,
}

impl DataChunk {
    // return one empty row
    pub fn unit() -> Self {
        Self {
            columns: vec![],
            visibility: Mask::new_set(1),
            cardinality: 1,
            capacity: 1,
        }
    }

    pub fn empty() -> Self {
        Self::new(vec![])
    }

    pub fn new(columns: Vec<ArrayImpl>) -> Self {
        let capacity = columns.first().map(|c| c.len()).unwrap_or(0);
        if cfg!(debug_assertions) {
            for col in columns.iter() {
                assert_eq!(col.len(), capacity)
            }
        }
        Self {
            columns,
            visibility: Mask::new_set(capacity),
            cardinality: capacity,
            capacity,
        }
    }

    pub fn row_len(&self) -> usize {
        self.cardinality
    }

    pub fn is_empty(&self) -> bool {
        self.cardinality == 0
    }

    pub fn column(&self, idx: usize) -> &ArrayImpl {
        &self.columns[idx]
    }

    pub fn columns(&self) -> &[ArrayImpl] {
        &self.columns
    }

    pub fn add_column(&mut self, col: ArrayImpl) {
        self.columns.push(col);
    }

    pub fn set_visibility(&mut self, visibility: Mask) {
        assert_eq!(self.capacity, visibility.len());
        self.cardinality = visibility.set_count();

        // if we already have visibility, we & it with the new visibility
        self.visibility = &self.visibility & &visibility;
    }

    pub fn is_visible(&self, idx: usize) -> bool {
        self.visibility.get(idx)
    }

    pub fn is_all_visible(&self) -> bool {
        self.visibility.all_set()
    }

    pub fn compact(self) -> Self {
        todo!()
    }
}

impl DataChunk {
    pub fn iter(&self) -> impl Iterator<Item = Row> + '_ {
        if self.is_all_visible() {
            (0..self.cardinality).map(|idx| self.get_row_by_idx(idx))
        } else {
            todo!()
        }
    }

    pub fn get_row_by_idx(&self, idx: usize) -> Row {
        let mut row = Row::new();
        for col in &self.columns {
            row.push(col.get(idx).map(|x| x.to_owned_scalar()));
        }
        row
    }
}
