use mojito_common::LabelId;
use mojito_common::array::prop_map::PropertyMapArray;
use mojito_common::array::{Array, ArrayBuilder, NodeIdArray, NodeIdArrayBuilder};

use crate::cf_property;
use crate::codec::NodeFormat;
use crate::error::GraphStoreError;
use crate::transaction::{DataChunkIterator, NodeScanOptions, TransactionImpl, TxRead};

// expected input columns
// label: Vec<LabelId> | ListArray<u16>
// properties: Vec<(PropertyKeyId, PropertyValue)> | AnyMapArray
// node -> encoding -> rocksdb write batch
pub(crate) fn batch_node_create(
    tx: &TransactionImpl,
    labels: &[LabelId],
    props: &PropertyMapArray,
) -> Result<NodeIdArray, GraphStoreError> {
    assert_eq!(labels.len(), props.len());
    let len = labels.len();

    // allocate node id for the batch
    let node_ids = tx.dict.batch_node_id(len)?;

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
    let mut ids = NodeIdArrayBuilder::with_capacity(len);
    for id in node_ids.iter() {
        ids.append(Some(*id));
    }
    let ids = ids.finish();
    Ok(ids)
}

pub(crate) fn batch_node_scan<T: TxRead>(
    tx: &T,
    _opts: &NodeScanOptions,
) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
    let _iter = tx.full_iter();
    todo!()
}
