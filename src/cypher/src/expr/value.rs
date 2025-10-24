use mojito_common::{data_type::DataType, value::Value};

#[derive(Debug, Clone)]
pub struct Constant {
    data: Value,
    typ: DataType,
}

impl Constant {
    pub fn boolean(b: bool) -> Self {
        Self {
            data: Value::Boolean(b),
            typ: DataType::Boolean,
        }
    }

    pub fn integer(i: i64) -> Self {
        Self {
            data: Value::Integer(i),
            typ: DataType::Integer,
        }
    }

    pub fn float(f: f64) -> Self {
        Self {
            data: Value::Float(f),
            typ: DataType::Float,
        }
    }

    pub fn string(s: String) -> Self {
        Self {
            data: Value::String(s),
            typ: DataType::String,
        }
    }
}
