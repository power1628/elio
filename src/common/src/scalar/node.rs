use crate::array::node::NodeArray;
use crate::scalar::{Scalar, ScalarRef};
use crate::store_types::PropertyValue;
use crate::{LabelId, NodeId, PropertyKeyId};

#[derive(Clone, Debug)]
pub struct NodeValue {
    pub id: NodeId,
    pub labels: Vec<LabelId>,
    pub properties: Vec<(PropertyKeyId, PropertyValue)>,
}

impl Scalar for NodeValue {
    type ArrayType = NodeArray;
    type RefType<'a> = NodeValueRef<'a>;

    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NodeValueRef<'scalar> {
    pub id: &'scalar NodeId,
    pub labels: &'scalar [LabelId],
    pub properties: &'scalar [(PropertyKeyId, PropertyValue)],
}

impl<'a> ScalarRef<'a> for NodeValueRef<'a> {
    type ArrayType = NodeArray;
    type ScalarType = NodeValue;

    fn to_owned_scalar(&self) -> Self::ScalarType {
        todo!()
    }
}
