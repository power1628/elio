use crate::array::buffer::{Buffer, BufferMut};
use crate::array::mask::{Mask, MaskMut};
use crate::array::{Array, ArrayBuilder, ArrayIterator};
use crate::data_type::DataType;

#[derive(Clone, PartialEq, Eq, Hash)]
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
            unsafe { Some(std::str::from_utf8_unchecked(&self.data[start..end])) }
        } else {
            None
        }
    }

    unsafe fn get_unchecked(&self, idx: usize) -> Self::RefItem<'_> {
        let start = self.offsets[idx] as usize;
        let end = self.offsets[idx + 1] as usize;
        unsafe { std::str::from_utf8_unchecked(&self.data[start..end]) }
    }

    fn len(&self) -> usize {
        self.valid.len()
    }

    fn iter(&self) -> super::ArrayIterator<'_, Self> {
        ArrayIterator::new(self)
    }

    fn data_type(&self) -> DataType {
        DataType::String
    }
}

#[derive(Debug)]
pub struct StringArrayBuilder {
    data: BufferMut<u8>,
    offsets: BufferMut<u32>,
    valid: MaskMut,
}

impl ArrayBuilder for StringArrayBuilder {
    type Array = StringArray;

    fn with_capacity(capacity: usize, typ: DataType) -> Self {
        assert_eq!(typ, DataType::String);
        let mut offsets = BufferMut::with_capacity(capacity + 1);
        offsets.push(0);
        Self {
            data: BufferMut::with_capacity(capacity),
            offsets,
            valid: MaskMut::with_capacity(capacity),
        }
    }

    fn append_n(&mut self, value: Option<<Self::Array as super::Array>::RefItem<'_>>, repeat: usize) {
        if let Some(value) = value {
            let bytes = value.as_bytes();
            for _ in 0..repeat {
                self.data.extend_from_slice(bytes);
                self.offsets.push(self.data.len() as u32);
            }
            self.valid.append_n(true, repeat);
        } else {
            self.offsets.push_n(self.data.len() as u32, repeat);
            self.valid.append_n(false, repeat);
        }
    }

    fn finish(self) -> Self::Array {
        let data = self.data.freeze();
        let offsets = self.offsets.freeze();
        let valid = self.valid.freeze();
        Self::Array { data, offsets, valid }
    }

    fn len(&self) -> usize {
        self.offsets.len() - 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::array::ArrayBuilder;
    use crate::data_type::DataType;

    #[test]
    fn test_string_array_builder() {
        let mut builder = StringArrayBuilder::with_capacity(5, DataType::String);
        builder.append(Some("hello"));
        builder.append(Some("world"));
        builder.append(None);
        builder.append(Some("!"));
        builder.append(None);

        assert_eq!(builder.len(), 5);

        let arr = builder.finish();

        assert_eq!(arr.len(), 5);
        assert_eq!(arr.get(0), Some("hello"));
        assert_eq!(arr.get(1), Some("world"));
        assert_eq!(arr.get(2), None);
        assert_eq!(arr.get(3), Some("!"));
        assert_eq!(arr.get(4), None);

        unsafe {
            assert_eq!(arr.get_unchecked(0), "hello");
            assert_eq!(arr.get_unchecked(1), "world");
            assert_eq!(arr.get_unchecked(3), "!");
            // For None values, get_unchecked will return an empty string
            // because the start and end offsets will be the same.
            assert_eq!(arr.get_unchecked(2), "");
            assert_eq!(arr.get_unchecked(4), "");
        }
    }

    #[test]
    fn test_string_array_iter() {
        let mut builder = StringArrayBuilder::with_capacity(3, DataType::String);
        builder.append(Some("first"));
        builder.append(None);
        builder.append(Some("third"));
        let arr = builder.finish();

        let mut iter = arr.iter();
        assert_eq!(iter.next(), Some(Some("first")));
        assert_eq!(iter.next(), Some(None));
        assert_eq!(iter.next(), Some(Some("third")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_string_array_empty() {
        let builder = StringArrayBuilder::with_capacity(0, DataType::String);
        let arr = builder.finish();
        assert!(arr.is_empty());
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_string_array_with_empty_string() {
        let mut builder = StringArrayBuilder::with_capacity(2, DataType::String);
        builder.append(Some(""));
        builder.append(Some("not empty"));
        let arr = builder.finish();

        assert_eq!(arr.len(), 2);
        assert_eq!(arr.get(0), Some(""));
        assert_eq!(arr.get(1), Some("not empty"));
    }

    #[test]
    fn test_all_nulls() {
        let mut builder = StringArrayBuilder::with_capacity(3, DataType::String);
        builder.append(None);
        builder.append(None);
        builder.append(None);
        let arr = builder.finish();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr.get(0), None);
        assert_eq!(arr.get(1), None);
        assert_eq!(arr.get(2), None);
    }

    #[test]
    #[should_panic]
    fn test_string_array_builder_wrong_type() {
        let _builder = StringArrayBuilder::with_capacity(5, DataType::Bool);
    }
}
