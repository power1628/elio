// Copyright 2022 Alex Chi. Licensed under Apache-2.0.

//! Contains array types for the system
//!
//! This crate contains two category of structs -- ArrayBuilder and Array. Developers may use
//! ArrayBuilder to create an Array. ArrayBuilder and Array are reciprocal traits. We can associate
//! an Array with an ArrayBuilder at compile time. This module also contains examples on how to use
//! generics around the Array and ArrayBuilder.
//!
//! This file is derived from https://github.com/skyzh/type-exercise-in-rust

pub mod boolean;
pub mod buffer;
pub mod chunk;
pub mod impls;
pub mod iterator;
pub mod list;
pub mod node;
pub mod primitive;
pub mod prop;
pub mod prop_map;
pub mod rel;
pub mod string;
pub use boolean::*;
use enum_as_inner::EnumAsInner;
pub use iterator::*;
use ordered_float::OrderedFloat;
pub use primitive::*;
pub use string::*;

use crate::array::buffer::BufferElementType;
use crate::array::list::{ListArray, ListArrayBuilder};
use crate::array::node::{NodeArray, NodeArrayBuilder};
use crate::array::prop::{PropertyArray, PropertyArrayBuilder};
use crate::array::prop_map::{PropertyMapArray, PropertyMapArrayBuilder};
use crate::array::rel::{RelArray, RelArrayBuilder};
use crate::data_type::DataType;
use crate::scalar::{DatumRef, Scalar, ScalarRef};
use crate::{NodeId, RelationshipId};

pub mod mask;

pub trait PrimitiveArrayElementType:
    BufferElementType + Clone + Copy + std::fmt::Debug + Sized + Send + Sync + Default + PartialEq + Eq + std::hash::Hash
{
    fn data_type() -> DataType;
}

impl PrimitiveArrayElementType for u16 {
    fn data_type() -> DataType {
        DataType::U16
    }
}
// impl PrimitiveType for u32 {}
impl PrimitiveArrayElementType for i64 {
    fn data_type() -> DataType {
        DataType::Integer
    }
}
// impl PrimitiveType for u64 {}
// impl PrimitiveType for f32 {}
impl PrimitiveArrayElementType for OrderedFloat<f64> {
    fn data_type() -> DataType {
        DataType::Float
    }
}
// impl PrimitiveType for usize {}
impl PrimitiveArrayElementType for NodeId {
    fn data_type() -> DataType {
        DataType::NodeId
    }
}
impl PrimitiveArrayElementType for RelationshipId {
    fn data_type() -> DataType {
        DataType::RelId
    }
}

/// [`Array`] is a collection of data of the same type.
pub trait Array: Send + Sync + Sized + 'static + Into<ArrayImpl> + Clone {
    /// The corresponding [`ArrayBuilder`] of this [`Array`].
    ///
    /// We constriant the associated type so that `Self::Builder::Array = Self`.
    /// TODO(pgao): remove this builder, builder should be constructed with runtime datatype.
    type Builder: ArrayBuilder<Array = Self>;

    /// The owned item of this array.
    type OwnedItem: Scalar<ArrayType = Self>;

    /// Type of the item that can be retrieved from the [`Array`]. For example, we can get a `i32`
    /// from [`I32Array`], while [`StringArray`] produces a `&str`. As we need a lifetime that is
    /// the same as `self` for `&str`, we use GAT here.
    type RefItem<'a>: ScalarRef<'a, ScalarType = Self::OwnedItem, ArrayType = Self>;

    /// Retrieve a reference to value.
    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;

    /// # SAFETY
    /// When calling, user should ensure `idx` is within bounds and the value is not null.
    unsafe fn get_unchecked(&self, idx: usize) -> Self::RefItem<'_>;

    /// Number of items of array.
    fn len(&self) -> usize;

    /// Indicates whether this array is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get iterator of this array.
    fn iter(&self) -> ArrayIterator<'_, Self>;

    // /// Build array from slice
    // fn from_slice(data: &[Option<Self::RefItem<'_>>]) -> Self {
    //     let mut builder = Self::Builder::with_capacity(data.len());
    //     for item in data {
    //         builder.push(*item);
    //     }
    //     builder.finish()
    // }

    fn data_type(&self) -> DataType;
}

/// [`ArrayBuilder`] builds an [`Array`].
pub trait ArrayBuilder {
    /// The corresponding [`Array`] of this [`ArrayBuilder`].
    ///
    /// Here we use associated type to constraint the [`Array`] type of this builder, so that
    /// `Self::Array::Builder == Self`. This property is very useful when constructing generic
    /// functions, and may help a lot when implementing expressions.
    type Array: Array<Builder = Self>;

    /// Create a new builder with `capacity`.
    fn with_capacity(capacity: usize, typ: DataType) -> Self;

    /// Append a value to builder.
    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>);
    // fn push(&mut self, value: Option<<<Self::Array as Array>::OwnedItem as Scalar>::RefType<'_>>);

    /// Finish build and return a new array.
    fn finish(self) -> Self::Array;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Clone, Debug, EnumAsInner, PartialEq, Eq, Hash)]
pub enum ArrayImpl {
    Bool(BoolArray),
    Integer(IntegerArray),
    Float(FloatArray),
    String(StringArray),
    U16(U16Array),
    NodeId(NodeIdArray),
    RelId(RelIdArray),
    Node(NodeArray),
    Rel(RelArray),
    List(ListArray),
    Property(PropertyArray),
    PropertyMap(PropertyMapArray),
}

impl ArrayImpl {
    pub fn iter(&self) -> impl Iterator<Item = DatumRef<'_>> {
        (0..self.len()).map(|i| self.get(i))
    }
}

// TODO(pgao): remove this, this enum as inner does the same thing
#[derive(Clone, Debug, EnumAsInner)]
pub enum ArrayImplRef<'a> {
    Bool(&'a BoolArray),
    Integer(&'a IntegerArray),
    Float(&'a FloatArray),
    String(&'a StringArray),
    U16(&'a U16Array),
    NodeId(&'a NodeIdArray),
    RelId(&'a RelIdArray),
    Node(&'a NodeArray),
    Rel(&'a RelArray),
    List(&'a ListArray),
    Property(&'a PropertyArray),
    PropertyMap(&'a PropertyMapArray),
}

/// Encapsules all variants of array builders in this library.
pub enum ArrayBuilderImpl {
    Bool(BoolArrayBuilder),
    String(StringArrayBuilder),
    Integer(IntegerArrayBuilder),
    Float(FloatArrayBuilder),
    U16(U16ArrayBuilder),
    NodeId(NodeIdArrayBuilder),
    RelId(RelIdArrayBuilder),
    Node(NodeArrayBuilder),
    Rel(RelArrayBuilder),
    List(ListArrayBuilder),
    Property(PropertyArrayBuilder),
    PropertyMap(PropertyMapArrayBuilder),
}

impl ArrayBuilderImpl {
    // TODO(pgao): needs to be refactored, use macro rules
    pub fn with_capacity(capacity: usize, typ: DataType) -> Self {
        match typ {
            DataType::Null => unreachable!("null type is not supported"),
            DataType::Bool => Self::Bool(BoolArrayBuilder::with_capacity(capacity, typ)),
            DataType::Integer => Self::Integer(IntegerArrayBuilder::with_capacity(capacity, typ)),
            DataType::Float => Self::Float(FloatArrayBuilder::with_capacity(capacity, typ)),
            DataType::String => Self::String(StringArrayBuilder::with_capacity(capacity, typ)),
            DataType::U16 => Self::U16(U16ArrayBuilder::with_capacity(capacity, typ)),
            DataType::NodeId => Self::NodeId(NodeIdArrayBuilder::with_capacity(capacity, typ)),
            DataType::RelId => Self::RelId(RelIdArrayBuilder::with_capacity(capacity, typ)),
            DataType::List(_) => Self::List(ListArrayBuilder::with_capacity(capacity, typ)),
            DataType::Node => Self::Node(NodeArrayBuilder::with_capacity(capacity, typ)),
            DataType::Rel => Self::Rel(RelArrayBuilder::with_capacity(capacity, typ)),
            DataType::Path => todo!("support path array"),
            DataType::Property => Self::Property(PropertyArrayBuilder::with_capacity(capacity, typ)),
            DataType::PropertyMap => Self::PropertyMap(PropertyMapArrayBuilder::with_capacity(capacity, typ)),
        }
    }
}
