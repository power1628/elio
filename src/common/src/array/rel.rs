use crate::array::mask::{Mask, MaskMut};
use crate::array::prop_map::{PropertyMapArray, PropertyMapArrayBuilder};
use crate::array::{
    Array, ArrayBuilder, ArrayIterator, NodeIdArray, NodeIdArrayBuilder, RelIdArray, RelIdArrayBuilder, U16Array,
    U16ArrayBuilder,
};
use crate::data_type::DataType;
use crate::scalar::rel::{RelValue, RelValueRef};

#[derive(Clone, Debug)]
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

    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>) {
        match value {
            None => {
                self.id.push(None);
                self.reltype.push(None);
                self.start.push(None);
                self.end.push(None);
                self.properties.push(None);
                self.valid.push(false);
            }
            Some(value) => {
                self.id.push(Some(value.id));
                self.reltype.push(Some(value.reltype));
                self.start.push(Some(value.start));
                self.end.push(Some(value.end));
                self.properties.push(Some(value.properties));
                self.valid.push(true);
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
