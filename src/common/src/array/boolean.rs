use crate::array::mask::{Mask, MaskMut};
use crate::array::{Array, ArrayBuilder, ArrayIterator};
use crate::data_type::DataType;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::array::ArrayBuilder;
    use crate::data_type::DataType;

    #[test]
    fn test_bool_array_builder_push_and_finish() {
        let mut builder = BoolArrayBuilder::with_capacity(5, DataType::Bool);
        builder.push(Some(true));
        builder.push(Some(false));
        builder.push(None);
        builder.push(Some(true));
        builder.push(None);

        let arr = builder.finish();

        assert_eq!(arr.len(), 5);
        assert_eq!(arr.get(0), Some(true));
        assert_eq!(arr.get(1), Some(false));
        assert_eq!(arr.get(2), None);
        assert_eq!(arr.get(3), Some(true));
        assert_eq!(arr.get(4), None);

        // Test unsafe get_unchecked
        unsafe {
            assert!(arr.get_unchecked(0));
            assert!(!arr.get_unchecked(1));
            // Note: get_unchecked for index 2 and 4 would return `false` as per current `MaskMut::push(false)` for None
            // values. This is expected behavior for `get_unchecked` as it doesn't check validity.
            assert!(!arr.get_unchecked(2));
            assert!(arr.get_unchecked(3));
            assert!(!arr.get_unchecked(4));
        }
    }

    #[test]
    fn test_bool_array_all_valid() {
        let mut builder = BoolArrayBuilder::with_capacity(3, DataType::Bool);
        builder.push(Some(true));
        builder.push(Some(false));
        builder.push(Some(true));
        let arr = builder.finish();

        assert_eq!(arr.len(), 3);
        assert_eq!(arr.get(0), Some(true));
        assert_eq!(arr.get(1), Some(false));
        assert_eq!(arr.get(2), Some(true));
    }

    #[test]
    fn test_bool_array_all_invalid() {
        let mut builder = BoolArrayBuilder::with_capacity(3, DataType::Bool);
        builder.push(None);
        builder.push(None);
        builder.push(None);
        let arr = builder.finish();

        assert_eq!(arr.len(), 3);
        assert_eq!(arr.get(0), None);
        assert_eq!(arr.get(1), None);
        assert_eq!(arr.get(2), None);
    }

    #[test]
    fn test_bool_array_iter() {
        let mut builder = BoolArrayBuilder::with_capacity(5, DataType::Bool);
        builder.push(Some(true));
        builder.push(Some(false));
        builder.push(None);
        builder.push(Some(true));
        builder.push(None);
        let arr = builder.finish();

        let mut iter = arr.iter();
        assert_eq!(iter.next(), Some(Some(true)));
        assert_eq!(iter.next(), Some(Some(false)));
        assert_eq!(iter.next(), Some(None));
        assert_eq!(iter.next(), Some(Some(true)));
        assert_eq!(iter.next(), Some(None));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_bool_array_is_empty() {
        let builder = BoolArrayBuilder::with_capacity(0, DataType::Bool);
        let arr = builder.finish();
        assert!(arr.is_empty());

        let mut builder = BoolArrayBuilder::with_capacity(1, DataType::Bool);
        builder.push(Some(true));
        let arr = builder.finish();
        assert!(!arr.is_empty());
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed\n  left: Integer\n right: Bool")]
    fn test_bool_array_builder_with_capacity_wrong_type() {
        let _builder = BoolArrayBuilder::with_capacity(5, DataType::Integer);
    }
}
