use crate::array::mask::{Mask, MaskMut};
use crate::array::{Array, ArrayBuilder, ArrayIterator};
use crate::data_type::DataType;

#[derive(Clone, Debug)]
pub struct BoolArray {
    bits: Mask,
    valid: Mask,
}

impl Array for BoolArray {
    type Builder = BoolArrayBuilder;
    type OwnedItem = bool;
    type RefItem<'a> = bool;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        if self.valid.get(idx) {
            Some(self.bits.get(idx))
        } else {
            None
        }
    }

    unsafe fn get_unchecked(&self, idx: usize) -> Self::RefItem<'_> {
        self.bits.get(idx)
    }

    fn len(&self) -> usize {
        self.valid.len()
    }

    fn iter(&self) -> super::ArrayIterator<'_, Self> {
        ArrayIterator::new(self)
    }

    fn data_type(&self) -> crate::data_type::DataType {
        DataType::Bool
    }
}

pub struct BoolArrayBuilder {
    data: MaskMut,
    valid: MaskMut,
}

impl ArrayBuilder for BoolArrayBuilder {
    type Array = BoolArray;

    fn with_capacity(capacity: usize, typ: DataType) -> Self {
        assert_eq!(typ, DataType::Bool);
        Self {
            data: MaskMut::with_capacity(capacity),
            valid: MaskMut::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Option<<Self::Array as super::Array>::RefItem<'_>>) {
        match value {
            Some(v) => {
                self.data.push(v);
                self.valid.push(true);
            }
            None => {
                self.data.push(false);
                self.valid.push(false);
            }
        }
    }

    fn finish(self) -> Self::Array {
        Self::Array {
            bits: self.data.freeze(),
            valid: self.valid.freeze(),
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}
