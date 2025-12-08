// Copyright 2022 Alex Chi. Licensed under Apache-2.0.

//! Contains array types for the system
//!
//! This crate contains two category of structs -- ArrayBuilder and Array. Developers may use
//! ArrayBuilder to create an Array. ArrayBuilder and Array are reciprocal traits. We can associate
//! an Array with an ArrayBuilder at compile time. This module also contains examples on how to use
//! generics around the Array and ArrayBuilder.
//! This file is derived from type-exercise-in-rust and modified by Mojito.

// pub mod array;
// pub mod basic;
pub mod any;
pub mod chunk;
pub mod datum;
pub mod list;
pub mod node;
pub mod rel;
pub mod struct_;

pub mod iter;

use std::sync::Arc;

pub use any::*;
use bitvec::prelude::*;
pub use chunk::*;
use datum::ScalarRef;
use enum_as_inner::EnumAsInner;
pub use list::*;
pub use node::*;
pub use rel::*;
pub use struct_::*;

use crate::array::iter::ArrayIterator;

/// [`Array`] is a collection of data of the same type.
pub trait Array: Send + Sync + Sized + 'static + Into<ArrayImpl> + std::fmt::Debug + Clone
// where
// for<'a> Self::OwnedItem: Scalar<RefType<'a> = Self::RefItem<'a>>,
{
    /// The corresponding [`ArrayBuilder`] of this [`Array`].
    ///
    /// We constriant the associated type so that `Self::Builder::Array = Self`.
    type Builder: ArrayBuilder<Array = Self>;

    /// The owned item of this array.
    // type OwnedItem: Scalar<ArrayType = Self>;

    /// Type of the item that can be retrieved from the [`Array`]. For example, we can get a `i32`
    /// from [`I32Array`], while [`StringArray`] produces a `&str`. As we need a lifetime that is
    /// the same as `self` for `&str`, we use GAT here.
    type RefItem<'a>;

    /// Retrieve a reference to value.
    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;

    /// Number of items of array.
    fn len(&self) -> usize;

    /// Indicates whether this array is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get iterator of this array.
    fn iter(&self) -> ArrayIterator<Self> {
        ArrayIterator::new(self)
    }

    fn physical_type(&self) -> PhysicalType;
}

/// [`ArrayBuilder`] builds an [`Array`].
pub trait ArrayBuilder {
    /// The corresponding [`Array`] of this [`ArrayBuilder`].
    ///
    /// Here we use associated type to constraint the [`Array`] type of this builder, so that
    /// `Self::Array::Builder == Self`. This property is very useful when constructing generic
    /// functions, and may help a lot when implementing expressions.
    type Array: Array<Builder = Self>;

    /// Append a value to builder.
    fn push_n(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>, repeat: usize);

    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>) {
        self.push_n(value, 1);
    }

    /// Finish build and return a new array.
    fn finish(self) -> Self::Array;
}

pub type ArrayRef = Arc<ArrayImpl>;

#[derive(EnumAsInner, Clone, Debug)]
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

            pub fn get(&self, idx: usize) -> Option<ScalarRef<'_>> {
                match self {
                    $(ArrayImpl::$variant(a) => a.get(idx).map(ScalarRef::from),)*
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

#[derive(Debug, EnumAsInner)]
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
