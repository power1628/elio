use std::collections::{VecDeque};

use async_stream::try_stream;
use futures::StreamExt;
use mojito_common::array::{ArrayImpl, DataChunkBuilder, RelArrayBuilder};
use mojito_common::array::datum::{ListValueRef, RelValue, ScalarRef, StructValue};
use mojito_common::store_types::RelDirection;
use mojito_common::{NodeId, SemanticDirection, TokenId, TokenKind};
use mojito_expr::impl_::BoxedExpression;
use mojito_storage::codec::RelFormat;

use super::*;

// output direction schema:
#[derive(Debug)]
pub struct VarExpandExecutor {
    pub input: BoxedExecutor,
    pub from: usize,
    pub to: Option<usize>,      // Some for ExpandInto
    pub dir: SemanticDirection, // expansion direction
    pub rel_types: Arc<[TokenId]>,
    pub len_min: usize,
    pub len_max: usize, 
    pub node_filter: Option<BoxedExpression>,
    pub rel_filter: Option<BoxedExpression>,
    pub schema: Arc<Schema>,
}

impl Executor for VarExpandExecutor {
    fn build_stream(self: Box<Self>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        let stream = try_stream! {

            let input_stream = self.input.build_stream(ctx.clone())?;
            let mut out_builder = DataChunkBuilder::new(self.schema.columns().iter().map(|col| col.typ.physical_type()), CHUNK_SIZE);
 
            for await chunk in input_stream{
                let outer = chunk?;
                for row in outer.iter() {
                    // if from is null, then remove this row
                    let from_id = match row[self.from].and_then(|id| id.get_node_id()){
                        Some(id) => id,
                        None => continue, // if from is null, then skip this row
                    };
                    let path_iter = VarExpandIter {
                        stack: VecDeque::from([(from_id, vec![])]),
                        ctx: ctx.clone(),
                        dir: self.dir,
                        rel_types: self.rel_types.clone(),
                        min_len: self.len_min,
                        max_len: self.len_max,
                    };

                    for item in path_iter {
                        let (_node, path) = item?;
                        // add node, path to output
                        // if output is full, emit the chunk
                        let mut row = row.clone();
                        // TODO(pgao): expand into
                        // push path rels list
                        let mut rel_array = RelArrayBuilder::with_capacity(path.len());
                        path.iter().for_each(|rel|
                            rel_array.push(Some(rel.as_scalar_ref()))
                        );
                        let rel_array: ArrayImpl= rel_array.finish().into();
                        row.push(Some(ScalarRef::List(ListValueRef::from_array(&rel_array, 0, rel_array.len()))));

                        // push last node
                        let to_node= path.last().map(|rel| ScalarRef::VirtualNode(rel.end_id));
                        row.push(to_node);

                        if let Some(chunk) = out_builder.append_row(row) {
                            yield chunk;
                        }
                    }
                }

                if let Some(chunk) = out_builder.yield_chunk() {
                    yield chunk;
                }
            }

        }
        .boxed();

        Ok(stream)
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }
}

/// for row in INPUT
///   let from = row[from];
///   let path_iter = VarExpandIter {
///       stack: vec_deque![(from, vec![])],
///       ctx,
///       dir: self.dir,
///       rel_types: self.rel_types.clone(),
///   };
///   for (node, path) in path_iter {
///       emit (node, path);
///   }

pub struct VarExpandIter {
    pub stack: VecDeque<(NodeId, Vec<RelValue>)>,
    pub ctx: Arc<TaskExecContext>,
    pub dir: SemanticDirection,
    pub rel_types: Arc<[TokenId]>,
    pub min_len: usize,
    pub max_len: usize, 
}

impl Iterator for VarExpandIter {
    type Item = Result<(NodeId, Vec<RelValue>), ExecError>;

    fn next(&mut self) -> Option<Self::Item> {
        // pop from stack
        let (node, path) = self.stack.pop_front()?;
        if path.len() < self.max_len {
            // expand node and path
            let rel_iter = match self.ctx.tx().rel_iter_for_node(node, self.dir, &self.rel_types) {
                Ok(rel_iter) => rel_iter,
                Err(e) => return Some(Err(e.into())),
            };

            // add expanded path to stack
            // for each rel, add (target, path + rel) to stack
            for rel_kv in rel_iter {
                let mut expanded_path = path.clone();
                let rel_kv = match rel_kv {
                    Ok(kv) => kv,
                    Err(e) => return Some(Err(e.into())),
                };
                let (from_id, rel_dir, token_id, to_id, rel_id, value) = rel_kv;
                // TODO(pgao): avoid get token value for each rel
                // maybe we can cache all the token value on the execution context
                let rel_type = match self.ctx.catalog().get_token_val(token_id, TokenKind::RelationshipType) {
                    Ok(rel_type) => rel_type,
                    Err(e) => return Some(Err(e.into())),
                };

                // TODO(pgao): we can know the start id and end id at planning time
                let (start_id, end_id) = match rel_dir {
                    RelDirection::Out => (from_id, to_id),
                    RelDirection::In => (to_id, from_id),
                };

                // TODO(pgao): lazy deserialize
                let prop_map = RelFormat::decode_value(&value);
                // TODO(pgao): avoid clone
                let struct_value = {
                    let mut fileds = vec![];
                    for entry in prop_map.iter() {
                        let key = match self
                            .ctx
                            .store()
                            .token_store()
                            .get_token_val(entry.key(), TokenKind::PropertyKey)
                        {
                            Ok(key) => key,
                            Err(e) => return Some(Err(e.into())),
                        };
                        // TODO(pgao): avoid clone
                        fileds.push((key, entry.value().to_owned_scalar()));
                    }
                    StructValue::new(fileds)
                };

                // TODO(pgao): expand into and filter etc


                expanded_path.push(RelValue {
                    id: rel_id,
                    reltype: rel_type.to_string(),
                    start_id,
                    end_id,
                    props: struct_value,
                });
                self.stack.push_back((end_id, expanded_path));
            }
        }
        // emit (node, path)
        Some(Ok((node, path)))
    }
}
