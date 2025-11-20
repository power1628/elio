// Copyright 2022 Alex Chi. Licensed under Apache-2.0.

// Copyright 2022 RisingLight Project Authors. Licensed under Apache-2.0.

//! Necessary macros to cover variants of array types.
//!
//! This file is derived from https://github.com/skyzh/type-exercise-in-rust

/// `for_all_variants` includes all variants of our array types. If you added a new array
/// type inside the project, be sure to add a variant here.
///
/// Every tuple has 6 elements, where
/// `{ enum variant name, function suffix name, array type, builder type, scalar type, scalar ref type }`
macro_rules! for_all_variants {
    ($macro:ident $(, $x:ident)*) => {
        $macro! {
            [$($x),*],
            { Bool, bool, crate::array::BoolArray, crate::array::BoolArrayBuilder, bool, bool },
            { String, string, crate::array::StringArray, crate::array::StringArrayBuilder, String, &'a str }
        }
    };
}

pub(crate) use for_all_variants;

macro_rules! for_all_primitive_variants {
    ($macro:ident $(, $x:ident)*) => {
        $macro! {
            [$($x),*],
            { Bool, bool, crate::array::BoolArray, crate::array::BoolArrayBuilder, bool, bool }
        }
    };
}
pub(crate) use for_all_primitive_variants;
