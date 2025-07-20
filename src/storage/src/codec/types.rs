//! Encode datatypes into u8 bytes.
//! | b7 b6 b5 b4 b3 b2 b1 b0 |
//! | inner_type | outer_type  |
//! for nested type like list, the inner type should be encoded.

use mojito_common::data_type::DataType;

const INNER_TYPE_SHIFT: u8 = 4;
const OUTER_TYPE_MASK: u8 = 0x0F;

macro_rules! register_type_id  {
    ($($constant_name:ident = $value:expr),* $(,)?) => {
        $(
            pub const $constant_name: u8 = $value;
        )*
    };
}

register_type_id!(
    TYPE_ID_NULL = 0,
    TYPE_ID_BOOLEAN = 1,
    TYPE_ID_INTEGER = 2,
    TYPE_ID_FLOAT = 3,
    TYPE_ID_STRING = 4,
    TYPE_ID_LIST = 5,
    TYPE_ID_NODE = 6,
    TYPE_ID_RELATIONSHIP = 7,
    TYPE_ID_PATH = 8,
);

pub struct DataTypeFormat;

impl DataTypeFormat {
    // there's an requirment on whether the datatype can be stored as a property value
    pub fn storable(data_type: &DataType) -> bool {
        match data_type {
            DataType::Null | DataType::Boolean | DataType::Integer | DataType::Float | DataType::String => true,
            DataType::List(data_type) => data_type.is_primitive(),
            DataType::Node | DataType::Relationship | DataType::Path => false,
        }
    }

    pub fn encode(data_type: &DataType) -> u8 {
        match data_type {
            DataType::Null => TYPE_ID_NULL,
            DataType::Boolean => TYPE_ID_BOOLEAN,
            DataType::Integer => TYPE_ID_INTEGER,
            DataType::Float => TYPE_ID_FLOAT,
            DataType::String => TYPE_ID_STRING,
            DataType::List(inner) => {
                let inner_type = Self::encode(inner);
                (inner_type << INNER_TYPE_SHIFT) | TYPE_ID_LIST
            }
            DataType::Node | DataType::Relationship | DataType::Path => {
                unreachable!("Node/Relationship/Path should not be encoded as a property value")
            }
        }
    }
}

impl DataTypeFormat {
    pub fn decode(data_type: u8) -> DataType {
        let outer_type = data_type & OUTER_TYPE_MASK;
        let inner_type = data_type >> INNER_TYPE_SHIFT;
        match outer_type {
            TYPE_ID_NULL => DataType::Null,
            TYPE_ID_BOOLEAN => DataType::Boolean,
            TYPE_ID_INTEGER => DataType::Integer,
            TYPE_ID_FLOAT => DataType::Float,
            TYPE_ID_STRING => DataType::String,
            TYPE_ID_LIST => {
                let inner_type = Self::decode(inner_type);
                DataType::List(Box::new(inner_type))
            }
            _ => unreachable!("Invalid data type"),
        }
    }
}
