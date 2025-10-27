use derive_more::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Display)]
pub enum DataType {
    Null,
    Boolean,
    Integer,
    Float,
    String,
    // composite
    #[display("List({})", _0)]
    List(Box<DataType>),
    // map
    // structural
    Node,
    Relationship,
    Path,
    // closed dynamic union type
    #[display("Union({})", _0.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "))]
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
