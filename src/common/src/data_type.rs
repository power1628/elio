use derive_more::Display;

use crate::array::list::ListArrayBuilder;
use crate::array::node::NodeArrayBuilder;
use crate::array::prop::PropertyArrayBuilder;
use crate::array::prop_map::PropertyMapArrayBuilder;
use crate::array::rel::RelArrayBuilder;
use crate::array::{
    ArrayBuilder, ArrayBuilderImpl, BoolArrayBuilder, FloatArrayBuilder, IntegerArrayBuilder, NodeIdArrayBuilder,
    RelIdArrayBuilder, StringArrayBuilder, U16ArrayBuilder,
};

pub type F64 = ordered_float::OrderedFloat<f64>;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Display)]
pub enum DataType {
    Bool,
    Integer,
    Float,
    String,
    // for label data type
    U16,
    // node with id
    NodeId,
    // relationship id
    RelId,
    // composite
    #[display("List({})", _0)]
    List(Box<DataType>),
    // materialized node: labels and properties
    Node,
    // materialized rel: reltype and properties
    Rel,
    Path,
    // closed dynamic union type
    // #[display("Union({})", _0.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "))]
    // Union(Vec<DataType>),
    // Any property type
    Property,
    // Any property map type
    PropertyMap,
}

impl DataType {
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            // DataType::Null
            DataType::Bool | DataType::Integer | DataType::Float | DataType::U16 | DataType::NodeId | DataType::RelId
        )
    }

    pub fn is_node(&self) -> bool {
        matches!(self, DataType::Node | DataType::NodeId)
    }

    pub fn is_rel(&self) -> bool {
        matches!(self, DataType::Rel | DataType::RelId)
    }

    pub fn is_entity(&self) -> bool {
        self.is_node() || self.is_rel()
    }
}

impl DataType {
    pub fn array_builder(&self, capacity: usize) -> ArrayBuilderImpl {
        match self {
            DataType::Bool => BoolArrayBuilder::with_capacity(capacity).into(),
            DataType::Integer => IntegerArrayBuilder::with_capacity(capacity).into(),
            DataType::Float => FloatArrayBuilder::with_capacity(capacity).into(),
            DataType::String => StringArrayBuilder::with_capacity(capacity).into(),
            DataType::U16 => U16ArrayBuilder::with_capacity(capacity).into(),
            DataType::NodeId => NodeIdArrayBuilder::with_capacity(capacity).into(),
            DataType::RelId => RelIdArrayBuilder::with_capacity(capacity).into(),
            DataType::List(inner) => ListArrayBuilder::with_capacity_and_type(capacity, &inner).into(),
            DataType::Node => NodeArrayBuilder::with_capacity(capacity).into(),
            DataType::Rel => RelArrayBuilder::with_capacity(capacity).into(),
            DataType::Path => todo!(),
            DataType::Property => PropertyArrayBuilder::with_capacity(capacity).into(),
            DataType::PropertyMap => PropertyMapArrayBuilder::with_capacity(capacity).into(),
        }
    }
}
