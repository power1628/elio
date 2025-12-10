use bitvec::vec::BitVec;
use mojito_common::array::datum::{RelValueRef, StructValue};
use mojito_common::array::{Array, NodeArray, RelArray, RelArrayBuilder, StructArray, VirtualNodeArray};
use mojito_common::store_types::RelDirection;
use mojito_common::{NodeId, TokenKind};

use crate::cf_topology;
use crate::codec::RelFormat;
use crate::error::GraphStoreError;
use crate::transaction::TransactionImpl;

/// start/end are expected to be :
///   - VirtualNodeArray
///   - NodeArray
/// prop expected to be:
///   - StructArray
///
/// 1. mapping rtype to TokenId
/// 2. mapping property key ids to TokenId
/// 3. create relationship id
/// 3. serialize key-value
/// 4. batch write
pub(crate) fn batch_rel_create<A, B>(
    tx: &TransactionImpl,
    rtype: &str,
    start: &A,
    end: &B,
    prop: &StructArray,
) -> Result<RelArray, GraphStoreError>
where
    A: NodeIdContainer,
    B: NodeIdContainer,
{
    // check start and end should not be null
    if !start.valid_map().all() || !end.valid_map().all() {
        return Err(GraphStoreError::internal("start/end should not be null".to_string()));
    }

    assert_eq!(start.len(), end.len());
    assert_eq!(prop.len(), start.len());

    let rtype_id = tx.token.get_or_create_token(rtype, TokenKind::RelationshipType)?;
    let rel_ids = tx.dict.batch_rel_id(start.len())?;
    let prop_key_ids = prop
        .field_names()
        .map(|k| tx.token.get_or_create_token(k, TokenKind::PropertyKey))
        .collect::<Result<Vec<_>, _>>()?;
    let len = start.len();

    let mut out_keys = Vec::with_capacity(len);
    let mut in_keys = Vec::with_capacity(len);
    let mut values = Vec::with_capacity(len);
    let empty_prop = StructValue::default();
    let empty_prop_ref = empty_prop.as_scalar_ref();
    for i in 0..len {
        let rel_id = rel_ids[i];
        let start_id = start.get_unchecked(i);
        let end_id = end.get_unchecked(i);
        let prop = prop.get(i);

        let out_key = RelFormat::encode_key(start_id, RelDirection::Out, rtype_id, end_id, rel_id);
        let in_key = RelFormat::encode_key(end_id, RelDirection::In, rtype_id, start_id, rel_id);
        out_keys.push(out_key);
        in_keys.push(in_key);

        let value = RelFormat::encode_value(&prop_key_ids, prop.unwrap_or(empty_prop_ref))
            .map_err(|e| GraphStoreError::internal(e.to_string()))?;
        values.push(value);
    }

    // construct batch
    let cf = tx.inner._db.cf_handle(cf_topology::CF_NAME).unwrap();
    let mut guard = tx.write_state.lock().unwrap();
    for (k, v) in out_keys.iter().chain(in_keys.iter()).zip(values.iter()) {
        guard.batch.put_cf(&cf, k, v);
    }
    for (k, v) in out_keys.iter().chain(in_keys.iter()).zip(values.iter()) {
        guard.batch.put_cf(&cf, k, v);
    }
    drop(guard);

    // create rel array
    let mut builder = RelArrayBuilder::with_capacity(len);

    for (i, rel_id) in rel_ids.iter().enumerate() {
        let rel_ref = RelValueRef {
            id: *rel_id,
            reltype: rtype,
            start_id: start.get_unchecked(i),
            end_id: end.get_unchecked(i),
            props: prop.get(i).unwrap_or(empty_prop_ref),
        };
        builder.push(Some(rel_ref));
    }

    Ok(builder.finish())
}

pub trait NodeIdContainer: Sized {
    fn len(&self) -> usize;
    fn get_unchecked(&self, index: usize) -> NodeId;
    fn valid_map(&self) -> &BitVec;
}

impl NodeIdContainer for NodeArray {
    fn len(&self) -> usize {
        <Self as Array>::len(self)
    }

    fn get_unchecked(&self, index: usize) -> NodeId {
        // TODO(pgao): optimize
        <Self as Array>::get(self, index).unwrap().id
    }

    fn valid_map(&self) -> &BitVec {
        self.valid_map()
    }
}

impl NodeIdContainer for VirtualNodeArray {
    fn len(&self) -> usize {
        <Self as Array>::len(self)
    }

    fn get_unchecked(&self, index: usize) -> NodeId {
        // TODO(pgao): optimize
        <Self as Array>::get(self, index).unwrap()
    }

    fn valid_map(&self) -> &BitVec {
        self.valid_map()
    }
}
