use std::collections::HashSet;

use mojito_parser::ast::SemanticDirection;

use crate::{
    expr::{Expr, IrToken},
    variable::VariableName,
};

pub enum MutatingPattern {
    Create(CreatePattern),
}

/// semantic:
//   - nodes are ok to reference previous patterns in MATCH clause
//   - rels must not reference previous patterns, and should only be defined here.
pub struct CreatePattern {
    pub nodes: Vec<CreateNode>,
    pub rels: Vec<CreateRel>,
}

pub struct CreateNode {
    pub variable: VariableName,
    // labels are conjuncted with AND
    pub labels: HashSet<IrToken>,
    // CREATE (a:{name: "Bob"})
    // properties: (name, "bob")
    pub properties: Vec<(IrToken, Expr)>,
}

/// Relationship vairables MUST NOT reference previous pattern.
/// Relationship variables must be defined in CreatePattern scope.
pub struct CreateRel {
    pub variable: VariableName,
    pub left: VariableName,
    pub right: VariableName,
    pub reltype: IrToken,
    pub direction: SemanticDirection,
    pub properties: Vec<(IrToken, Expr)>,
}

impl CreateRel {
    // Return (start, end) node
    pub fn start_end_nodes(&self) -> (&VariableName, &VariableName) {
        if matches!(self.direction, SemanticDirection::Outgoing | SemanticDirection::Both) {
            (&self.left, &self.right)
        } else {
            (&self.right, &self.left)
        }
    }
}
