use mojito_common::variable::VariableName;

use crate::{
    expr::FilterExprs,
    ir::node_connection::{ExhaustiveNodeConnection, RelPattern},
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
#[derive(Clone)]
pub struct NodeConnections {
    pub connections: Vec<ExhaustiveNodeConnection>,
}

impl NodeConnections {
    pub fn endpoint_nodes(&self) -> Vec<&VariableName> {
        let mut nodes = Vec::new();
        for conn in &self.connections {
            nodes.extend(conn.endpoint_nodes());
        }
        nodes
    }
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

#[derive(Clone)]
pub struct SelectivePathPattern {
    path_pattern: NodeConnections,
    filter: FilterExprs,
    selector: Selector,
}

impl SelectivePathPattern {
    pub fn endpoint_nodes(&self) -> Vec<&VariableName> {
        self.path_pattern.endpoint_nodes()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Selector {
    /// Any k paths
    AnyK(i64),
    // shortest k paths
    ShortestK(i64),
    // TODO(pgao): Shortest K GROUPS
    ShortestKGroup(i64),
}
