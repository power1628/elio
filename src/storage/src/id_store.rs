use redb::{Table, TableDefinition, TableHandle};

use crate::{error::Error, types::{NodeId, RelationshipId}};

const NODE_ID_KEY: &str = "id.node";
const RELATIONSHIP_ID_KEY: &str = "id.relationship";

const ID_TABLE_DEFINITION: TableDefinition<&str, NodeId> = TableDefinition::new("id");

pub struct IdStore<'a> {
    db: &'a redb::Database,
}

impl<'a> IdStore<'a> {
    /// new and init table, if table does not exists, create one.
    pub fn try_new(db: &'a redb::Database) -> Result<Self, Error> {
        let tx = db.begin_write();

        let handle = db.
        Self { db }
    }

    /// allocate node id
    pub fn alloc_node_id(&self) -> NodeId {
        todo!()
    }

    /// allocate relationship id
    pub fn alloc_relationship_id(&self) -> RelationshipId {
        todo!()
    }
}
