// Copyright 2022 Alex Chi. Licensed under Apache-2.0.

//! Contains types for single values
//!
//! This crate contains two reciprocal traits -- Scalar and ScalarRef. As it is named, Scalar is an
//! owned value of ScalarRef, and ScalarRef is a reference to Scalar. We associate Scalar and
//! ScalarRef with Array types, and present examples on how to use these traits.
//!
//! This file is derived from https://github.com/skyzh/type-exercise-in-rust

use crate::array::Array;

pub trait Scalar: std::fmt::Debug + Clone + Send + Sync + 'static + TryFrom<ScalarImpl> + Into<ScalarImpl> {
    type ArrayType: Array<OwnedItem = Self>;
    type RefType<'a>: ScalarRef<'a, ScalarType = Self, ArrayType = Self::ArrayType>;

    /// Get a reference of the current value
    fn as_scalar_ref(&self) -> Self::RefType<'_>;
}

/// A borrowed value.
///
/// For example, `i32`, `&str` both implements [`ScalarRef`].
pub trait ScalarRef<'a>:
    std::fmt::Debug + Clone + Copy + Send + 'a + TryFrom<ScalarRefImpl<'a>> + Into<ScalarRefImpl<'a>>
{
    // corresponding array type
    type ArrayType: Array<RefItem<'a> = Self>;

    // corresponding scalar type
    type ScalarType: Scalar<RefType<'a> = Self>;

    fn to_owned_scalar(&self) -> Self::ScalarType;
}

#[derive(Debug, Clone)]
pub enum ScalarImpl {
    Boolean(bool),
}

#[derive(Debug, Clone, Copy)]
pub enum ScalarRefImpl<'a> {
    Boolean(bool),
    String(&'a str),
}
