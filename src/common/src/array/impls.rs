// Copyright 2022 Alex Chi. Licensed under Apache-2.0.

// Copyright 2022 RisingLight Project Authors. Licensed under Apache-2.0.
// This file is derived from https://github.com/skyzh/type-exercise-in-rust

//! Contains all macro-generated implementations of array methods

use crate::array::ArrayBuilderImpl;
use crate::array::ArrayImpl;
use crate::array::ArrayImplRef;
use crate::macros::for_all_variants;

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

            #[doc = concat!("Implement [`", stringify!($AbcArrayBuilder), "`] -> [`ArrayBuilderImpl`]")]
            impl From<$AbcArrayBuilder> for ArrayBuilderImpl {
                fn from(builder: $AbcArrayBuilder) -> Self {
                    Self::$Abc(builder)
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
