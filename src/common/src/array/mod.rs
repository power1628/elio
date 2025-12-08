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
use bitvec::prelude::*;
pub use chunk::*;
use enum_as_inner::EnumAsInner;
pub use list::*;
pub use node::*;
pub use rel::*;
pub use struct_::*;

pub type ArrayRef = Arc<ArrayImpl>;

#[derive(EnumAsInner, Clone)]
pub enum ArrayImpl {
    Any(AnyArray),
    // graph
    VirtualNode(VirtualNodeArray),
    VirtualRel(VirtualRelArray),
    Node(NodeArray),
    Rel(RelArray),
    // structure
    List(ListArray),
    Struct(StructArray),
}

macro_rules! impl_array_dispatch {
    ($($variant:ident),*) => {
        impl ArrayImpl {
            pub fn physical_type(&self) -> PhysicalType {
                match self {
                    $(ArrayImpl::$variant(a) => a.physical_type(),)*
                }
            }

            pub fn valid_map(&self) -> &BitVec {
                match self {
                    $(ArrayImpl::$variant(a) => a.valid_map(),)*
                }
            }

            pub fn set_valid_map(&mut self, valid: BitVec) {
                match self {
                    $(ArrayImpl::$variant(a) => a.set_valid_map(valid),)*
                }
            }

            pub fn len(&self) -> usize {
                match self {
                    $(ArrayImpl::$variant(a) => a.len(),)*
                }
            }

        }
    };
}

impl_array_dispatch!(Any, VirtualNode, VirtualRel, Node, Rel, List, Struct);

macro_rules! impl_array_convert {
    ($({$Abc:ident, $AbcArray:ident}),*) => {

        $(
            impl From<$AbcArray> for ArrayImpl {
                fn from(array: $AbcArray) -> Self {
                    ArrayImpl::$Abc(array)
                }
            }
        )*

    };
}

impl_array_convert!(
{Any, AnyArray},
{VirtualNode, VirtualNodeArray},
{VirtualRel, VirtualRelArray},
{Node, NodeArray},
{Rel, RelArray},
{List, ListArray},
{Struct, StructArray});

#[derive(EnumAsInner)]
pub enum ArrayBuilderImpl {
    Any(AnyArrayBuilder),
    // graph
    VirtualNode(VirtualNodeArrayBuilder),
    VirtualRel(VirtualRelArrayBuilder),
    Node(NodeArrayBuilder),
    Rel(RelArrayBuilder),
    // structure
    List(ListArrayBuilder),
    Struct(StructArrayBuilder),
}

macro_rules! impl_array_builder_dispatch {
    ($($variant:ident),*) => {
        impl ArrayBuilderImpl {
            pub fn finish(self) -> ArrayImpl {
                match self{
                    $(ArrayBuilderImpl::$variant(b) => ArrayImpl::$variant(b.finish()),)*
                }
            }
        }
    };
}

impl_array_builder_dispatch!(Any, VirtualNode, VirtualRel, Node, Rel, List, Struct);

// physical array type
#[derive(Debug)]
pub enum PhysicalType {
    // basic
    Any,
    // graph
    VirtualNode,
    VirtualRel,
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
            PhysicalType::Any => ArrayBuilderImpl::Any(AnyArrayBuilder::with_capacity(capacity)),
            PhysicalType::VirtualNode => {
                ArrayBuilderImpl::VirtualNode(VirtualNodeArrayBuilder::with_capacity(capacity))
            }
            PhysicalType::VirtualRel => ArrayBuilderImpl::VirtualRel(VirtualRelArrayBuilder::with_capacity(capacity)),

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
