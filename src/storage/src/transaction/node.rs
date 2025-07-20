use bytes::BytesMut;
use mojito_common::{Label, NodeId, PropertyKey};

use crate::{
    codec::NodeWriter, error::GraphStoreError, model::node_key, transaction::GraphWrite, types::PropertyValue,
};

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

        let mut property_key_ids = vec![];
        let mut property_values = vec![];

        for (key, value) in properties {
            let property_id = self.register_property_key(&key)?;
            property_key_ids.push(property_id);
            property_values.push(value);
        }

        let mut buf = BytesMut::new();
        let mut writer = NodeWriter::new(&mut buf);
        writer.write_labels(&label_ids);
        writer.write_properties(&property_key_ids, &property_values);

        writer.finish();

        // insert
        let key = node_key(node_id);
        let value = buf.freeze();
        self.table_mut()
            .insert(key.as_slice(), value.as_ref())
            .map_err(Box::new)?;
        Ok(node_id)
    }
}
