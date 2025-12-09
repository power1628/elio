use std::sync::Arc;

use bitvec::vec::BitVec;

use crate::array::datum::Row;
use crate::array::{ArrayImpl, ArrayRef};

pub struct DataChunk {
    columns: Vec<Arc<ArrayImpl>>,
    visibility: Arc<BitVec>,
}

impl DataChunk {
    pub fn unit() -> Self {
        Self {
            columns: vec![],
            visibility: Arc::new(BitVec::repeat(true, 1)),
        }
    }

    pub fn new(columns: Vec<Arc<ArrayImpl>>) -> Self {
        if columns.is_empty() {
            return Self {
                columns: Vec::new(),
                visibility: Arc::new(BitVec::new()),
            };
        }

        let len = columns.first().unwrap().len();
        Self {
            columns,
            visibility: Arc::new(BitVec::repeat(true, len)),
        }
    }

    pub fn add_column(&mut self, column: Arc<ArrayImpl>) {
        assert_eq!(column.len(), self.len());
        self.columns.push(column);
    }

    pub fn with_visibility(self, visibility: BitVec) -> Self {
        Self {
            visibility: Arc::new(visibility),
            ..self
        }
    }

    pub fn column(&self, idx: usize) -> ArrayRef {
        self.columns[idx].clone()
    }

    pub fn visibility(&self) -> &BitVec {
        &self.visibility
    }

    pub fn len(&self) -> usize {
        self.visibility.len()
    }

    // valid row len
    pub fn visible_row_len(&self) -> usize {
        self.visibility.count_ones()
    }

    pub fn iter(&self) -> ChunkIter<'_> {
        ChunkIter {
            chunk: self,
            idx: 0,
            len: self.visibility.len(),
        }
    }
}

pub struct ChunkIter<'a> {
    chunk: &'a DataChunk,
    idx: usize,
    // visbility map len
    len: usize,
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.len {
            if self.chunk.visibility[self.idx] {
                let mut row = vec![];
                for col in self.chunk.columns.iter() {
                    row.push(col.get(self.idx).map(|x| x.to_owned_value()));
                }
                self.idx += 1;
                return Some(row);
            }
            self.idx += 1;
        }
        None
    }
}
