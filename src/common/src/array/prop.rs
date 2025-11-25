use std::sync::Arc;

use crate::array::{Array, ArrayBuilder, ArrayIterator};
use crate::data_type::DataType;
use crate::store_types::PropertyValue;

#[derive(Debug, Clone)]
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

pub struct PropertyArrayBuilder {
    buffer: Vec<PropertyValue>,
}

impl ArrayBuilder for PropertyArrayBuilder {
    type Array = PropertyArray;

    fn with_capacity(capacity: usize, typ: DataType) -> Self {
        assert_eq!(typ, DataType::Property);
        Self {
            buffer: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>) {
        assert!(value.is_some());
        self.buffer.push(value.unwrap().clone());
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
