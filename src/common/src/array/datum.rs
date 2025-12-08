use std::sync::Arc;

use enum_as_inner::EnumAsInner;

use crate::array::{ArrayImpl, StructArray};
use crate::data_type::F64;
use crate::{NodeId, RelationshipId};

#[derive(Debug, Clone, Default)]
pub struct NodeValue {
    pub id: NodeId,
    pub labels: Vec<String>,
    pub props: StructValue,
}

pub struct NodeValueRef<'a> {
    pub id: NodeId,
    pub labels: &'a [String],
    pub props: &'a StructValue,
}

#[derive(Debug, Clone, Default)]
pub struct RelValue {
    pub id: RelationshipId,
    pub reltype: String,
    pub start_id: NodeId,
    pub end_id: NodeId,
    pub props: StructValue,
}

pub struct RelValueRef<'a> {
    pub id: RelationshipId,
    pub reltype: &'a str,
    pub start_id: NodeId,
    pub end_id: NodeId,
    pub props: &'a StructValue,
}

#[derive(Debug, Clone, Default)]
pub struct VirtualRel {
    pub id: RelationshipId,
    pub reltype: String,
    pub start_id: NodeId,
    pub end_id: NodeId,
}

pub struct VirtualRelRef<'a> {
    pub id: RelationshipId,
    pub reltype: &'a str,
    pub start_id: NodeId,
    pub end_id: NodeId,
}

#[derive(Debug, Clone, Default)]
pub struct ListValue {
    values: Vec<ScalarValue>,
}

pub enum ListValueRef<'a> {
    Index {
        child: &'a ArrayImpl,
        start: usize,
        end: usize,
    },
    Slice(&'a [ScalarValue]),
}

#[derive(Debug, Clone, Default)]
pub struct StructValue {
    // fields must be unique and ordered
    fields: Vec<(Arc<str>, ScalarValue)>,
}

impl StructValue {
    pub fn iter(&self) -> impl Iterator<Item = (&Arc<str>, &ScalarValue)> {
        self.fields.iter().map(|(k, v)| (k, v))
    }

    pub fn field_at(&self, name: &str) -> Option<ScalarRef<'_>> {
        self.fields.iter().find(|(k, _)| **k == *name).map(|(_, v)| v.into())
    }
}

pub enum StructValueRef<'a> {
    Index { array: &'a StructArray, idx: usize },
    Value { value: &'a StructValue },
}

impl<'a> StructValueRef<'a> {
    pub fn field_at(&self, name: &str) -> Option<ScalarRef<'_>> {
        match self {
            StructValueRef::Index { array, idx } => array.field_at(name).unwrap().get(*idx),
            StructValueRef::Value { value } => value.field_at(name),
        }
    }
}

#[derive(Debug, Clone, Default, EnumAsInner)]
pub enum ScalarValue {
    // this is the place holder for null values
    #[default]
    Unknown,
    // primitives
    Integer(i64),
    Float(F64),
    String(String),
    // graph
    VirtualNode(NodeId),
    VirtualRel(VirtualRel),
    Node(Box<NodeValue>),
    Rel(Box<RelValue>),
    // nested
    List(Box<ListValue>),
    Struct(Box<StructValue>),
}

pub enum ScalarRef<'a> {
    Null,
    Integer(i64),
    Float(F64),
    String(&'a str),
    // graph
    VirtualNode(NodeId),
    VirtualRel(VirtualRelRef<'a>),
    Node(NodeValueRef<'a>),
    Rel(RelValueRef<'a>),
    //
    List(ListValueRef<'a>),
    Struct(StructValueRef<'a>),
}

impl<'a> From<&'a ScalarValue> for ScalarRef<'a> {
    fn from(value: &'a ScalarValue) -> Self {
        match value {
            ScalarValue::Unknown => ScalarRef::Null,
            ScalarValue::Integer(i) => ScalarRef::Integer(*i),
            ScalarValue::Float(f) => ScalarRef::Float(*f),
            ScalarValue::String(s) => ScalarRef::String(s),
            ScalarValue::VirtualNode(id) => ScalarRef::VirtualNode(*id),
            ScalarValue::VirtualRel(vrel) => ScalarRef::VirtualRel(VirtualRelRef {
                id: vrel.id,
                reltype: &vrel.reltype,
                start_id: vrel.start_id,
                end_id: vrel.end_id,
            }),
            ScalarValue::Node(node) => ScalarRef::Node(NodeValueRef {
                id: node.id,
                labels: &node.labels,
                props: &node.props,
            }),
            ScalarValue::Rel(rel) => ScalarRef::Rel(RelValueRef {
                id: rel.id,
                reltype: &rel.reltype,
                start_id: rel.start_id,
                end_id: rel.end_id,
                props: &rel.props,
            }),
            ScalarValue::List(list) => ScalarRef::List(ListValueRef::Slice(&list.values)),
            ScalarValue::Struct(struct_) => ScalarRef::Struct(StructValueRef::Value { value: struct_ }),
        }
    }
}

macro_rules! impl_into_for_scalar_ref {
    ($({$AbcRef:ty, $Abc:ident}),*) => {
        $(impl<'a> From<$AbcRef> for ScalarRef<'a> {
            fn from(value: $AbcRef) -> ScalarRef<'a> {
                ScalarRef::$Abc(value)
            }
        })*
    };

    ($({&'a $AbcRef:ty, $Abc:ident}),*) => {
        $(impl<'a> From<&'a $AbcRef> for ScalarRef<'a> {
            fn from(value: &'a $AbcRef) -> ScalarRef<'a> {
                ScalarRef::$Abc(value)
            }
        })*
    };

}

impl_into_for_scalar_ref!(
    {i64, Integer},
    {F64, Float},
    {&'a str, String},
    {NodeId, VirtualNode},
    {VirtualRelRef<'a>, VirtualRel},
    {NodeValueRef<'a>, Node},
    {RelValueRef<'a>, Rel},
    {ListValueRef<'a>, List},
    {StructValueRef<'a>, Struct}
);
