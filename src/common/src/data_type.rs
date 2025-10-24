#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DataType {
    Null,
    Boolean,
    Integer,
    Float,
    String,
    // composite
    List(Box<DataType>),
    // map
    // structural
    Node,
    Relationship,
    Path,
    // closed dynamic union type
    Union(Vec<DataType>),
    // Any type
    Any,
}

impl DataType {
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            DataType::Null | DataType::Boolean | DataType::Integer | DataType::Float | DataType::String
        )
    }
}
