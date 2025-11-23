use derive_more::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Display)]
pub enum DataType {
    Null,
    Bool,
    Integer,
    Float,
    String,
    // composite
    #[display("List({})", _0)]
    List(Box<DataType>),
    // materialized node: labels and properties
    Node,
    // node with id
    NodeRef,
    // materialized rel: reltype and properties
    Rel,
    // relationship id
    RelationshipRef,
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
            DataType::Null | DataType::Bool | DataType::Integer | DataType::Float | DataType::String
        )
    }

    pub fn is_node(&self) -> bool {
        matches!(self, DataType::Node | DataType::NodeRef)
    }

    pub fn is_rel(&self) -> bool {
        matches!(self, DataType::Rel | DataType::RelationshipRef)
    }

    pub fn is_entity(&self) -> bool {
        self.is_node() || self.is_rel()
    }
}
