use mojito_common::{LabelId, NodeId, PropertyKeyId, value::Value};

use crate::{error::GraphStoreError, transaction::Transaction};

impl Transaction {
    pub fn node_create(
        &self,
        labels: Vec<LabelId>,
        props: Vec<(PropertyKeyId, Value)>,
    ) -> Result<NodeId, GraphStoreError> {
        todo!()
    }
}
