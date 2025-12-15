use std::sync::Arc;

use derive_more::Display;

use crate::array::PhysicalType;

pub type F64 = ordered_float::OrderedFloat<f64>;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Display)]
pub enum DataType {
    // basic
    Bool,
    Integer,
    Float,
    String,
    U16,
    Any,
    // graph
    VirtualNode,
    VirtualRel, // TODO(pgao): should be removed in the future
    VirtualPath,
    Node,
    Rel,
    Path,
    // structure
    #[display("List({})", _0)]
    List(Box<DataType>),
    #[display("Struct({})", _0.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<_>>().join(", "))]
    Struct(Vec<(Arc<str>, DataType)>),
    // // closed dynamic union type
    // // #[display("Union({})", _0.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "))]
    // // Union(Vec<DataType>),
}

impl DataType {
    pub fn is_node(&self) -> bool {
        matches!(self, DataType::Node | DataType::VirtualNode)
    }

    pub fn is_rel(&self) -> bool {
        matches!(self, DataType::Rel | DataType::VirtualRel)
    }

    pub fn is_entity(&self) -> bool {
        self.is_node() || self.is_rel()
    }

    pub fn new_struct(fields: impl Iterator<Item = (Arc<str>, DataType)>) -> Self {
        Self::Struct(fields.collect())
    }

    pub fn new_list(inner: DataType) -> Self {
        Self::List(Box::new(inner))
    }

    pub fn materialize(&self) -> Self {
        match self {
            DataType::VirtualNode => DataType::Node,
            DataType::VirtualRel => DataType::Rel,
            _ => self.clone(),
        }
    }
}

impl DataType {
    pub fn physical_type(&self) -> PhysicalType {
        match self {
            DataType::Bool | DataType::Integer | DataType::Float | DataType::String | DataType::U16 | DataType::Any => {
                PhysicalType::Any
            }
            DataType::VirtualNode => PhysicalType::VirtualNode,
            DataType::VirtualRel => PhysicalType::VirtualRel,
            DataType::VirtualPath => PhysicalType::VirtualPath,
            DataType::Node => PhysicalType::Node,
            DataType::Rel => PhysicalType::Rel,
            DataType::Path => PhysicalType::Path,
            DataType::List(inner) => PhysicalType::List(Box::new(inner.physical_type())),
            DataType::Struct(fields) => {
                let fields = fields
                    .iter()
                    .map(|(k, v)| (k.clone(), v.physical_type()))
                    .collect::<Vec<_>>();
                PhysicalType::Struct(fields.into_boxed_slice())
            }
        }
    }
}
