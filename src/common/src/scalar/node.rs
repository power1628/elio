use crate::NodeId;
use crate::array::node::NodeArray;
use crate::scalar::list::{ListValue, ListValueRef};
use crate::scalar::{PropertyMapValue, PropertyMapValueRef, Scalar, ScalarRef};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeValue {
    pub id: NodeId,
    pub labels: ListValue,
    pub properties: PropertyMapValue,
}

impl Scalar for NodeValue {
    type ArrayType = NodeArray;
    type RefType<'a> = NodeValueRef<'a>;

    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        NodeValueRef {
            id: self.id,
            labels: self.labels.as_scalar_ref(),
            properties: self.properties.as_scalar_ref(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeValueRef<'a> {
    pub id: NodeId,
    pub labels: ListValueRef<'a>,
    pub properties: PropertyMapValueRef<'a>,
}

impl<'a> NodeValueRef<'a> {
    pub fn new(id: NodeId, labels: ListValueRef<'a>, properties: PropertyMapValueRef<'a>) -> Self {
        Self { id, labels, properties }
    }

    pub fn pretty(&self) -> String {
        format!(
            "{{id: {}, labels: {}, properties: {}}}",
            self.id,
            self.labels.pretty(),
            self.properties.pretty(),
        )
    }
}

impl<'a> ScalarRef<'a> for NodeValueRef<'a> {
    type ArrayType = NodeArray;
    type ScalarType = NodeValue;

    fn to_owned_scalar(&self) -> Self::ScalarType {
        NodeValue {
            id: self.id,
            labels: self.labels.to_owned_scalar(),
            properties: self.properties.to_owned_scalar(),
        }
    }
}
