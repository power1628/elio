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
}

impl DataType {
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            DataType::Null | DataType::Boolean | DataType::Integer | DataType::Float | DataType::String
        )
    }
}
