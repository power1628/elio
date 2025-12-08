// Copyright 2022 Alex Chi. Licensed under Apache-2.0.

// Copyright 2022 RisingLight Project Authors. Licensed under Apache-2.0.
// This file is derived from https://github.com/skyzh/type-exercise-in-rust

//! Contains all macro-generated implementations of array methods

use crate::array::{Array, ArrayBuilder, ArrayBuilderImpl, ArrayImpl, ArrayImplRef};
use crate::data_type::DataType;
use crate::macros::for_all_variants;
use crate::scalar::ScalarRefImpl;

/// Implements dispatch functions for [`Array`]
macro_rules! impl_array_dispatch {
    ([], $( { $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty } ),*) => {
        impl ArrayImpl {

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

        /// Convert [`&ArrayImpl`] to [`&AbcArray`].
        $(
                #[doc = concat!("Implement [`&ArrayImpl`] -> [`&", stringify!($AbcArray), "`]")]
                impl<'a> From<&'a ArrayImpl> for &'a $AbcArray {
                    fn from(array: &'a ArrayImpl) -> &'a $AbcArray {
                        match array {
                            ArrayImpl::$Abc(array) => array,
                            _other => panic!("type mismatch, expected {}", stringify!($AbcArray)),
                        }
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
// macro_rules! impl_data_type {
//     (
//         [], $({ $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty }),*
//     ) => {
//         $(
//             impl $AbcArray {
//                 fn data_type(&self) -> DataType {
//                     DataType::$Abc
//                 }
//             }

//             impl $AbcArrayBuilder {
//                 fn data_type(&self) -> DataType {
//                     DataType::$Abc
//                 }
//             }
//         )*
//     };
// }

// for_all_variants! { impl_data_type }

/// Implements dispatch functions for [`ArrayBuilder`]
macro_rules! impl_array_builder_dispatch {
    ([], $( { $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty } ),*) => {
        impl ArrayBuilderImpl {

            pub fn append_n(&mut self, v: Option<ScalarRefImpl<'_>>, repeat: usize) {
                match (self, v) {
                    $(
                        (Self::$Abc(a), Some(ScalarRefImpl::$Abc(v))) => a.append_n(Some(v), repeat),
                        (Self::$Abc(a), None) => a.append_n(None, repeat),
                    )*
                    // (a, Some(b)) => panic!("type mismatch {}, expected {}", b.data_type(), a.data_type()),
                    (_, Some(_)) => panic!("type mismatch"),
                }
            }

            /// Appends an element to the back of array.
            pub fn append(&mut self, v: Option<ScalarRefImpl<'_>>) {
                match (self, v) {
                    $(
                        (Self::$Abc(a), Some(ScalarRefImpl::$Abc(v))) => a.append(Some(v)),
                        (Self::$Abc(a), None) => a.append(None),
                    )*
                    // (a, Some(b)) => panic!("type mismatch {}, expected {}", b.data_type(), a.data_type()),
                    (_, Some(_)) => panic!("type mismatch"),
                }
            }

            pub fn len(&self) -> usize {
                match self {
                    $(
                        Self::$Abc(a) => a.len(),
                    )*
                }
            }

            pub fn is_empty(&self) -> bool {
                match self {
                    $(
                        Self::$Abc(a) => a.is_empty(),
                    )*
                }
            }

            /// Finish build and return a new array.
            pub fn finish(self) -> ArrayImpl {
                match self {
                    $(
                        Self::$Abc(a) => ArrayImpl::$Abc(a.finish()),
                    )*
                }
            }

        }
    }
}

for_all_variants! { impl_array_builder_dispatch }

fn debug_array<A: Array>(f: &mut std::fmt::Formatter<'_>, array: &A) -> std::fmt::Result {
    f.debug_list().entries(array.iter()).finish()
}

/// Implements Debug for [`Array`]
macro_rules! impl_array_debug {
    (
        [], $({ $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty }),*
    ) => {
        $(
            impl std::fmt::Debug for $AbcArray {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    debug_array(f, self)
                }
            }
        )*
    };
}

for_all_variants! { impl_array_debug }
