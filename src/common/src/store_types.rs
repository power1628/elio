use crate::data_type::F64;

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

#[repr(C)]
#[derive(Copy, Clone)]
pub enum RelDirection {
    Incoming, // 0
    Outgoing, // 1
}
