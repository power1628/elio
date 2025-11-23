use crate::array::rel::RelArray;
use crate::scalar::{Scalar, ScalarRef};
use crate::store_types::PropertyValue;
use crate::{NodeId, PropertyKeyId, RelationshipId, RelationshipTypeId};

#[derive(Clone, Debug)]
pub struct RelValue {
    pub id: RelationshipId,
    pub reltype: RelationshipTypeId,
    pub start: NodeId,
    pub end: NodeId,
    pub properties: Vec<(PropertyKeyId, PropertyValue)>,
}

impl Scalar for RelValue {
    type ArrayType = RelArray;
    type RefType<'a> = RelValueRef<'a>;

    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RelValueRef<'scalar> {
    pub id: &'scalar RelationshipId,
    pub reltype: &'scalar RelationshipTypeId,
    pub start: &'scalar NodeId,
    pub end: &'scalar NodeId,
    pub properties: &'scalar [(PropertyKeyId, PropertyValue)],
}

impl<'a> ScalarRef<'a> for RelValueRef<'a> {
    type ArrayType = RelArray;
    type ScalarType = RelValue;

    fn to_owned_scalar(&self) -> Self::ScalarType {
        todo!()
    }
}
