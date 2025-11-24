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
            { Integer, integer, crate::array::IntegerArray, crate::array::IntegerArrayBuilder, i64, i64},
            { Float, float, crate::array::FloatArray, crate::array::FloatArrayBuilder, f64, f64},
            { TokenId, token_id, crate::array::TokenIdArray, crate::array::TokenIdArrayBuilder, u16, u16 },
            { NodeId, node_id, crate::array::NodeIdArray, crate::array::NodeIdArrayBuilder, u64, u64 },
            { RelId, rel_id, crate::array::RelIdArray, crate::array::RelIdArrayBuilder, u64, u64 },
            { String, string, crate::array::StringArray, crate::array::StringArrayBuilder, String, &'a str },
            { Node, node, crate::array::NodeArray, crate::array::NodeArrayBuilder, crate::scalar::NodeValue, crate::scalar::NodeValueRef<'a> },
            { Rel, rel, crate::array::RelArray, crate::array::RelArrayBuilder, crate::scalar::RelValue, crate::scalar::RelValueRef<'a> },
            { List, list, crate::array::ListArray, crate::array::ListArrayBuilder, crate::scalar::ListValue, crate::scalar::ListValueRef<'a> },
            { Property, any, crate::array::PropertyArray, crate::array::PropertyArrayBuilder, crate::scalar::PropertyValue, &'a crate::scalar::PropertyValue },
            { PropertyMap, map, crate::array::PropertyMapArray, crate::array::PropertyMapArrayBuilder, crate::scalar::PropertyMapValue, &'a crate::scalar::PropertyMapValue }
        }
    };
}

pub(crate) use for_all_variants;

macro_rules! for_all_primitive_variants {
    ($macro:ident $(, $x:ident)*) => {
        $macro! {
            [$($x),*],
            { Bool, bool, crate::array::BoolArray, crate::array::BoolArrayBuilder, bool, bool },
            { Integer, integer, crate::array::IntegerArray, crate::array::IntegerArrayBuilder, i64, i64},
            { Float, float, crate::array::FloatArray, crate::array::FloatArrayBuilder, f64, f64},
            { TokenId, token_id, crate::array::TokenIdArray, crate::array::TokenIdArrayBuilder, u16, u16 },
            { NodeId, node_id, crate::array::NodeIdArray, crate::array::NodeIdArrayBuilder, u64, u64 },
            { RelId, rel_id, crate::array::RelIdArray, crate::array::RelIdArrayBuilder, u64, u64 }
        }
    };
}
pub(crate) use for_all_primitive_variants;
