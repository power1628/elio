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

#[derive(Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug)]
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

    fn append_n(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>, repeat: usize) {
        match value {
            Some(v) => {
                self.buffer.extend(std::iter::repeat_n(v.to_owned_scalar(), repeat));
                self.valid.append_n(true, repeat);
            }
            None => {
                self.buffer
                    .extend(std::iter::repeat_n(PropertyMapValue::default(), repeat));
                self.valid.append_n(false, repeat);
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

#[cfg(test)]
mod tests {

    use itertools::Itertools;

    use super::*;
    use crate::array::{Array, ArrayBuilder};
    use crate::data_type::DataType;

    // #[test]
    // fn test_prop_map_array_builder_and_get() {
    //     let mut builder = PropertyMapArrayBuilder::with_capacity(4, DataType::PropertyMap);

    //     // Create some property maps
    //     let mut map1 = BTreeMap::new();
    //     map1.insert(1u16, PropertyValue::String("Alice".to_string()));
    //     map1.insert(2u16, PropertyValue::Integer(30));
    //     let prop_map1 = PropertyMapValue(map1.into_iter().collect_vec());
    //     let mut map2 = BTreeMap::new();
    //     map2.insert(3u16, PropertyValue::String("New York".to_string()));
    //     let prop_map2 = PropertyMapValue(map2.into_iter().collect_vec());

    //     let empty_map = PropertyMapValue::default();

    //     builder.push(Some(prop_map1.as_scalar_ref()));
    //     builder.push(None);
    //     builder.push(Some(prop_map2.as_scalar_ref()));
    //     builder.push(Some(empty_map.as_scalar_ref()));

    //     assert_eq!(builder.len(), 4);
    //     let arr = builder.finish();

    //     assert_eq!(arr.len(), 4);
    //     assert_eq!(arr.data_type(), DataType::PropertyMap);

    //     // Test get()
    //     assert_eq!(arr.get(0), Some(prop_map1.as_scalar_ref()));
    //     assert_eq!(arr.get(1), None);
    //     assert_eq!(arr.get(2), Some(prop_map2.as_scalar_ref()));
    //     assert_eq!(arr.get(3), Some(empty_map.as_scalar_ref()));

    //     // Test get_unchecked()
    //     unsafe {
    //         assert_eq!(arr.get_unchecked(0), prop_map1.as_scalar_ref());
    //         assert_eq!(arr.get_unchecked(2), prop_map2.as_scalar_ref());
    //         // For None, it should return the default value which was pushed
    //         assert_eq!(arr.get_unchecked(1), PropertyMapValue::default().as_scalar_ref());
    //     }
    // }

    // #[test]
    // fn test_prop_map_array_iter() {
    //     let mut builder = PropertyMapArrayBuilder::with_capacity(3, DataType::PropertyMap);

    //     let mut map1 = BTreeMap::new();
    //     map1.insert(1u16, PropertyValue::String("value".to_string()));
    //     let prop_map1 = PropertyMapValue(map1.into_iter().collect_vec());

    //     builder.push(Some(prop_map1.as_scalar_ref()));
    //     builder.push(None);

    //     let arr = builder.finish();
    //     let mut iter = arr.iter();

    //     assert_eq!(iter.next(), Some(Some(prop_map1.as_scalar_ref())));
    //     assert_eq!(iter.next(), Some(None));
    //     assert_eq!(iter.next(), None);
    // }

    #[test]
    fn test_empty_prop_map_array() {
        let builder = PropertyMapArrayBuilder::with_capacity(0, DataType::PropertyMap);
        let arr = builder.finish();
        assert!(arr.is_empty());
        assert_eq!(arr.len(), 0);
        assert_eq!(arr.iter().next(), None);
    }

    #[test]
    fn test_all_nulls_prop_map_array() {
        let mut builder = PropertyMapArrayBuilder::with_capacity(3, DataType::PropertyMap);
        builder.append_n(None, 2);
        let arr = builder.finish();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr.get(0), None);
        assert_eq!(arr.get(1), None);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn test_builder_with_wrong_type() {
        let _builder = PropertyMapArrayBuilder::with_capacity(5, DataType::String);
    }
}
