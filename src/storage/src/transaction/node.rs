use mojito_common::{Label, NodeId, PropertyKey};

use crate::{error::GraphStoreError, transaction::GraphWrite, types::PropertyValue};

impl GraphWrite {
    pub fn node_create(
        &mut self,
        labels: Vec<Label>,
        properties: Vec<(PropertyKey, PropertyValue)>,
    ) -> Result<NodeId, GraphStoreError> {
        let node_id = self.alloc_node_id()?;
        let mut label_ids = vec![];
        for label in labels {
            let label_id = self.register_label(&label)?;
            label_ids.push(label_id);
        }

        // write header
        // write lables
        // write properties

        todo!()
    }
}
