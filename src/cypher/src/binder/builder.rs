use crate::ir::query::{IrSingleQuery, IrSingleQueryPart};

// TODO(pgao): do we really need this structure?
pub struct IrSingleQueryBuilder {
    parts: Vec<IrSingleQueryPart>,
    // imported variables are stored in bctx::outer_scopes
    // we do not have imported variables here,
    // since when binding variables we need the symbol name, not variablename
}

impl IrSingleQueryBuilder {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub fn tail_mut(&mut self) -> Option<&mut IrSingleQueryPart> {
        self.parts.last_mut()
    }

    pub fn new_part(&mut self) {
        self.parts.push(IrSingleQueryPart::empty());
    }

    pub fn build(self) -> IrSingleQuery {
        IrSingleQuery { parts: self.parts }
    }
}
