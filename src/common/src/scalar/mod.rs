// Copyright 2022 Alex Chi. Licensed under Apache-2.0.

//! Contains types for single values
//!
//! This crate contains two reciprocal traits -- Scalar and ScalarRef. As it is named, Scalar is an
//! owned value of ScalarRef, and ScalarRef is a reference to Scalar. We associate Scalar and
//! ScalarRef with Array types, and present examples on how to use these traits.
//!
//! This file is derived from https://github.com/skyzh/type-exercise-in-rust

use crate::array::Array;
use crate::data_type::F64;
use crate::scalar::list::{ListValue, ListValueRef};
use crate::store_types::PropertyValue;
use crate::{NodeId, RelationshipId};
pub mod impls;
pub mod list;
pub mod node;
pub mod prop_map;
pub mod prop_value;
pub mod rel;
pub use node::*;
use ordered_float::OrderedFloat;
pub use prop_map::*;
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ScalarImpl {
    Bool(bool),
    Integer(i64),
    Float(OrderedFloat<f64>),
    String(String),
    U16(u16),
    NodeId(NodeId),
    RelId(RelationshipId),
    List(ListValue),
    Node(NodeValue),
    Rel(RelValue),
    Property(PropertyValue),
    PropertyMap(PropertyMapValue),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarRefImpl<'a> {
    Bool(bool),
    Integer(i64),
    Float(F64),
    String(&'a str),
    U16(u16),
    NodeId(NodeId),
    RelId(RelationshipId),
    Node(NodeValueRef<'a>),
    Rel(RelValueRef<'a>),
    List(ListValueRef<'a>),
    Property(&'a PropertyValue),
    PropertyMap(PropertyMapValueRef<'a>),
}

impl<'a> ScalarRefImpl<'a> {
    /// Convert ScalarRefImpl to ScalarImpl
    pub fn to_owned_scalar(&self) -> ScalarImpl {
        match self {
            Self::Bool(b) => ScalarImpl::Bool(*b),
            Self::Integer(i) => ScalarImpl::Integer(*i),
            Self::Float(f) => ScalarImpl::Float(*f),
            Self::String(s) => ScalarImpl::String(s.to_string()),
            Self::U16(u) => ScalarImpl::U16(*u),
            Self::NodeId(n) => ScalarImpl::NodeId(*n),
            Self::RelId(r) => ScalarImpl::RelId(*r),
            Self::Node(n) => ScalarImpl::Node(n.to_owned_scalar()),
            Self::Rel(r) => ScalarImpl::Rel(r.to_owned_scalar()),
            Self::List(l) => ScalarImpl::List(l.to_owned_scalar()),
            Self::Property(p) => ScalarImpl::Property(p.to_owned_scalar()),
            Self::PropertyMap(m) => ScalarImpl::PropertyMap(m.to_owned_scalar()),
        }
    }
}

pub type Datum = Option<ScalarImpl>;
pub type DatumRef<'a> = Option<ScalarRefImpl<'a>>;

pub type Row = Vec<Datum>;
