use std::sync::Arc;

use bitvec::prelude::*;

//// scalar
pub struct NodeValue {
    id: u64,
    labels: Vec<String>,
    props: Vec<(Arc<str>, Datum)>,
}

pub struct RelValue {
    id: u64,
    reltype: String,
    start: u64,
    end: u64,
    props: Vec<(Arc<str>, Datum)>,
}

pub struct ListValue {
    values: Vec<Datum>,
}

pub struct StructValue {
    fields: Vec<(Arc<str>, Datum)>,
}

pub enum Datum {
    // primitives
    Integer(i64),
    Float(f64),
    String(String),
    // graph
    Node(NodeValue),
    Rel(RelValue),
    // nested
    List(ListValue),
    Struct(StructValue),
}

// 不和 datatype 绑定，这个是 execution 阶段的类型。不一定是 schema 中的类型。

pub enum Array {
    IdArray,
    AnyArray,
    NodeArray,
    RelArray,
    ListArray,
    StructArray,
}

pub struct IdArray {
    data: Box<[u64]>,
    valid: BitVec,
}

pub struct IdArrayBuilder {}

pub struct AnyArray {
    data: Box<[Datum]>,
    valid: BitVec,
}

pub struct AnyArrayBuilder {}

pub struct ListArray {
    offsets: Box<[usize]>,
    child: Box<Array>,
    valid: BitVec,
}

pub struct ListArrayBuilder {}

pub struct StructArray {
    fields: Vec<(Arc<str>, Array)>,
    valid: BitVec,
}

pub struct StructArrayBuilder {}

pub struct NodeArray {
    ids: Box<[u64]>,
    label_offsets: Box<[usize]>,
    label_values: Box<[String]>,
    props: Box<[Datum]>,
    valid: BitVec,
}

pub struct NodeArrayBuilder {}

pub struct RelArray {
    ids: Box<[u64]>,
    reltypes: Box<[String]>,
    start_ids: Box<[u64]>,
    end_ids: Box<[u64]>,
    props: Box<[Datum]>,
    valid: BitVec,
}

pub struct RelArrayBuilder {}

pub enum ArrayBuilder {
    IdArray(IdArrayBuilder),
    AnyArray(AnyArrayBuilder),
    NodeArray(NodeArrayBuilder),
    RelArray(RelArrayBuilder),
    ListArray(ListArrayBuilder),
    StructArray(StructArrayBuilder),
}

pub struct DataChunk {
    pub columns: Vec<Array>,
}
