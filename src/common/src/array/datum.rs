use std::sync::Arc;

use enum_as_inner::EnumAsInner;
use itertools::Itertools;

use crate::array::{Array, ArrayImpl, StructArray};
use crate::data_type::F64;
use crate::{NodeId, RelationshipId};

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct NodeValue {
    pub id: NodeId,
    pub labels: Vec<String>,
    pub props: StructValue,
}

pub struct NodeValueRef<'a> {
    pub id: NodeId,
    pub labels: &'a [String],
    pub props: StructValueRef<'a>,
}

impl<'a> NodeValueRef<'a> {
    pub fn pretty(&self) -> String {
        format!(
            "node{{id: {}, labels: [{}], props: {}}}",
            self.id,
            self.labels
                .iter()
                .map(|l| l.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.props.pretty()
        )
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
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
    pub props: StructValueRef<'a>,
}
impl<'a> RelValueRef<'a> {
    pub fn pretty(&self) -> String {
        format!(
            "rel{{id: {}, rtype: {}, start_id: {}, end_id: {}, props: {}}}",
            self.id,
            self.reltype,
            self.start_id,
            self.end_id,
            self.props.pretty()
        )
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
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

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct ListValue {
    values: Vec<ScalarValue>,
}

#[derive(Clone, Copy)]
pub enum ListValueRef<'a> {
    Index {
        child: &'a ArrayImpl,
        start: usize,
        end: usize,
    },
    Slice(&'a [ScalarValue]),
}

impl<'a> ListValueRef<'a> {
    pub fn pretty(&self) -> String {
        format!(
            "[{}]",
            self.iter().map(|item| item.pretty()).collect::<Vec<_>>().join(", ")
        )
    }

    pub fn iter(&self) -> ListValueIter<'a> {
        ListValueIter { list: *self, idx: 0 }
    }

    pub fn as_integer_list(&self) -> Option<Vec<i64>> {
        let mut vec = vec![];
        for item in self.iter() {
            if let ScalarRef::Integer(b) = item {
                vec.push(b);
            } else {
                return None;
            }
        }
        Some(vec)
    }

    pub fn as_float_list(&self) -> Option<Vec<F64>> {
        let mut vec = vec![];
        for item in self.iter() {
            if let ScalarRef::Float(b) = item {
                vec.push(b);
            } else {
                return None;
            }
        }
        Some(vec)
    }

    pub fn as_string_list(&self) -> Option<Vec<String>> {
        let mut vec = vec![];
        for item in self.iter() {
            if let ScalarRef::String(b) = item {
                vec.push(b.to_owned());
            } else {
                return None;
            }
        }
        Some(vec)
    }
}

pub struct ListValueIter<'a> {
    list: ListValueRef<'a>,
    idx: usize,
}

impl<'a> Iterator for ListValueIter<'a> {
    type Item = ScalarRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list {
            ListValueRef::Index { child, start, end } => {
                if self.idx >= end - start {
                    None
                } else {
                    let item = child.get(start + self.idx).unwrap();
                    self.idx += 1;
                    Some(item)
                }
            }
            ListValueRef::Slice(scalar_values) => {
                if self.idx >= scalar_values.len() {
                    None
                } else {
                    let item = &scalar_values[self.idx];
                    self.idx += 1;
                    Some(item.as_scalar_ref())
                }
            }
        }
    }
}

impl<'a> ScalarRefVTable for ListValueRef<'a> {
    type Owned = ListValue;

    fn to_owned_value(&self) -> Self::Owned {
        ListValue {
            values: self.iter().map(|x| x.to_owned_value()).collect_vec(),
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct StructValue {
    // fields must be unique and ordered
    fields: Vec<(Arc<str>, ScalarValue)>,
}

impl StructValue {
    pub fn iter(&self) -> impl Iterator<Item = (&Arc<str>, &ScalarValue)> {
        self.fields.iter().map(|(k, v)| (k, v))
    }

    pub fn field_at_pos(&self, idx: usize) -> Option<&(Arc<str>, ScalarValue)> {
        self.fields.get(idx)
    }

    pub fn field_at(&self, name: &str) -> Option<ScalarRef<'_>> {
        self.fields
            .iter()
            .find(|(k, _)| **k == *name)
            .map(|(_, v)| v.as_scalar_ref())
    }

    pub fn as_scalar_ref(&self) -> StructValueRef<'_> {
        StructValueRef::Value { value: self }
    }
}

#[derive(Clone, Copy)]
pub enum StructValueRef<'a> {
    Index { array: &'a StructArray, idx: usize },
    Value { value: &'a StructValue },
}

impl<'a> StructValueRef<'a> {
    pub fn pretty(&self) -> String {
        format!(
            "struct{{{}}}",
            self.iter()
                .map(|(k, v)| format!("{}: {}", k, v.pretty()))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    pub fn field_at(&self, name: &str) -> Option<ScalarRef<'_>> {
        match self {
            StructValueRef::Index { array, idx } => array.field_at(name).unwrap().get(*idx),
            StructValueRef::Value { value } => value.field_at(name),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            StructValueRef::Index { array, .. } => array.fields().len(),
            StructValueRef::Value { value } => value.fields.len(),
        }
    }

    pub fn iter(&self) -> StructValueRefIter<'a> {
        StructValueRefIter {
            struct_ref: *self,
            pos: 0,
            len: self.len(),
        }
    }
}

impl<'a> ScalarRefVTable for StructValueRef<'a> {
    type Owned = StructValue;

    fn to_owned_value(&self) -> Self::Owned {
        match self {
            StructValueRef::Index { array, idx } => {
                let mut fields = vec![];
                for (name, array) in array.fields() {
                    let value = array.get(*idx).unwrap().to_owned_value();
                    fields.push((name.clone(), value));
                }
                StructValue { fields }
            }
            StructValueRef::Value { value } => (*value).clone(),
        }
    }
}

pub struct StructValueRefIter<'a> {
    struct_ref: StructValueRef<'a>,
    pos: usize,
    len: usize,
}

impl<'a> Iterator for StructValueRefIter<'a> {
    type Item = (&'a Arc<str>, ScalarRef<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.len {
            let item = match self.struct_ref {
                StructValueRef::Index { array, idx } => array
                    .field_at_pos(self.pos)
                    .map(|(name, array)| (name, array.get(idx).unwrap())),
                StructValueRef::Value { value } => value
                    .field_at_pos(self.pos)
                    .map(|(name, value)| (name, value.as_scalar_ref())),
            };
            self.pos += 1;
            item
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for StructValueRefIter<'a> {
    fn len(&self) -> usize {
        self.len
    }
}

#[derive(Debug, Hash, Clone, Default, EnumAsInner, Eq, PartialEq)]
pub enum ScalarValue {
    // this is the place holder for null values
    #[default]
    Unknown,
    // primitives
    Bool(bool),
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

impl ScalarValue {
    pub fn as_scalar_ref(&self) -> ScalarRef<'_> {
        match self {
            ScalarValue::Unknown => ScalarRef::Null,
            ScalarValue::Bool(b) => ScalarRef::Bool(*b),
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
                props: node.props.as_scalar_ref(),
            }),
            ScalarValue::Rel(rel) => ScalarRef::Rel(RelValueRef {
                id: rel.id,
                reltype: &rel.reltype,
                start_id: rel.start_id,
                end_id: rel.end_id,
                props: rel.props.as_scalar_ref(),
            }),
            ScalarValue::List(list) => ScalarRef::List(ListValueRef::Slice(&list.values)),
            ScalarValue::Struct(struct_) => ScalarRef::Struct(StructValueRef::Value { value: struct_ }),
        }
    }
}

pub enum ScalarRef<'a> {
    Null,
    Bool(bool),
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

pub trait ScalarRefVTable {
    type Owned;
    fn to_owned_value(&self) -> Self::Owned;
}

impl<'a> ScalarRef<'a> {
    pub fn pretty(&self) -> String {
        match self {
            ScalarRef::Null => "null".to_string(),
            ScalarRef::Bool(b) => b.to_string(),
            ScalarRef::Integer(i) => i.to_string(),
            ScalarRef::Float(ordered_float) => ordered_float.to_string(),
            ScalarRef::String(s) => s.to_string(),
            ScalarRef::VirtualNode(node_id) => format!("virtualnode({})", node_id),
            ScalarRef::VirtualRel(virtual_rel_ref) => format!("virtualrel({})", virtual_rel_ref.id),
            ScalarRef::Node(node_value_ref) => node_value_ref.pretty(),
            ScalarRef::Rel(rel_value_ref) => rel_value_ref.pretty(),
            ScalarRef::List(list_value_ref) => list_value_ref.pretty(),
            ScalarRef::Struct(struct_value_ref) => struct_value_ref.pretty(),
        }
    }

    pub fn to_owned_value(&self) -> ScalarValue {
        match self {
            ScalarRef::Null => ScalarValue::Unknown,
            ScalarRef::Bool(b) => ScalarValue::Bool(*b),
            ScalarRef::Integer(i) => ScalarValue::Integer(*i),
            ScalarRef::Float(f) => ScalarValue::Float(*f),
            ScalarRef::String(s) => ScalarValue::String(s.to_string()),
            ScalarRef::VirtualNode(node_id) => ScalarValue::VirtualNode(*node_id),
            ScalarRef::VirtualRel(virtual_rel_ref) => ScalarValue::VirtualRel(VirtualRel {
                id: virtual_rel_ref.id,
                reltype: virtual_rel_ref.reltype.to_owned(),
                start_id: virtual_rel_ref.start_id,
                end_id: virtual_rel_ref.end_id,
            }),
            ScalarRef::Node(node_value_ref) => ScalarValue::Node(Box::new(NodeValue {
                id: node_value_ref.id,
                labels: node_value_ref.labels.to_vec(),
                props: node_value_ref.props.to_owned_value(),
            })),
            ScalarRef::Rel(rel_value_ref) => ScalarValue::Rel(Box::new(RelValue {
                id: rel_value_ref.id,
                reltype: rel_value_ref.reltype.to_owned(),
                start_id: rel_value_ref.start_id,
                end_id: rel_value_ref.end_id,
                props: rel_value_ref.props.to_owned_value(),
            })),
            ScalarRef::List(list_value_ref) => ScalarValue::List(Box::new(list_value_ref.to_owned_value())),
            ScalarRef::Struct(struct_value_ref) => ScalarValue::Struct(Box::new(struct_value_ref.to_owned_value())),
        }
    }
}

macro_rules! impl_into_for_scalar_ref {
    // without lifetime
    ($({$AbcRef:ty, $Abc:ident}),*) => {
        $(impl<'a> From<$AbcRef> for ScalarRef<'a> {
            fn from(value: $AbcRef) -> ScalarRef<'a> {
                ScalarRef::$Abc(value)
            }
        })*
    };

    // with lifetime
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

pub type Row = Vec<Option<ScalarValue>>;
