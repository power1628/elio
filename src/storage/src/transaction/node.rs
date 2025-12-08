use mojito_common::LabelId;
use mojito_common::array::chunk::DataChunk;
use mojito_common::array::datum::{StructValue, StructValueRef};
use mojito_common::array::{NodeArray, StructArray, VirtualNodeArray, VirtualNodeArrayBuilder};

use crate::cf_property;
use crate::codec::NodeFormat;
use crate::error::GraphStoreError;
use crate::transaction::{DataChunkIterator, NodeScanOptions, TransactionImpl};

// expected input columns
// label: Vec<LabelId> | ListArray<u16>
// properties: Vec<(PropertyKeyId, PropertyValue)> | AnyMapArray
// node -> encoding -> rocksdb write batch
pub(crate) fn batch_node_create(
    tx: &TransactionImpl,
    labels: &[LabelId],
    props: &StructArray,
) -> Result<NodeArray, GraphStoreError> {
    assert_eq!(labels.len(), props.len());
    let len = labels.len();

    // allocate node id for the batch
    let node_ids = tx.dict.batch_node_id(len)?;

    // create node fields for the batch

    let mut keys = Vec::with_capacity(len);
    let mut values = Vec::with_capacity(len);

    for (node_id, prop) in node_ids.iter().zip(props.iter()) {
        let key = NodeFormat::encode_node_key(*node_id);
        keys.push(key);
        // labels and props must not be null
        let prop = prop.unwrap();
        let value = NodeFormat::encode_node_value(labels, prop);
        values.push(value);
    }

    // construct batch
    let cf = tx.inner._db.cf_handle(cf_property::CF_NAME).unwrap();
    let mut guard = tx.write_state.lock().unwrap();
    for (k, v) in keys.iter().zip(values.iter()) {
        guard.batch.put_cf(&cf, k, v);
    }
    drop(guard);

    // create node ids array
    let mut ids = VirtualNodeArrayBuilder::with_capacity(len);
    for id in node_ids.iter() {
        ids.append(Some(*id));
    }
    let ids = ids.finish();
    Ok(ids)
}

pub(crate) fn batch_node_scan(
    tx: &TransactionImpl,
    opts: NodeScanOptions,
) -> Result<Box<dyn DataChunkIterator + '_>, GraphStoreError> {
    let cf_handle = tx.inner._db.cf_handle(cf_property::CF_NAME).unwrap();
    let mut readopts = rocksdb::ReadOptions::default();
    // TODO(pgao): check the behavior of prefix scan
    readopts.set_prefix_same_as_start(true);
    let mode = rocksdb::IteratorMode::From(cf_property::NODE_KEY_PREFIX, rocksdb::Direction::Forward);
    let iter = tx.inner.snapshot.iterator_cf_opt(&cf_handle, readopts, mode);
    Ok(Box::new(NodeIterator { iter, opts }))
}

pub struct NodeIterator<'a, D: rocksdb::DBAccess> {
    iter: rocksdb::DBIteratorWithThreadMode<'a, D>,
    opts: NodeScanOptions,
}

impl<'a, D: rocksdb::DBAccess> DataChunkIterator for NodeIterator<'a, D> {
    fn next_batch(&mut self) -> Result<Option<DataChunk>, GraphStoreError> {
        let mut builder = VirtualNodeArrayBuilder::with_capacity(self.opts.batch_size);
        for _ in 0..self.opts.batch_size {
            if let Some(item) = self.iter.next() {
                let (key, _val) = item.map_err(GraphStoreError::Rocksdb)?;
                if !key.starts_with(cf_property::NODE_KEY_PREFIX) {
                    break;
                }
                let node_id = NodeFormat::decode_node_key(&key);
                builder.append(Some(node_id));
            } else {
                break;
            }
        }

        let array = builder.finish();
        if array.is_empty() {
            Ok(None)
        } else {
            let chunk = DataChunk::new(vec![array.into()]);
            Ok(Some(chunk))
        }
    }
}
