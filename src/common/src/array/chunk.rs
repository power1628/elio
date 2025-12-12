use std::sync::Arc;

use bitvec::vec::BitVec;
use itertools::Itertools;

use crate::array::datum::ScalarRef;
use crate::array::{ArrayBuilderImpl, ArrayImpl, ArrayRef, PhysicalType};

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

        if cfg!(debug_assertions) {
            assert!(
                columns.iter().all(|c| c.len() == len),
                "All columns in a DataChunk must have the same length"
            );
        }
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
    type Item = Vec<Option<ScalarRef<'a>>>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.len {
            // TODO(pgao): optimize visibility map
            if self.chunk.visibility[self.idx] {
                let mut row = vec![];
                for col in self.chunk.columns.iter() {
                    row.push(col.get(self.idx));
                }
                self.idx += 1;
                return Some(row);
            }
            self.idx += 1;
        }
        None
    }
}

pub struct DataChunkBuilder {
    // array data types
    types: Vec<PhysicalType>,
    // chunk capacity, if full build the data chunk
    capacity: usize,

    columns: Vec<ArrayBuilderImpl>,
    len: usize,
}

impl DataChunkBuilder {
    pub fn new(types: impl Iterator<Item = PhysicalType>, capacity: usize) -> Self {
        let types = types.collect_vec();
        let columns = types.iter().map(|t| t.array_builder(capacity)).collect_vec();
        Self {
            types,
            capacity,
            columns,
            len: 0,
        }
    }

    pub fn append_row(&mut self, row: Vec<Option<ScalarRef<'_>>>) -> Option<DataChunk> {
        assert_eq!(row.len(), self.columns.len());
        for (col, item) in self.columns.iter_mut().zip_eq(row) {
            col.push(item);
        }
        self.len += 1;
        if self.len >= self.capacity {
            Some(self.build_chunk())
        } else {
            None
        }
    }

    pub fn yield_chunk(&mut self) -> Option<DataChunk> {
        if self.len == 0 { None } else { Some(self.build_chunk()) }
    }

    // build the data chunk and reset the builder
    fn build_chunk(&mut self) -> DataChunk {
        let builders = std::mem::take(&mut self.columns);
        let columns = builders.into_iter().map(|b| Arc::new(b.finish())).collect_vec();
        let chunk = DataChunk::new(columns);
        self.reset();
        chunk
    }

    fn reset(&mut self) {
        self.len = 0;
        self.columns.clear();
        for dtype in self.types.iter() {
            self.columns.push(dtype.array_builder(self.capacity));
        }
    }
}
