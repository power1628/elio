use crate::array::{ArrayImpl, mask::Mask};

#[derive(Clone)]
pub struct DataChunk {
    pub columns: Vec<ArrayImpl>,
    // selection mask
    pub sel: Mask,
}

impl DataChunk {
    pub fn into_parts(self) -> (Vec<ArrayImpl>, Mask) {
        (self.columns, self.sel)
    }
}
