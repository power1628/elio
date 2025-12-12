use async_stream::try_stream;
use futures::StreamExt;
use mojito_common::array::chunk::DataChunkBuilder;
use mojito_common::array::datum::{RelValueRef, ScalarRef, StructValue};
use mojito_common::{SemanticDirection, TokenId, TokenKind};
use mojito_storage::codec::RelFormat;

use super::*;

// Given from which column to expand
// generate rel and to columns at the end of input schema
// direct_output = input + rel + to
#[derive(Debug)]
pub struct ExpandAllExecutor {
    pub input: BoxedExecutor,
    pub from: usize,
    pub dir: SemanticDirection,
    // optimizer will remove invalid tokens and empty tokens
    pub rtype: Vec<TokenId>,
    pub schema: Arc<Schema>,
}
const CHUNK_SIZE: usize = 4096;

/// ExpandState. Two Loops
///  for row in OUTER
///     for rel in INNER(row[from])
///        construct row = input_row + rel + to
///        append row to output chunk
///        if output chunk full, then yield output chunk
impl Executor for ExpandAllExecutor {
    fn build_stream(self: Box<Self>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        let stream = try_stream! {
            let input_stream = self.input.build_stream(ctx.clone())?;
            let mut out_builder = DataChunkBuilder::new(self.schema.columns().iter().map(|col| col.typ.physical_type()), CHUNK_SIZE);
            for await chunk in input_stream {
                let outer = chunk?;
                for row in outer.iter(){
                    // if from is null, then remove this row
                    let from_id = match row[self.from]{
                        None => continue,
                        Some(id) => id.get_node_id().unwrap(),
                    };
                    let rel_iter = ctx.tx().rel_iter_for_node(from_id, self.dir, &self.rtype)?;
                    for rel_kv in rel_iter {
                        let (from_id, _rel_dir, token_id, to_id, rel_id, value) = rel_kv?;
                        let mut row = row.clone();
                        // add rel to row
                        let reltype = ctx.store().token_store().get_token_val(token_id, TokenKind::RelationshipType).unwrap();
                        // TODO(pgao): lazy deserialize
                        let prop_map = RelFormat::decode_value(&value);
                        // TODO(pgao): avoid clone
                        let struct_value = {
                            let mut fileds = vec![];
                            for entry in prop_map.iter() {
                                let key = ctx.store().token_store().get_token_val(entry.key(), TokenKind::PropertyKey)?;
                                // TODO(pgao): avoid clone
                                fileds.push((key, entry.value().to_owned_scalar()));
                            }
                            StructValue::new(fileds)
                        };

                        let rel_ref = RelValueRef{
                            id: rel_id,
                            reltype: &reltype,
                            start_id: from_id,
                            end_id: to_id,
                            props: struct_value.as_scalar_ref(),
                        };
                        row.push(Some(ScalarRef::Rel(rel_ref)));

                        // add to node to row
                        row.push(Some(ScalarRef::VirtualNode(to_id)));

                        // add to output
                        if let Some(chunk) = out_builder.append_row(row) {
                            yield chunk;
                        }
                    }
                }

                // flush out builder
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

#[derive(Debug)]

pub struct ExpandIntoExecutor {
    input: BoxedExecutor,

    schema: Arc<Schema>,
}

impl Executor for ExpandIntoExecutor {
    fn build_stream(self: Box<Self>, _ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        todo!()
    }

    fn schema(&self) -> &Schema {
        todo!()
    }
}
