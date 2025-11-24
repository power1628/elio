use std::sync::Arc;

use crate::array::{Array, ArrayBuilder};
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
        todo!()
    }

    unsafe fn get_unchecked(&self, idx: usize) -> Self::RefItem<'_> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn iter(&self) -> super::ArrayIterator<'_, Self> {
        todo!()
    }

    fn data_type(&self) -> crate::data_type::DataType {
        todo!()
    }
}

pub struct PropertyArrayBuilder {
    buffer: Vec<PropertyValue>,
}

impl ArrayBuilder for PropertyArrayBuilder {
    type Array = PropertyArray;

    fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>) {
        todo!()
    }

    fn finish(self) -> Self::Array {
        todo!()
    }
}
