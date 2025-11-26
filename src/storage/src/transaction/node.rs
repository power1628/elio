use mojito_common::array::list::ListArray;
use mojito_common::array::prop_map::PropertyMapArray;
use mojito_common::array::{Array, ArrayBuilder, NodeIdArray, NodeIdArrayBuilder};
use mojito_common::data_type::DataType;

use crate::cf_property;
use crate::codec::NodeFormat;
use crate::error::GraphStoreError;
use crate::transaction::{DataChunkIterator, NodeScanOptions, RwTransaction, TxRead};

// expected input columns
// label: Vec<LabelId> | ListArray<u16>
// properties: Vec<(PropertyKeyId, PropertyValue)> | AnyMapArray
// node -> encoding -> rocksdb write batch
pub(crate) fn batch_node_create(
    tx: &RwTransaction,
    labels: &ListArray,
    props: &PropertyMapArray,
) -> Result<NodeIdArray, GraphStoreError> {
    assert_eq!(labels.len(), props.len());
    let len = labels.len();

    // allocate node id for the batch
    let node_ids = tx.dict.batch_node_id(len)?;

    let mut keys = Vec::with_capacity(len);
    let mut values = Vec::with_capacity(len);

    for (node_id, (label, prop)) in node_ids.iter().zip(labels.iter().zip(props.iter())) {
        let key = NodeFormat::encode_node_key(*node_id);
        keys.push(key);
        // labels and props must not be null
        let label = label.unwrap();
        let label = label.as_u16_slice().unwrap();
        let prop = prop.unwrap();
        let value = NodeFormat::encode_node_value(label, prop);
        values.push(value);
    }

    // construct batch
    let cf = tx.inner._db.cf_handle(cf_property::CF_NAME).unwrap();
    for (k, v) in keys.iter().zip(values.iter()) {
        tx.inner.put_cf(&cf, k, v)?;
    }

    // create node ids array
    let mut ids = NodeIdArrayBuilder::with_capacity(len, DataType::NodeId);
    for id in node_ids.iter() {
        ids.push(Some(*id));
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
