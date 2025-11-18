use crate::array::{
    Array, ArrayBuilder, ArrayIterator,
    buffer::{Buffer, BufferMut},
    mask::{Mask, MaskMut},
};

#[derive(Clone, Debug)]
pub struct StringArray {
    // string content
    data: Buffer<u8>,
    // string offset
    offsets: Buffer<u32>,
    valid: Mask,
}

impl Array for StringArray {
    type Builder = StringArrayBuilder;

    type OwnedItem = String;

    type RefItem<'a> = &'a str;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        if self.valid.get(idx) {
            let start = self.offsets[idx] as usize;
            let end = self.offsets[idx + 1] as usize;
            unsafe { std::str::from_utf8_unchecked(&self.data[start..end]) }
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.valid.len()
    }

    fn iter(&self) -> super::ArrayIterator<'_, Self> {
        ArrayIterator::new(self)
    }
}

pub struct StringArrayBuilder {
    data: BufferMut<u8>,
    offsets: BufferMut<u32>,
    valid: MaskMut,
}

impl ArrayBuilder for StringArrayBuilder {
    type Array = StringArray;

    fn with_capacity(capacity: usize) -> Self {
        let mut offsets = BufferMut::with_capacity(capacity + 1);
        offsets.push(0);
        Self {
            data: BufferMut::with_capacity(capacity),
            offsets,
            valid: MaskMut::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Option<<Self::Array as super::Array>::RefItem<'_>>) {
        if let Some(value) = value {
            self.data.extend_from_slice(value.as_bytes());
            self.offsets.push(self.data.len() as u32);
            self.valid.push(true);
        } else {
            self.valid.push(false);
        }
    }

    fn finish(self) -> Self::Array {
        let data = self.data.freeze();
        let offsets = self.offsets.freeze();
        let valid = self.valid.freeze();
        Self::Array { data, offsets, valid }
    }
}
