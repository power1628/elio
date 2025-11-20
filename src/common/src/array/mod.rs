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
pub mod string;
use crate::scalar::{Scalar, ScalarRef};
pub use boolean::*;
pub use iterator::*;
pub use string::*;

pub mod mask;
// pub mod primitive_array;

/// [`Array`] is a collection of data of the same type.
pub trait Array: Send + Sync + Sized + 'static + Into<ArrayImpl> + Clone {
    /// The corresponding [`ArrayBuilder`] of this [`Array`].
    ///
    /// We constriant the associated type so that `Self::Builder::Array = Self`.
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

    /// Build array from slice
    fn from_slice(data: &[Option<Self::RefItem<'_>>]) -> Self {
        let mut builder = Self::Builder::with_capacity(data.len());
        for item in data {
            builder.push(*item);
        }
        builder.finish()
    }
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
    fn with_capacity(capacity: usize) -> Self;

    /// Append a value to builder.
    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>);
    // fn push(&mut self, value: Option<<<Self::Array as Array>::OwnedItem as Scalar>::RefType<'_>>);

    /// Finish build and return a new array.
    fn finish(self) -> Self::Array;
}

#[derive(Clone, Debug)]
pub enum ArrayImpl {
    Bool(BoolArray),
    String(StringArray),
}

#[derive(Clone, Debug)]
pub enum ArrayImplRef<'a> {
    Bool(&'a BoolArray),
    String(&'a StringArray),
}

/// Encapsules all variants of array builders in this library.
pub enum ArrayBuilderImpl {
    Bool(BoolArrayBuilder),
    String(StringArrayBuilder),
}
