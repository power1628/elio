use mojito_common::array::ArrayImpl;
use mojito_common::array::chunk::DataChunk;

use crate::cf_property;
use crate::error::GraphStoreError;
use crate::transaction::{DataChunkIterator, NodeScanOptions, OwnedTransaction, TxRead};

// node create id
// node -> encoding -> rocksdb write batch
pub(crate) fn batch_node_create(tx: &OwnedTransaction, chunk: &DataChunk) -> Result<ArrayImpl, GraphStoreError> {
    // allocate node id for the batch

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
    opts: &NodeScanOptions,
) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
    let mut iter = tx.full_iter();
    todo!()
}
