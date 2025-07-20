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
