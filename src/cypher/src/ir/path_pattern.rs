use crate::{
    expr::FilterExprs,
    ir::node_connection::{ExhaustiveNodeConnection, RelPattern},
    variable::VariableName,
};

pub enum PathPattern {
    SingleNode(SingleNode),
    NodeConnections(NodeConnections),
    SelectivePathPattern(SelectivePathPattern),
}

impl PathPattern {
    pub fn as_node_connections(self) -> Option<NodeConnections> {
        match self {
            PathPattern::NodeConnections(node_connections) => Some(node_connections),
            _ => None,
        }
    }
}

// length 0 path pattern of single node
pub struct SingleNode {
    pub variable: VariableName,
}

// length 1 or more path pattern made of node connections
pub struct NodeConnections {
    pub connections: Vec<ExhaustiveNodeConnection>,
}

impl NodeConnections {
    pub fn as_rels(self) -> Option<Vec<RelPattern>> {
        let mut rels = Vec::new();
        for conn in self.connections {
            match conn {
                ExhaustiveNodeConnection::RelPattern(rel) => rels.push(rel),
                _ => return None,
            }
        }
        Some(rels)
    }
}

pub struct SelectivePathPattern {
    path_pattern: NodeConnections,
    filter: FilterExprs,
    selector: Selector,
}

pub enum Selector {
    /// Any k paths
    AnyK(i64),
    // shortest k paths
    ShortestK(i64),
    // TODO(pgao): Shortest K GROUPS
    ShortestKGroup(i64),
}
