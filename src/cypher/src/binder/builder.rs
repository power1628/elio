use elio_common::schema::Variable;
use indexmap::IndexSet;

use crate::ir::query::{IrSingleQuery, IrSingleQueryPart};

// TODO(pgao): do we really need this structure?
pub struct IrSingleQueryBuilder {
    parts: Vec<IrSingleQueryPart>,
    // imported variables are stored in bctx::outer_scopes
    // we do not have imported variables here,
    // since when binding variables we need the symbol name, not variablename
}

impl Default for IrSingleQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IrSingleQueryBuilder {
    pub fn new() -> Self {
        Self {
            parts: vec![IrSingleQueryPart::empty()],
        }
    }

    pub fn tail_mut(&mut self) -> Option<&mut IrSingleQueryPart> {
        self.parts.last_mut()
    }

    pub fn new_tail(&mut self, imported: IndexSet<Variable>) {
        let mut tail = IrSingleQueryPart::empty();
        tail.query_graph.add_imported_set(&imported);
        self.parts.push(tail);
    }

    pub fn build(self) -> IrSingleQuery {
        IrSingleQuery { parts: self.parts }
    }
}
