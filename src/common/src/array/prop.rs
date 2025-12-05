use std::sync::Arc;

use crate::array::{Array, ArrayBuilder, ArrayIterator};
use crate::data_type::DataType;
use crate::store_types::PropertyValue;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PropertyArray {
    data: Arc<[PropertyValue]>,
}

impl Array for PropertyArray {
    type Builder = PropertyArrayBuilder;
    type OwnedItem = PropertyValue;
    type RefItem<'a> = &'a PropertyValue;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        // note: propertyvalue contains null
        Some(&self.data[idx])
    }

    unsafe fn get_unchecked(&self, idx: usize) -> Self::RefItem<'_> {
        // note: propertyvalue contains null
        &self.data[idx]
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn iter(&self) -> super::ArrayIterator<'_, Self> {
        ArrayIterator::new(self)
    }

    fn data_type(&self) -> DataType {
        DataType::Property
    }
}

#[derive(Debug)]
pub struct PropertyArrayBuilder {
    buffer: Vec<PropertyValue>,
}

impl ArrayBuilder for PropertyArrayBuilder {
    type Array = PropertyArray;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
        }
    }

    fn append_n(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>, repeat: usize) {
        assert!(value.is_some());
        self.buffer.extend(std::iter::repeat_n(value.unwrap().clone(), repeat));
    }

    fn finish(self) -> Self::Array {
        Self::Array {
            data: self.buffer.into(),
        }
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::array::ArrayBuilder;
    use crate::data_type::DataType;
    use crate::store_types::PropertyValue;

    #[test]
    fn test_property_array_builder() {
        let mut builder = PropertyArrayBuilder::with_capacity(4);
        let prop1 = PropertyValue::Integer(123);
        let prop2 = PropertyValue::String("test".to_string());
        let prop3 = PropertyValue::Null;
        let prop4 = PropertyValue::Float(45.67.into());

        builder.append(Some(&prop1));
        builder.append(Some(&prop2));
        builder.append(Some(&prop3));
        builder.append(Some(&prop4));

        assert_eq!(builder.len(), 4);

        let arr = builder.finish();
        assert_eq!(arr.len(), 4);
        assert_eq!(arr.data_type(), DataType::Property);

        assert_eq!(arr.get(0), Some(&prop1));
        assert_eq!(arr.get(1), Some(&prop2));
        assert_eq!(arr.get(2), Some(&prop3));
        assert_eq!(arr.get(3), Some(&prop4));

        unsafe {
            assert_eq!(arr.get_unchecked(0), &prop1);
            assert_eq!(arr.get_unchecked(1), &prop2);
            assert_eq!(arr.get_unchecked(2), &prop3);
            assert_eq!(arr.get_unchecked(3), &prop4);
        }
    }

    #[test]
    fn test_property_array_iter() {
        let mut builder = PropertyArrayBuilder::with_capacity(3);
        let prop1 = PropertyValue::Boolean(true);
        let prop2 = PropertyValue::Null;
        let prop3 = PropertyValue::String("another".to_string());

        builder.append(Some(&prop1));
        builder.append(Some(&prop2));
        builder.append(Some(&prop3));

        let arr = builder.finish();
        let mut iter = arr.iter();

        assert_eq!(iter.next(), Some(Some(&prop1)));
        assert_eq!(iter.next(), Some(Some(&prop2)));
        assert_eq!(iter.next(), Some(Some(&prop3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_empty_property_array() {
        let builder = PropertyArrayBuilder::with_capacity(0);
        let arr = builder.finish();
        assert!(arr.is_empty());
        assert_eq!(arr.len(), 0);
    }

    // #[test]
    // #[should_panic(expected = "assertion `left == right` failed")]
    // fn test_builder_with_wrong_type() {
    //     // This should panic because we are not passing DataType::Property
    //     let _builder = PropertyArrayBuilder::with_capacity(5);
    // }

    // #[test]
    // #[should_panic(expected = "assertion failed: value.is_some()")]
    // fn test_pushing_none_panics() {
    //     let mut builder = PropertyArrayBuilder::with_capacity(1, DataType::Property);
    //     builder.append(None);
    // }
}
