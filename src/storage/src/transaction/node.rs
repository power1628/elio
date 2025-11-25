use mojito_common::array::ArrayImpl;

use crate::cf_property;
use crate::error::GraphStoreError;
use crate::transaction::{DataChunkIterator, NodeScanOptions, OwnedTransaction, RwTransaction, TxRead};

// expected input columns
// label: Vec<LabelId> | ListArray<u16>
// properties: Vec<(PropertyKeyId, PropertyValue)> | AnyMapArray
// node -> encoding -> rocksdb write batch
pub(crate) fn batch_node_create(
    tx: &RwTransaction,
    labels: &ArrayImpl,
    props: &ArrayImpl,
) -> Result<ArrayImpl, GraphStoreError> {
    assert_eq!(labels.len(), props.len());
    let len = labels.len();

    // allocate node id for the batch
    let node_ids = tx.dict.batch_node_id(len)?;

    // encode node to keys and values

    // construct batch
    let keys: Vec<Vec<u8>> = vec![];
    let vals: Vec<Vec<u8>> = vec![];
    let cf = tx._db.cf_handle(cf_property::CF_NAME).unwrap();
    for (k, v) in keys.iter().zip(vals.iter()) {
        tx.put_cf(&cf, k, v)?;
    }
    todo!()
}

pub(crate) fn batch_node_scan<T: TxRead>(
    tx: &T,
    _opts: &NodeScanOptions,
) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
    let _iter = tx.full_iter();
    todo!()
}
