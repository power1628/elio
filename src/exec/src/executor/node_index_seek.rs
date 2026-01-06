//! NodeIndexSeek executor - uses unique index for O(1) node lookup

use std::sync::Arc;

use async_stream::try_stream;
use bitvec::vec::BitVec;
use elio_common::array::chunk::DataChunk;
use elio_common::array::{ArrayImpl, VirtualNodeArrayBuilder};
use elio_common::schema::Schema;
use elio_common::{LabelId, PropertyKeyId};
use futures::StreamExt;

use super::*;
use crate::error::ExecError;
use crate::executor::Executor;

/// Executor that performs index lookup to find nodes
#[derive(Debug)]
pub struct NodeIndexSeekExecutor {
    pub schema: Arc<Schema>,
    pub label_id: LabelId,
    pub property_key_ids: Vec<PropertyKeyId>,
    /// Serialized property values for lookup
    pub property_values: Vec<Vec<u8>>,
}

impl NodeIndexSeekExecutor {
    pub fn new(
        schema: Arc<Schema>,
        label_id: LabelId,
        property_key_ids: Vec<PropertyKeyId>,
        property_values: Vec<Vec<u8>>,
    ) -> Self {
        Self {
            schema,
            label_id,
            property_key_ids,
            property_values,
        }
    }
}

impl Executor for NodeIndexSeekExecutor {
    fn build_stream(self: Box<Self>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        let stream = try_stream! {
            let tx = ctx.tx();
            let prop_value_refs: Vec<&[u8]> = self.property_values.iter().map(|v| v.as_slice()).collect();

            // Perform single index lookup
            let node_id = tx.get_unique_index(
                self.label_id,
                &self.property_key_ids,
                &prop_value_refs,
            )?;

            if let Some(node_id) = node_id {
                // Found a node - create a single-row result with VirtualNodeArray
                let mut builder = VirtualNodeArrayBuilder::with_capacity(1);
                builder.push(Some(node_id));
                let node_array = builder.finish();

                let chunk = DataChunk::new(
                    vec![Arc::new(ArrayImpl::VirtualNode(node_array))],
                    BitVec::repeat(true, 1),
                );
                yield chunk;
            }
            // If no node found, yield nothing (empty result)
        }
        .boxed();
        Ok(stream)
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }

    fn name(&self) -> &'static str {
        "NodeIndexSeek"
    }
}
