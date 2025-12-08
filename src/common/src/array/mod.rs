// pub mod array;
// pub mod basic;
pub mod any;
pub mod chunk;
pub mod datum;
pub mod list;
pub mod node;
pub mod rel;
pub mod struct_;

use std::sync::Arc;

pub use any::*;
pub use chunk::*;
pub use list::*;
pub use node::*;
pub use rel::*;
pub use struct_::*;

pub enum ArrayImpl {
    NodeId(NodeIdArray),
    Any(AnyArray),
    // graph
    Node(NodeArray),
    Rel(RelArray),
    // structure
    List(ListArray),
    Struct(StructArray),
}

pub enum ArrayBuilderImpl {
    NodeId(NodeIdArrayBuilder),
    Any(AnyArrayBuilder),
    // graph
    Node(NodeArrayBuilder),
    Rel(RelArrayBuilder),
    // structure
    List(ListArrayBuilder),
    Struct(StructArrayBuilder),
}

macro_rules! impl_array_builder_dispatch {
    ($({$name:ident, $variant:ident}),*) => {
        impl ArrayBuilderImpl {
            pub fn finish(self) -> ArrayImpl {
                match self{
                    $(ArrayBuilderImpl::$variant(b) => ArrayImpl::$variant(b.finish()),)*
                }
            }
        }
    };
}

impl_array_builder_dispatch!(
    {NodeId, NodeId},
    {Any, Any},
    {Node, Node},
    {Rel, Rel},
    {List, List},
    {Struct, Struct}
);

// physical array type
pub enum PhysicalType {
    // basic
    NodeId,
    Any,
    // graph
    Node,
    Rel,
    // structure
    List(Box<PhysicalType>),
    // (field_name, field type)
    Struct(Box<[(Arc<str>, PhysicalType)]>),
}

impl PhysicalType {
    pub fn array_builder(&self, capacity: usize) -> ArrayBuilderImpl {
        match self {
            PhysicalType::NodeId => ArrayBuilderImpl::NodeId(NodeIdArrayBuilder::with_capacity(capacity)),
            PhysicalType::Any => ArrayBuilderImpl::Any(AnyArrayBuilder::with_capacity(capacity)),
            PhysicalType::Node => ArrayBuilderImpl::Node(NodeArrayBuilder::with_capacity(capacity)),
            PhysicalType::Rel => ArrayBuilderImpl::Rel(RelArrayBuilder::with_capacity(capacity)),
            PhysicalType::List(inner) => {
                let child = inner.array_builder(capacity);
                ArrayBuilderImpl::List(ListArrayBuilder::new(Box::new(child)))
            }
            PhysicalType::Struct(fields) => {
                let mut field_builders = Vec::with_capacity(fields.len());
                for (name, ty) in fields {
                    field_builders.push((name.clone(), ty.array_builder(capacity)));
                }
                ArrayBuilderImpl::Struct(StructArrayBuilder::new(field_builders.into_iter()))
            }
        }
    }
}
