use crate::NodeId;
use crate::array::buffer::Buffer;
use crate::array::list::{ListArray, ListArrayBuilder};
use crate::array::prop_map::{PropertyMapArray, PropertyMapArrayBuilder};
use crate::array::{Array, ArrayBuilder};
use crate::data_type::DataType;
use crate::scalar::node::{NodeValue, NodeValueRef};

#[derive(Clone, Debug)]
pub struct NodeArray {
    id: Buffer<NodeId>,
    labels: ListArray,
    properties: PropertyMapArray,
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
    labels: ListArrayBuilder,
    properties: PropertyMapArrayBuilder,
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
