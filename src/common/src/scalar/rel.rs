use crate::array::rel::RelArray;
use crate::scalar::{PropertyMapValue, PropertyMapValueRef, Scalar, ScalarRef};
use crate::{NodeId, PropertyKeyId, RelationshipId, RelationshipTypeId};

#[derive(Clone, Debug)]
pub struct RelValue {
    pub id: RelationshipId,
    pub reltype: RelationshipTypeId,
    pub start: NodeId,
    pub end: NodeId,
    pub properties: PropertyMapValue,
}

impl Scalar for RelValue {
    type ArrayType = RelArray;
    type RefType<'a> = RelValueRef<'a>;

    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        RelValueRef {
            id: self.id,
            reltype: self.reltype,
            start: self.start,
            end: self.end,
            properties: self.properties.as_scalar_ref(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RelValueRef<'scalar> {
    pub id: RelationshipId,
    pub reltype: RelationshipTypeId,
    pub start: NodeId,
    pub end: NodeId,
    pub properties: PropertyMapValueRef<'scalar>,
}

impl<'a> ScalarRef<'a> for RelValueRef<'a> {
    type ArrayType = RelArray;
    type ScalarType = RelValue;

    fn to_owned_scalar(&self) -> Self::ScalarType {
        RelValue {
            id: self.id,
            reltype: self.reltype,
            start: self.start,
            end: self.end,
            properties: self.properties.to_owned_scalar(),
        }
    }
}
