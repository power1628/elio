use crate::array::list::{ListArray, ListArrayBuilder};
use crate::array::mask::{Mask, MaskMut};
use crate::array::prop_map::{PropertyMapArray, PropertyMapArrayBuilder};
use crate::array::{Array, ArrayBuilder, NodeIdArray, NodeIdArrayBuilder};
use crate::data_type::DataType;
use crate::scalar::node::{NodeValue, NodeValueRef};

#[derive(Clone, Debug)]
pub struct NodeArray {
    id: NodeIdArray,
    labels: ListArray,
    properties: PropertyMapArray,
    // TODO(pgao): inline hot properties here
    valid: Mask,
}

impl Array for NodeArray {
    type Builder = NodeArrayBuilder;
    type OwnedItem = NodeValue;
    type RefItem<'a> = NodeValueRef<'a>;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        self.valid.get(idx).then(|| {
            let id = self.id.get(idx).unwrap();
            // SAFETY:
            // labels and properties never be null
            let labels = self.labels.get(idx).unwrap();
            let properties = self.properties.get(idx).unwrap();
            NodeValueRef::new(id, labels, properties)
        })
    }

    unsafe fn get_unchecked(&self, idx: usize) -> Self::RefItem<'_> {
        unsafe {
            let id = self.id.get_unchecked(idx);
            let labels = self.labels.get_unchecked(idx);
            let properties = self.properties.get_unchecked(idx);
            NodeValueRef::new(id, labels, properties)
        }
    }

    fn len(&self) -> usize {
        self.id.len()
    }

    fn iter(&self) -> super::ArrayIterator<'_, Self> {
        super::ArrayIterator::new(self)
    }

    fn data_type(&self) -> DataType {
        DataType::Node
    }
}

pub struct NodeArrayBuilder {
    id: NodeIdArrayBuilder,
    labels: ListArrayBuilder,
    properties: PropertyMapArrayBuilder,
    valid: MaskMut,
}

impl ArrayBuilder for NodeArrayBuilder {
    type Array = NodeArray;

    fn with_capacity(capacity: usize, typ: DataType) -> Self {
        assert_eq!(typ, DataType::Node);
        Self {
            id: NodeIdArrayBuilder::with_capacity(capacity, DataType::NodeId),
            labels: ListArrayBuilder::with_capacity(capacity, DataType::List(Box::new(DataType::U16))),
            properties: PropertyMapArrayBuilder::with_capacity(capacity, DataType::PropertyMap),
            valid: MaskMut::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Option<NodeValueRef<'_>>) {
        match value {
            Some(NodeValueRef { id, labels, properties }) => {
                self.valid.push(true);
                self.id.push(Some(id));
                self.labels.push(Some(labels));
                self.properties.push(Some(properties));
            }
            None => {
                self.valid.push(false);
                self.id.push(None);
                self.labels.push(None);
                self.properties.push(None);
            }
        }
    }

    fn finish(self) -> Self::Array {
        Self::Array {
            id: self.id.finish(),
            labels: self.labels.finish(),
            properties: self.properties.finish(),
            valid: self.valid.freeze(),
        }
    }

    fn len(&self) -> usize {
        self.id.len()
    }
}
