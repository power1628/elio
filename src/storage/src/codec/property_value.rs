//! PropertyFormat
//! All entities properties are stored in an continous block
//!
//! The block is composed of three parts:
//!   - header: 2B, u16, which contains the number of keys in the block
//!   - keys: 8B * num_keys, which contains the key_id(u16) and the size of the value(u32)
//!
//! keys ::= key_id key_type padding value_bytes
//! key_id ::= u16
//! key_type ::= u8
//! padding ::= u8
//! value_bytes ::= u64
//!                 | bool(u8) with padding
//!                 | Integer(i64)
//!                 | Float(f64)
//!
//! value_data ::= string_data
//!                | boolean_list_data
//!                | integer_list_data
//!                | float_list_data
//!                | string_list_data
//! TODO(pgao): maybe we should have an StorageDataTypes, to make it easier to extend
//! ```
//!       2B             12*num_keys B             variable size          
//!   +-----------++------------------------++--------------------------+
//!   |keys_header||key_and_value_size_array||value_data                |
//!   +---+-------++------------------------++--------------------------+
//!       |        |                        |
//!       v        |                        |
//!    num_keys    |                        |
//!             +--+                        +--------------+
//!             v------------------++------------------+   v
//!             |key_id value_bytes||key_id value_bytes|....
//!             +------------------++------------------+
//!```
//!
//! For fixed size value, we embed the value in the key_and_value_size_array
//! For variable size value, we store the value in the value_data
//!

const KEY_HEADER_BYTES: usize = 2;
const KEY_ITEM_BYTES: usize = 12;

pub struct PropertyFormat {}

impl PropertyFormat {}
