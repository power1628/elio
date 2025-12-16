use std::sync::Arc;

use bitvec::vec::BitVec;
use itertools::Itertools;
use mojito_common::NodeId;
use mojito_common::array::datum::RelValueRef;
use mojito_common::array::{
    Array, ArrayImpl, ListArrayBuilder, RelArrayBuilder, VirtualNodeArrayBuilder, VirtualPathArray,
};

use super::*;

#[derive(Debug)]
pub struct ProjectPathExpr {
    // expected to be interleaved by node and rel
    pub inputs: Vec<BoxedExpression>,
    // output type, expected to be virtualpath
    pub typ: DataType,
}

impl Expression for ProjectPathExpr {
    fn typ(&self) -> &DataType {
        &self.typ
    }

    fn eval_batch(&self, chunk: &DataChunk, ctx: &dyn EvalCtx) -> Result<ArrayRef, EvalError> {
        let steps = self
            .inputs
            .iter()
            .map(|expr| expr.eval_batch(chunk, ctx))
            .collect::<Result<Vec<_>, _>>()?;

        let len = steps.first().unwrap().len();

        // push nodes
        let mut node_builder = ListArrayBuilder::new(Box::new(VirtualNodeArrayBuilder::with_capacity(len).into()));
        let mut rel_builder = ListArrayBuilder::new(Box::new(RelArrayBuilder::with_capacity(len).into()));

        let nodes = steps
            .iter()
            .filter(|step| matches!(&***step, ArrayImpl::VirtualNode(_) | ArrayImpl::Node(_)))
            .cloned()
            .collect_vec();

        let rels = steps
            .iter()
            .filter(|step| matches!(&***step, ArrayImpl::Rel(_)))
            .cloned()
            .collect_vec();

        fn get_nodes(node_steps: &[ArrayRef], rowid: usize) -> impl Iterator<Item = NodeId> + '_ {
            node_steps.iter().filter_map(move |step| {
                if let ArrayImpl::VirtualNode(virtual_node_array) = step.as_ref() {
                    virtual_node_array.get(rowid)
                } else if let ArrayImpl::Node(node_array) = step.as_ref() {
                    node_array.get(rowid).map(|x| x.id)
                } else {
                    None
                }
            })
        }

        fn get_rels(rel_steps: &[ArrayRef], rowid: usize) -> impl Iterator<Item = RelValueRef<'_>> + '_ {
            rel_steps.iter().filter_map(move |step| {
                if let ArrayImpl::Rel(rel_array) = step.as_ref() {
                    rel_array.get(rowid)
                } else {
                    None
                }
            })
        }

        let mut valid = BitVec::with_capacity(len);
        // build node list
        for rowid in 0..len {
            let nodes = get_nodes(&nodes, rowid);
            let rels = get_rels(&rels, rowid);
            node_builder.push_virtual_nodes(nodes);
            rel_builder.push_rels(rels);
            valid.push(true);
        }

        let path = VirtualPathArray::from_parts(Arc::new(node_builder.finish()), Arc::new(rel_builder.finish()), valid);

        Ok(Arc::new(path.into()))
    }
}
