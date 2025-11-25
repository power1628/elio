//! AnyMap array contains AnyMap type scalar
//! Only used to represent properties.
//! Since AnyMap type contains heterogeneous key-value pairs,
//! we consider AnyMapArray as an leaf array.

use std::sync::Arc;

use crate::array::mask::{Mask, MaskMut};
use crate::array::{Array, ArrayBuilder, ArrayIterator};
use crate::data_type::DataType;
use crate::scalar::prop_map::PropertyMapValue;
use crate::scalar::{PropertyMapValueRef, Scalar, ScalarRef};

#[derive(Clone, Debug)]
pub struct PropertyMapArray {
    data: Arc<[PropertyMapValue]>,
    valid: Mask,
}

impl Array for PropertyMapArray {
    type Builder = PropertyMapArrayBuilder;
    type OwnedItem = PropertyMapValue;
    type RefItem<'a> = PropertyMapValueRef<'a>;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        self.valid.get(idx).then(|| self.data[idx].as_scalar_ref())
    }

    unsafe fn get_unchecked(&self, idx: usize) -> Self::RefItem<'_> {
        unsafe { self.data.get_unchecked(idx).as_scalar_ref() }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn iter(&self) -> super::ArrayIterator<'_, Self> {
        ArrayIterator::new(self)
    }

    fn data_type(&self) -> crate::data_type::DataType {
        crate::data_type::DataType::PropertyMap
    }
}

pub struct PropertyMapArrayBuilder {
    buffer: Vec<PropertyMapValue>,
    valid: MaskMut,
}

impl ArrayBuilder for PropertyMapArrayBuilder {
    type Array = PropertyMapArray;

    fn with_capacity(capacity: usize, typ: DataType) -> Self {
        assert_eq!(typ, DataType::PropertyMap);
        Self {
            buffer: Vec::with_capacity(capacity),
            valid: MaskMut::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>) {
        match value {
            Some(v) => {
                self.buffer.push(v.to_owned_scalar());
                self.valid.push(true);
            }
            None => {
                self.buffer.push(PropertyMapValue::default());
                self.valid.push(false);
            }
        }
    }

    fn finish(self) -> Self::Array {
        Self::Array {
            data: self.buffer.into(),
            valid: self.valid.freeze(),
        }
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}
