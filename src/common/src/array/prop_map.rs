//! AnyMap array contains AnyMap type scalar
//! Only used to represent properties.
//! Since AnyMap type contains heterogeneous key-value pairs,
//! we consider AnyMapArray as an leaf array.

use std::sync::Arc;

use crate::array::{Array, ArrayBuilder};
use crate::scalar::PropertyMapValueRef;
use crate::scalar::prop_map::PropertyMapValue;

#[derive(Clone, Debug)]
pub struct PropertyMapArray {
    data: Arc<[PropertyMapValue]>,
}

impl Array for PropertyMapArray {
    type Builder = PropertyMapArrayBuilder;
    type OwnedItem = PropertyMapValue;
    type RefItem<'a> = PropertyMapValueRef<'a>;

    fn get(&self, _idx: usize) -> Option<Self::RefItem<'_>> {
        todo!()
    }

    unsafe fn get_unchecked(&self, _idx: usize) -> Self::RefItem<'_> {
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

pub struct PropertyMapArrayBuilder {
    buffer: Vec<PropertyMapValue>,
}

impl ArrayBuilder for PropertyMapArrayBuilder {
    type Array = PropertyMapArray;

    fn with_capacity(_capacity: usize) -> Self {
        todo!()
    }

    fn push(&mut self, _value: Option<<Self::Array as Array>::RefItem<'_>>) {
        todo!()
    }

    fn finish(self) -> Self::Array {
        todo!()
    }
}
