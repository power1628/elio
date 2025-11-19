// Copyright 2022 Alex Chi. Licensed under Apache-2.0.

// Copyright 2022 RisingLight Project Authors. Licensed under Apache-2.0.
// This file is derived from https://github.com/skyzh/type-exercise-in-rust

//! Contains all macro-generated implementations of array methods

use crate::array::Array;
use crate::array::ArrayBuilder;
use crate::array::ArrayBuilderImpl;
use crate::array::ArrayImpl;
use crate::array::ArrayImplRef;
use crate::data_type::DataType;
use crate::macros::for_all_variants;
use crate::scalar::ScalarRefImpl;

/// Implements dispatch functions for [`Array`]
macro_rules! impl_array_dispatch {
    ([], $( { $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty } ),*) => {
        impl ArrayImpl {
            /// Create new [`ArrayBuilder`] from [`Array`] type.
            pub fn new_builder(&self, capacity: usize) -> ArrayBuilderImpl {
                match self {
                    $(
                        Self::$Abc(_) => ArrayBuilderImpl::$Abc(<$AbcArrayBuilder>::with_capacity(capacity))
                    ),*
                }
            }

            /// Get the value at the given index.
            pub fn get(&self, idx: usize) -> Option<ScalarRefImpl<'_>> {
                match self {
                    $(
                        Self::$Abc(array) => array.get(idx).map(ScalarRefImpl::$Abc),
                    )*
                }
            }

            /// Number of items of array.
            pub fn len(&self) -> usize {
                match self {
                    $(
                        Self::$Abc(a) => a.len(),
                    )*
                }
            }

            /// Number of items of array.
            pub fn is_empty(&self) -> bool {
                match self {
                    $(
                        Self::$Abc(a) => a.is_empty(),
                    )*
                }
            }

            /// Get physical type of the current array
            pub fn data_type(&self) -> DataType {
                match self {
                    $(
                        Self::$Abc(a) => a.data_type(),
                    )*
                }
            }
        }
    }
}

for_all_variants! { impl_array_dispatch }

/// Implements `From` for [`Array`].
macro_rules! impl_array_conversion {
    ([], $({ $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty }),*) => {
        $(
            #[doc = concat!("Implement [`", stringify!($AbcArray), "`] -> [`ArrayImpl`]")]
            impl From<$AbcArray> for ArrayImpl {
                fn from(array: $AbcArray) -> Self {
                    Self::$Abc(array)
                }
            }

            #[doc = concat!("Implement [`ArrayImpl`] -> [`", stringify!($AbcArray), "`]")]
            impl From<ArrayImpl> for $AbcArray {
                fn from(array: ArrayImpl) -> Self {
                    match array {
                        ArrayImpl::$Abc(array) => array,
                        other => panic!("type mismatch {}, expected {}", other.data_type(), stringify!($AbcArray)),
                    }
                }
            }


            #[doc = concat!("Implement [`", stringify!($AbcArrayBuilder), "`] -> [`ArrayBuilderImpl`]")]
            impl From<$AbcArrayBuilder> for ArrayBuilderImpl {
                fn from(builder: $AbcArrayBuilder) -> Self {
                    Self::$Abc(builder)
                }
            }

        )*


        /// Convert [`ArrayImplRef<'a>`] to [`&AbcArray`].
        $(
            impl<'a> From<ArrayImplRef<'a>> for &'a $AbcArray {
                fn from(array_ref: ArrayImplRef<'a>) -> &'a $AbcArray {
                    match array_ref {
                        ArrayImplRef::$Abc(array) => array,
                        _other => panic!("type mismatch, expected {}", stringify!($AbcArray)),
                    }
                }
            }
        )*

        impl ArrayImpl {
            /// Convert [`&ArrayImpl`] to [`ArrayImplRef`].
            pub fn as_ref(&self) -> ArrayImplRef<'_> {
                match self {
                    $(
                        ArrayImpl::$Abc(array) => ArrayImplRef::$Abc(array),
                    )*
                }
            }
        }
    };
}

for_all_variants! { impl_array_conversion }

/// Implements `data_type` for [`Array`]
macro_rules! impl_data_type {
    (
        [], $({ $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty }),*
    ) => {
        $(
            impl $AbcArray {
                fn data_type(&self) -> DataType {
                    DataType::$Abc
                }
            }

            impl $AbcArrayBuilder {
                fn data_type(&self) -> DataType {
                    DataType::$Abc
                }
            }
        )*
    };
}

for_all_variants! { impl_data_type }
