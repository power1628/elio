use std::hash::Hash;
use std::sync::Arc;

use enum_as_inner::EnumAsInner;
use itertools::Itertools;

use crate::array::{Array, ArrayImpl, StructArray};
use crate::data_type::F64;
use crate::store_types::RelDirection;
use crate::{NodeId, RelationshipId};

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, derive_more::Display)]
#[display("Node{{id: {}, labels: [{}], props: {}}}", id, labels.iter().map(|l| l.to_string()).collect::<Vec<_>>().join(", "), props)]
pub struct NodeValue {
    pub id: NodeId,
    // TODO(pgao): Vec<Arc<str>>
    pub labels: Vec<String>,
    // TODO(pgao): PropertyMap
    // NodeValue properties have constraint on property types.
    // not general structure value
    pub props: StructValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            self.labels.iter().map(|l| l.to_string()).collect::<Vec<_>>().join(", "),
            self.props.pretty()
        )
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, derive_more::Display)]
#[display(
    "Rel{{id: {}, rtype: {}, start: {}, end: {}, props: {}}}",
    id,
    reltype,
    start_id,
    end_id,
    props
)]
pub struct RelValue {
    pub id: RelationshipId,
    // Arc<str>
    pub reltype: String,
    pub start_id: NodeId,
    pub end_id: NodeId,
    pub props: StructValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn relative_dir(&self, node: NodeId) -> Option<RelDirection> {
        if node == self.start_id {
            Some(RelDirection::Out)
        } else if node == self.end_id {
            Some(RelDirection::In)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, derive_more::Display)]
#[display("VirtualRel{{id: {}, rtype: {}, start: {}, end: {}}}", id, reltype, start_id, end_id)]
pub struct VirtualRel {
    pub id: RelationshipId,
    pub reltype: String,
    pub start_id: NodeId,
    pub end_id: NodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VirtualRelRef<'a> {
    pub id: RelationshipId,
    pub reltype: &'a str,
    pub start_id: NodeId,
    pub end_id: NodeId,
}

#[derive(Debug, Clone)]
pub struct VirtualPath {
    pub nodes: Arc<ArrayImpl>,
    pub rels: Arc<ArrayImpl>,
}

impl PartialEq for VirtualPath {
    fn eq(&self, other: &Self) -> bool {
        self.node_iter().eq(other.node_iter()) && self.rel_iter().eq(other.rel_iter())
    }
}
impl Eq for VirtualPath {}

impl Hash for VirtualPath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.node_iter().for_each(|node| node.hash(state));
        self.rel_iter().for_each(|rel| rel.hash(state));
    }
}

impl VirtualPath {
    pub fn as_scalar_ref<'a>(&'a self) -> VirtualPathRef<'a> {
        VirtualPathRef {
            nodes: self.nodes.as_ref(),
            node_start: 0,
            node_end: self.nodes.len(),
            rels: self.rels.as_ref(),
            rel_start: 0,
            rel_end: self.rels.len(),
        }
    }

    pub fn node_iter(&self) -> impl ExactSizeIterator<Item = Option<NodeId>> + '_ {
        self.nodes.as_virtual_node().unwrap().iter()
    }

    pub fn rel_iter(&self) -> impl ExactSizeIterator<Item = Option<RelValueRef<'_>>> + '_ {
        self.rels.as_rel().unwrap().iter()
    }
}

// TODO(pgao): pretty vs display
impl std::fmt::Display for VirtualPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert_eq!(self.nodes.len(), self.rels.len() + 1);
        let nodes = self.nodes.as_virtual_node().unwrap();
        let rels = self.rels.as_rel().unwrap();

        // SAFETY
        //  PATH element must not be null
        write!(f, "({})", nodes.get(0).expect("path element must not be null"));
        let len = rels.len();

        let lhs = nodes.get(0).expect("path element must not be null");

        for i in 1..len {
            let rhs = nodes.get(i).expect("path element must not be null");
            let rel = rels.get(i).expect("path rel must not be null");
            let (ldir, rdir) = match rel.relative_dir(lhs) {
                Some(RelDirection::Out) => ("-", "->"),
                Some(RelDirection::In) => ("<-", "-"),
                _ => unreachable!("path element must be connected"),
            };
            write!(f, " {ldir}[{}]{rdir}({})", rel.pretty(), rhs);
        }
        Ok(())
    }
}

fn format_path<'a>(
    f: &mut String,
    mut nodes: impl ExactSizeIterator<Item = Option<NodeValueRef<'a>>>,
    mut rels: impl ExactSizeIterator<Item = Option<RelValueRef<'a>>>,
) -> std::fmt::Result {
    use std::fmt::Write;
    assert_eq!(nodes.len(), rels.len() + 1);

    // SAFETY
    //  PATH element must not be null
    write!(f, "({})", nodes.next().unwrap().unwrap().pretty());
    let len = rels.len();

    let lhs = nodes.next().unwrap().unwrap().id;

    for _ in 1..len {
        let rhs = nodes.next().unwrap().unwrap().id;
        let rel = rels.next().unwrap().unwrap();
        let (ldir, rdir) = match rel.relative_dir(lhs) {
            Some(RelDirection::Out) => ("-", "->"),
            Some(RelDirection::In) => ("<-", "-"),
            _ => unreachable!("path element must be connected"),
        };
        write!(f, " {ldir}[{}]{rdir}({})", rel.pretty(), rhs);
    }
    Ok(())
}

fn format_virtual_path<'a>(
    f: &mut String,
    mut nodes: impl ExactSizeIterator<Item = Option<NodeId>>,
    mut rels: impl ExactSizeIterator<Item = Option<RelValueRef<'a>>>,
) -> std::fmt::Result {
    use std::fmt::Write;
    assert_eq!(nodes.len(), rels.len() + 1);

    // SAFETY
    //  PATH element must not be null
    write!(f, "({})", nodes.next().unwrap().unwrap());
    let len = rels.len();

    // TODO(pgao): unsafe iterator
    let lhs = nodes.next().unwrap().unwrap();

    for _ in 1..len {
        let rhs = nodes.next().unwrap().unwrap();
        let rel = rels.next().unwrap().unwrap();
        let (ldir, rdir) = match rel.relative_dir(lhs) {
            Some(RelDirection::Out) => ("-", "->"),
            Some(RelDirection::In) => ("<-", "-"),
            _ => unreachable!("path element must be connected"),
        };
        write!(f, " {ldir}[{}]{rdir}({})", rel.pretty(), rhs);
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct VirtualPathRef<'a> {
    pub nodes: &'a ArrayImpl, // virtual node
    pub node_start: usize,
    pub node_end: usize,
    pub rels: &'a ArrayImpl, // rel
    pub rel_start: usize,
    pub rel_end: usize,
}

impl<'a> std::hash::Hash for VirtualPathRef<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let node_iter = self.node_iter();
        let rel_iter = self.rel_iter();
        for node in node_iter {
            node.hash(state);
        }
        for rel in rel_iter {
            rel.hash(state);
        }
    }
}

impl<'a> ScalarRefVTable for VirtualPathRef<'a> {
    type Owned = VirtualPath;

    fn to_owned_value(&self) -> Self::Owned {
        VirtualPath {
            nodes: Arc::new(
                self.nodes
                    .as_virtual_node()
                    .unwrap()
                    .slice(self.node_start, self.node_end)
                    .into(),
            ),
            rels: Arc::new(self.rels.as_rel().unwrap().slice(self.rel_start, self.rel_end).into()),
        }
    }
}

impl<'a> VirtualPathRef<'a> {
    pub fn node_list_ref(&'a self) -> ListValueRef<'a> {
        ListValueRef::Index {
            child: self.nodes,
            start: self.node_start,
            end: self.node_end,
        }
    }

    pub fn rel_list_ref(&'a self) -> ListValueRef<'a> {
        ListValueRef::Index {
            child: self.rels,
            start: self.rel_start,
            end: self.rel_end,
        }
    }

    pub fn node_iter(&'a self) -> impl ExactSizeIterator<Item = Option<NodeId>> {
        (self.node_start..self.node_end).map(|idx| self.nodes.as_virtual_node().unwrap().get(idx))
    }

    pub fn rel_iter(&'a self) -> impl ExactSizeIterator<Item = Option<RelValueRef<'a>>> {
        (self.rel_start..self.rel_end).map(|idx| self.rels.as_rel().unwrap().get(idx))
    }

    pub fn pretty(&self) -> String {
        let niter = self.node_iter();
        let riter = self.rel_iter();

        let mut f = String::new();

        format_virtual_path(&mut f, niter, riter).unwrap();
        f
    }
}

impl<'a> PartialEq for VirtualPathRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.node_iter().eq(other.node_iter()) && self.rel_iter().eq(other.rel_iter())
    }
}

impl<'a> Eq for VirtualPathRef<'a> {}

#[derive(Debug, Clone)]
pub struct PathValue {
    pub nodes: Arc<ArrayImpl>, // node array
    pub rels: Arc<ArrayImpl>,
}

impl PartialEq for PathValue {
    fn eq(&self, other: &Self) -> bool {
        self.node_iter().eq(other.node_iter()) && self.rel_iter().eq(other.rel_iter())
    }
}

impl Eq for PathValue {}

impl Hash for PathValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.node_iter().for_each(|node| node.hash(state));
        self.rel_iter().for_each(|rel| rel.hash(state));
    }
}

impl PathValue {
    pub fn as_scalar_ref<'a>(&'a self) -> PathValueRef<'a> {
        PathValueRef {
            nodes: &self.nodes,
            node_start: 0,
            node_end: self.nodes.len(),
            rels: &self.rels,
            rel_start: 0,
            rel_end: self.rels.len(),
        }
    }

    pub fn node_iter(&self) -> impl ExactSizeIterator<Item = Option<NodeValueRef<'_>>> {
        // TODO(pgao):avoid downcast in iter
        (0..self.nodes.len()).map(|idx| self.nodes.as_node().unwrap().get(idx))
    }

    pub fn rel_iter(&self) -> impl ExactSizeIterator<Item = Option<RelValueRef<'_>>> {
        (0..self.rels.len()).map(|idx| self.rels.as_rel().unwrap().get(idx))
    }
}

impl std::fmt::Display for PathValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert_eq!(self.nodes.len(), self.rels.len() + 1);
        let nodes = self.nodes.as_node().unwrap();
        let rels = self.rels.as_rel().unwrap();

        // SAFETY
        //  PATH element must not be null
        write!(f, "({})", nodes.get(0).expect("path element must not be null").pretty());
        let len = rels.len();

        let lhs = nodes.get(0).expect("path element must not be null");

        for i in 1..len {
            let rhs = nodes.get(i).expect("path element must not be null");
            let rel = rels.get(i).expect("path rel must not be null");
            let (ldir, rdir) = match rel.relative_dir(lhs.id) {
                Some(RelDirection::Out) => ("-", "->"),
                Some(RelDirection::In) => ("<-", "-"),
                _ => unreachable!("path element must be connected"),
            };
            write!(f, " {ldir}[{}]{rdir}({})", rel.pretty(), rhs.pretty());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PathValueRef<'a> {
    pub nodes: &'a ArrayImpl, // node array
    pub node_start: usize,
    pub node_end: usize,
    pub rels: &'a ArrayImpl, // rel array
    pub rel_start: usize,
    pub rel_end: usize,
}

impl<'a> std::hash::Hash for PathValueRef<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let node = self.node_iter();
        let rel = self.rel_iter();
        for node in node {
            node.hash(state);
        }
        for rel in rel {
            rel.hash(state);
        }
    }
}

impl<'a> ScalarRefVTable for PathValueRef<'a> {
    type Owned = PathValue;

    fn to_owned_value(&self) -> Self::Owned {
        PathValue {
            nodes: Arc::new(
                self.nodes
                    .as_node()
                    .unwrap()
                    .slice(self.node_start, self.node_end)
                    .into(),
            ),
            rels: Arc::new(self.rels.as_rel().unwrap().slice(self.rel_start, self.rel_end).into()),
        }
    }
}

impl<'a> PathValueRef<'a> {
    pub fn node_list_ref(&'a self) -> ListValueRef<'a> {
        ListValueRef::Index {
            child: self.nodes,
            start: self.node_start,
            end: self.node_end,
        }
    }

    pub fn rel_list_ref(&'a self) -> ListValueRef<'a> {
        ListValueRef::Index {
            child: self.rels,
            start: self.rel_start,
            end: self.rel_end,
        }
    }

    pub fn node_iter(&'a self) -> impl ExactSizeIterator<Item = Option<NodeValueRef<'a>>> {
        // TODO(pgao): avoid downcast in iter
        (self.node_start..self.node_end).map(|idx| self.nodes.as_node().unwrap().get(idx))
    }

    pub fn rel_iter(&'a self) -> impl ExactSizeIterator<Item = Option<RelValueRef<'a>>> {
        // TODO(pgao): avoid downcast in iter
        (self.rel_start..self.rel_end).map(|idx| self.rels.as_rel().unwrap().get(idx))
    }

    pub fn pretty(&self) -> String {
        let niter = self.node_iter();
        let riter = self.rel_iter();

        let mut f = String::new();

        format_path(&mut f, niter, riter).unwrap();
        f
    }
}

impl<'a> PartialEq for PathValueRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.node_iter().eq(other.node_iter()) && self.rel_iter().eq(other.rel_iter())
    }
}

impl<'a> Eq for PathValueRef<'a> {}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, derive_more::Display)]
#[display("[{}]", values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "))]
pub struct ListValue {
    values: Vec<ScalarValue>,
}

impl ListValue {
    pub fn new(values: Vec<ScalarValue>) -> Self {
        Self { values }
    }
}

#[derive(Debug, Clone, Copy)]
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

    pub fn len(&self) -> usize {
        match self {
            ListValueRef::Index { child: _, start, end } => end - start,
            ListValueRef::Slice(scalar_values) => scalar_values.len(),
        }
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

impl<'a> PartialEq for ListValueRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<'a> Eq for ListValueRef<'a> {}

impl<'a> std::hash::Hash for ListValueRef<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for item in self.iter() {
            item.hash(state);
        }
    }
}

pub struct ListValueIter<'a> {
    list: ListValueRef<'a>,
    idx: usize,
}

impl<'a> Iterator for ListValueIter<'a> {
    // TODO(pgao): maybe we should use option here?
    type Item = ScalarRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list {
            ListValueRef::Index { child, start, end } => {
                if self.idx >= end - start {
                    None
                } else {
                    let item = child.get(start + self.idx).unwrap_or(ScalarRef::Null);
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

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, derive_more::Display)]
#[display("{{{}}}", fields.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<_>>().join(", "))]
pub struct StructValue {
    // fields must be unique and ordered
    fields: Vec<(Arc<str>, ScalarValue)>,
}

impl StructValue {
    pub fn new(fields: Vec<(Arc<str>, ScalarValue)>) -> Self {
        Self { fields }
    }

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

#[derive(Debug, Clone, Copy)]
pub enum StructValueRef<'a> {
    Index { array: &'a StructArray, idx: usize },
    Value { value: &'a StructValue },
}

impl<'a> PartialEq for StructValueRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<'a> Eq for StructValueRef<'a> {}

impl<'a> std::hash::Hash for StructValueRef<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.iter().for_each(|(k, v)| {
            k.hash(state);
            v.hash(state);
        })
    }
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

    pub fn field_at(&self, name: &str) -> Option<ScalarRef<'a>> {
        match self {
            StructValueRef::Index { array, idx } => array.field_at(name).and_then(|arr| arr.get(*idx)),
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
                    .map(|(name, array)| (name, array.get(idx).unwrap_or(ScalarRef::Null))),
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

#[derive(derive_more::Display, Debug, Clone, Default, EnumAsInner, Eq, PartialEq, Hash)]
pub enum ScalarValue {
    // this is the place holder for null values
    #[default]
    #[display("null")]
    Unknown,
    // primitives
    Bool(bool),
    Integer(i64),
    Float(F64),
    #[display("'{}'", _0)]
    String(String),
    // graph
    #[display("VirtualNode{{_0}}")]
    VirtualNode(NodeId),
    VirtualRel(VirtualRel),
    VirtualPath(VirtualPath),
    Node(Box<NodeValue>),
    Rel(Box<RelValue>),
    Path(Box<PathValue>),
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
            ScalarValue::VirtualPath(vpath) => ScalarRef::VirtualPath(vpath.as_scalar_ref()),
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
            ScalarValue::Path(path) => ScalarRef::Path(path.as_scalar_ref()),
            ScalarValue::List(list) => ScalarRef::List(ListValueRef::Slice(&list.values)),
            ScalarValue::Struct(struct_) => ScalarRef::Struct(StructValueRef::Value { value: struct_ }),
        }
    }
}

#[derive(Debug, EnumAsInner, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarRef<'a> {
    Null,
    Bool(bool),
    Integer(i64),
    Float(F64),
    String(&'a str),
    // graph
    VirtualNode(NodeId),
    VirtualRel(VirtualRelRef<'a>),
    VirtualPath(VirtualPathRef<'a>),
    Node(NodeValueRef<'a>),
    Rel(RelValueRef<'a>),
    Path(PathValueRef<'a>),
    //
    List(ListValueRef<'a>),
    Struct(StructValueRef<'a>),
}

// TODO(pgao): ScalarVTable to scalar_ref

pub trait ScalarRefVTable {
    type Owned;
    fn to_owned_value(&self) -> Self::Owned;
}

impl<'a> ScalarRef<'a> {
    pub fn get_node_id(&self) -> Option<NodeId> {
        match self {
            ScalarRef::VirtualNode(node_id) => Some(*node_id),
            ScalarRef::Node(node_value_ref) => Some(node_value_ref.id),
            _ => None,
        }
    }

    pub fn pretty(&self) -> String {
        match self {
            ScalarRef::Null => "null".to_string(),
            ScalarRef::Bool(b) => b.to_string(),
            ScalarRef::Integer(i) => i.to_string(),
            ScalarRef::Float(ordered_float) => ordered_float.to_string(),
            ScalarRef::String(s) => s.to_string(),
            ScalarRef::VirtualNode(node_id) => format!("virtualnode({})", node_id),
            ScalarRef::VirtualRel(virtual_rel_ref) => format!("virtualrel({})", virtual_rel_ref.id),
            ScalarRef::VirtualPath(vpath) => vpath.pretty().to_string(),
            ScalarRef::Node(node_value_ref) => node_value_ref.pretty(),
            ScalarRef::Rel(rel_value_ref) => rel_value_ref.pretty(),
            ScalarRef::Path(path_value_ref) => path_value_ref.pretty(),
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
            ScalarRef::VirtualPath(vpath) => ScalarValue::VirtualPath(vpath.to_owned_value()),
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
            ScalarRef::Path(path) => ScalarValue::Path(Box::new(path.to_owned_value())),
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
    {bool, Bool},
    {i64, Integer},
    {F64, Float},
    {&'a str, String},
    {NodeId, VirtualNode},
    {VirtualRelRef<'a>, VirtualRel},
    {NodeValueRef<'a>, Node},
    {VirtualPathRef<'a>, VirtualPath},
    {RelValueRef<'a>, Rel},
    {ListValueRef<'a>, List},
    {PathValueRef<'a>, Path},
    {StructValueRef<'a>, Struct}
);

pub type Row = Vec<Option<ScalarValue>>;
