use std::sync::Arc;

use crate::array::buffer::Buffer;
use crate::array::{Array, ArrayBuilder};
use crate::data_type::DataType;
use crate::scalar::rel::{RelValue, RelValueRef};
use crate::store_types::PropertyValue;
use crate::{NodeId, PropertyKeyId, RelationshipId, RelationshipTypeId};

#[derive(Clone, Debug)]
pub struct RelArray {
    id: Buffer<RelationshipId>,
    reltype: Buffer<RelationshipId>,
    start: Buffer<NodeId>,
    end: Buffer<NodeId>,
    properties: Arc<[Box<[(PropertyKeyId, PropertyValue)]>]>,
}

impl Array for RelArray {
    type Builder = RelArrayBuilder;
    type OwnedItem = RelValue;
    type RefItem<'a> = RelValueRef<'a>;

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

    fn data_type(&self) -> DataType {
        DataType::Rel
    }
}

pub struct RelArrayBuilder {
    id: Buffer<RelationshipId>,
    reltype: Buffer<RelationshipTypeId>,
    start: Buffer<NodeId>,
    end: Buffer<NodeId>,
    properties: Vec<Vec<(PropertyKeyId, PropertyValue)>>,
}

impl ArrayBuilder for RelArrayBuilder {
    type Array = RelArray;

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
