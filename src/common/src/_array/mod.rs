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
    const DATA_TYPE: DataType;
}

impl PrimitiveArrayElementType for u16 {
    const DATA_TYPE: DataType = DataType::U16;
}
// impl PrimitiveType for u32 {}
impl PrimitiveArrayElementType for i64 {
    const DATA_TYPE: DataType = DataType::Integer;
}
impl PrimitiveArrayElementType for OrderedFloat<f64> {
    const DATA_TYPE: DataType = DataType::Float;
}
// impl PrimitiveType for usize {}
impl PrimitiveArrayElementType for NodeId {
    const DATA_TYPE: DataType = DataType::NodeId;
}
impl PrimitiveArrayElementType for RelationshipId {
    const DATA_TYPE: DataType = DataType::RelId;
}

/// [`Array`] is a collection of data of the same type.
pub trait Array: Send + Sync + Sized + 'static + Into<ArrayImpl> + Clone {
    /// The corresponding [`ArrayBuilder`] of this [`Array`].
    ///
    /// We constriant the associated type so that `Self::Builder::Array = Self`.
    /// This is used in expression evaluation, we need to know the output scalar's array builder
    /// This should only be used in the case of non-nested data types.
    /// For nested data types, e.g. List, we should use data type to create an array builder.
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
    /// NOTICE: should only be called for non-nested data types.
    /// List is nested, we do not know the inner data type at build
    /// so List will return an List<Integer> by default
    fn with_capacity(capacity: usize) -> Self;

    /// Append a value to builder.
    fn append_n(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>, repeat: usize);
    // fn push(&mut self, value: Option<<<Self::Array as Array>::OwnedItem as Scalar>::RefType<'_>>);

    fn append(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>) {
        self.append_n(value, 1);
    }

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
    pub fn new_builder(&self, capacity: usize) -> ArrayBuilderImpl {
        let typ = self.data_type();
        typ.array_builder(capacity)
    }

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
#[derive(Debug, EnumAsInner)]
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
