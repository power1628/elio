// Copyright 2022 Alex Chi. Licensed under Apache-2.0.

//! Contains types for single values
//!
//! This crate contains two reciprocal traits -- Scalar and ScalarRef. As it is named, Scalar is an
//! owned value of ScalarRef, and ScalarRef is a reference to Scalar. We associate Scalar and
//! ScalarRef with Array types, and present examples on how to use these traits.
//!
//! This file is derived from https://github.com/skyzh/type-exercise-in-rust

use crate::array::Array;
use crate::scalar::list::{ListValue, ListValueRef};
use crate::store_types::PropertyValue;
pub mod impls;
pub mod list;
pub mod node;
pub mod prop_map;
pub mod prop_value;
pub mod rel;
pub use node::*;
pub use prop_map::*;
pub use prop_value::*;
pub use rel::*;

pub trait Scalar: std::fmt::Debug + Clone + Send + Sync + 'static + Into<ScalarImpl>
where
    for<'a> Self::ArrayType: Array<RefItem<'a> = Self::RefType<'a>>,
{
    type ArrayType: Array<OwnedItem = Self>;
    type RefType<'a>: ScalarRef<'a, ScalarType = Self, ArrayType = Self::ArrayType>;

    /// Get a reference of the current value
    fn as_scalar_ref(&self) -> Self::RefType<'_>;
}

/// A borrowed value.
///
/// For example, `i32`, `&str` both implements [`ScalarRef`].
pub trait ScalarRef<'a>: std::fmt::Debug + Clone + Copy + Send + 'a + Into<ScalarRefImpl<'a>> {
    // corresponding array type
    type ArrayType: Array<RefItem<'a> = Self>;

    // corresponding scalar type
    type ScalarType: Scalar<RefType<'a> = Self>;

    fn to_owned_scalar(&self) -> Self::ScalarType;
}

#[derive(Debug, Clone)]
pub enum ScalarImpl {
    Bool(bool),
    String(String),
    Node(NodeValue),
    Rel(RelValue),
    List(ListValue),
    Property(PropertyValue),
    PropertyMap(PropertyMapValue),
}

#[derive(Debug, Clone, Copy)]
pub enum ScalarRefImpl<'a> {
    Bool(bool),
    String(&'a str),
    Node(NodeValueRef<'a>),
    Rel(RelValueRef<'a>),
    List(ListValueRef<'a>),
    Property(&'a PropertyValue),
    PropertyMap(&'a PropertyMapValue),
}

pub type Datum = Option<ScalarImpl>;
pub type DatumRef<'a> = Option<ScalarRefImpl<'a>>;
