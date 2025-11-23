use std::sync::Arc;

use crate::array::buffer::Buffer;
use crate::array::{Array, ArrayBuilder};
use crate::data_type::DataType;
use crate::scalar::node::{NodeValue, NodeValueRef};
use crate::store_types::PropertyValue;
use crate::{LabelId, NodeId, PropertyKeyId};

#[derive(Clone, Debug)]
pub struct NodeArray {
    id: Buffer<NodeId>,
    labels: Arc<[Box<[LabelId]>]>,
    properties: Arc<[Box<[(PropertyKeyId, PropertyValue)]>]>,
    // TODO(pgao): inline hot properties here
}

impl Array for NodeArray {
    type Builder = NodeArrayBuilder;
    type OwnedItem = NodeValue;
    type RefItem<'a> = NodeValueRef<'a>;

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
        DataType::Node
    }
}

pub struct NodeArrayBuilder {
    id: Buffer<NodeId>,
    labels: Vec<Vec<LabelId>>,
    properties: Vec<Vec<(PropertyKeyId, PropertyValue)>>,
}

impl ArrayBuilder for NodeArrayBuilder {
    type Array = NodeArray;

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
