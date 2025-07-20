//! PropertyFormat
//! All entities properties are stored in an continous block
//!
//! The block is composed of three parts:
//!   - header: 2B, u16, which contains the number of keys in the block
//!   - keys: 8B * num_keys, which contains the key_id(u16) and the size of the value(u32)
//!
//! ```
//!       8B             8*num_keys B             variable size          
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

pub struct PropertyFormat {}
