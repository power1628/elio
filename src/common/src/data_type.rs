use derive_more::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Display)]
pub enum DataType {
    Null,
    Bool,
    Integer,
    Float,
    String,
    // for label data type
    U16,
    // node with id
    NodeId,
    // relationship id
    RelId,
    // composite
    #[display("List({})", _0)]
    List(Box<DataType>),
    // materialized node: labels and properties
    Node,
    // materialized rel: reltype and properties
    Rel,
    Path,
    // closed dynamic union type
    // #[display("Union({})", _0.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "))]
    // Union(Vec<DataType>),
    // Any type
    Property,
    // Any map type
    PropertyMap,
}

impl DataType {
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            DataType::Null
                | DataType::Bool
                | DataType::Integer
                | DataType::Float
                // | DataType::String
                | DataType::U16
        )
    }

    pub fn is_node(&self) -> bool {
        matches!(self, DataType::Node | DataType::NodeId)
    }

    pub fn is_rel(&self) -> bool {
        matches!(self, DataType::Rel | DataType::RelId)
    }

    pub fn is_entity(&self) -> bool {
        self.is_node() || self.is_rel()
    }
}
