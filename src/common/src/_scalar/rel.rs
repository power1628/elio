use crate::array::rel::RelArray;
use crate::data_type::DataType;
use crate::scalar::{PropertyMapValue, PropertyMapValueRef, Scalar, ScalarRef};
use crate::{NodeId, RelationshipId, RelationshipTypeId};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

    fn data_type(&self) -> DataType {
        DataType::Rel
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

impl<'a> RelValueRef<'a> {
    pub fn pretty(&self) -> String {
        format!(
            "{{id: {}, reltype: {}, start: {}, end: {}, properties: {}}}",
            self.id,
            self.reltype,
            self.start,
            self.end,
            self.properties.pretty()
        )
    }
}
