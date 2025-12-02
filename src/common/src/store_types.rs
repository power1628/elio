use itertools::Itertools;

use crate::data_type::F64;
use crate::mapb::entry::EntryValueRef;

// TODO(pgao): binary representation
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PropertyValue {
    // TODO(pgao): maybe get rid of null here?
    Null,
    Boolean(bool),
    Integer(i64),
    Float(F64),
    String(String),
    // list
    ListBool(Vec<bool>),
    ListInteger(Vec<i64>),
    ListFloat(Vec<F64>),
    ListString(Vec<String>),
}

impl PropertyValue {
    pub fn pretty(&self) -> String {
        match self {
            PropertyValue::Null => "null".to_string(),
            PropertyValue::Boolean(b) => b.to_string(),
            PropertyValue::Integer(i) => i.to_string(),
            PropertyValue::Float(f) => f.to_string(),
            PropertyValue::String(s) => format!("\"{}\"", s),
            PropertyValue::ListBool(b) => format!("[{}]", b.iter().map(|b| b.to_string()).join(", ")),
            PropertyValue::ListInteger(i) => format!("[{}]", i.iter().map(|i| i.to_string()).join(", ")),
            PropertyValue::ListFloat(f) => format!("[{}]", f.iter().map(|f| f.to_string()).join(", ")),
            PropertyValue::ListString(s) => format!("[{}]", s.iter().map(|s| format!("\"{}\"", s)).join(", ")),
        }
    }
}

pub enum StoreDataType {
    Null,
    Boolean,
    Integer,
    Float,
    String,
    List(Box<StoreDataType>),
}

impl PropertyValue {
    pub fn data_type(&self) -> StoreDataType {
        match self {
            PropertyValue::Null => StoreDataType::Null,
            PropertyValue::Boolean(_) => StoreDataType::Boolean,
            PropertyValue::Integer(_) => StoreDataType::Integer,
            PropertyValue::Float(_) => StoreDataType::Float,
            PropertyValue::String(_) => StoreDataType::String,
            PropertyValue::ListBool(_) => StoreDataType::List(Box::new(StoreDataType::Boolean)),
            PropertyValue::ListInteger(_) => StoreDataType::List(Box::new(StoreDataType::Integer)),
            PropertyValue::ListFloat(_) => StoreDataType::List(Box::new(StoreDataType::Float)),
            PropertyValue::ListString(_) => StoreDataType::List(Box::new(StoreDataType::String)),
        }
    }
}

impl PropertyValue {
    pub fn from_map_entry_value(entry_value: EntryValueRef<'_>) -> Self {
        match entry_value {
            EntryValueRef::Null => PropertyValue::Null,
            EntryValueRef::Bool(b) => PropertyValue::Boolean(b),
            EntryValueRef::Integer(i) => PropertyValue::Integer(i),
            EntryValueRef::Float(f) => PropertyValue::Float(F64::from(f)),
            EntryValueRef::String(s) => PropertyValue::String(s.to_string()),
            EntryValueRef::ListBool(b) => PropertyValue::ListBool(b.to_vec()),
            EntryValueRef::ListInteger(i) => PropertyValue::ListInteger(i.iter().collect_vec()),
            EntryValueRef::ListFloat(f) => PropertyValue::ListFloat(f.iter().map(F64::from).collect_vec()),
            EntryValueRef::ListString(s) => PropertyValue::ListString(s.iter().map(|s| s.to_string()).collect_vec()),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum RelDirection {
    Incoming, // 0
    Outgoing, // 1
}
