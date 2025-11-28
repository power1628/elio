use crate::array::mask::{Mask, MaskMut};
use crate::array::prop_map::{PropertyMapArray, PropertyMapArrayBuilder};
use crate::array::{
    Array, ArrayBuilder, ArrayIterator, NodeIdArray, NodeIdArrayBuilder, RelIdArray, RelIdArrayBuilder, U16Array,
    U16ArrayBuilder,
};
use crate::data_type::DataType;
use crate::scalar::rel::{RelValue, RelValueRef};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RelArray {
    id: RelIdArray,
    reltype: U16Array,
    start: NodeIdArray,
    end: NodeIdArray,
    properties: PropertyMapArray,
    valid: Mask,
}

impl Array for RelArray {
    type Builder = RelArrayBuilder;
    type OwnedItem = RelValue;
    type RefItem<'a> = RelValueRef<'a>;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        self.valid.get(idx).then(|| unsafe { self.get_unchecked(idx) })
    }

    unsafe fn get_unchecked(&self, idx: usize) -> Self::RefItem<'_> {
        unsafe {
            RelValueRef {
                id: self.id.get_unchecked(idx),
                reltype: self.reltype.get_unchecked(idx),
                start: self.start.get_unchecked(idx),
                end: self.end.get_unchecked(idx),
                properties: self.properties.get_unchecked(idx),
            }
        }
    }

    fn len(&self) -> usize {
        self.id.len()
    }

    fn iter(&self) -> super::ArrayIterator<'_, Self> {
        ArrayIterator::new(self)
    }

    fn data_type(&self) -> DataType {
        DataType::Rel
    }
}

#[derive(Debug)]
pub struct RelArrayBuilder {
    id: RelIdArrayBuilder,
    reltype: U16ArrayBuilder,
    start: NodeIdArrayBuilder,
    end: NodeIdArrayBuilder,
    properties: PropertyMapArrayBuilder,
    valid: MaskMut,
}

impl ArrayBuilder for RelArrayBuilder {
    type Array = RelArray;

    fn with_capacity(capacity: usize, typ: DataType) -> Self {
        assert_eq!(typ, DataType::Rel);
        Self {
            id: RelIdArrayBuilder::with_capacity(capacity, DataType::RelId),
            reltype: U16ArrayBuilder::with_capacity(capacity, DataType::U16),
            start: NodeIdArrayBuilder::with_capacity(capacity, DataType::NodeId),
            end: NodeIdArrayBuilder::with_capacity(capacity, DataType::NodeId),
            properties: PropertyMapArrayBuilder::with_capacity(capacity, DataType::PropertyMap),
            valid: MaskMut::with_capacity(capacity),
        }
    }

    fn append_n(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>, repeat: usize) {
        match value {
            None => {
                self.id.append_n(None, repeat);
                self.reltype.append_n(None, repeat);
                self.start.append_n(None, repeat);
                self.end.append_n(None, repeat);
                self.properties.append_n(None, repeat);
                self.valid.append_n(false, repeat);
            }
            Some(value) => {
                self.id.append_n(Some(value.id), repeat);
                self.reltype.append_n(Some(value.reltype), repeat);
                self.start.append_n(Some(value.start), repeat);
                self.end.append_n(Some(value.end), repeat);
                self.properties.append_n(Some(value.properties), repeat);
                self.valid.append_n(true, repeat);
            }
        }
    }

    fn finish(self) -> Self::Array {
        Self::Array {
            id: self.id.finish(),
            reltype: self.reltype.finish(),
            start: self.start.finish(),
            end: self.end.finish(),
            properties: self.properties.finish(),
            valid: self.valid.freeze(),
        }
    }

    fn len(&self) -> usize {
        self.id.len()
    }
}
