use async_stream::try_stream;
use elio_storage::transaction::NodeScanOptions;
use futures::StreamExt;
use tokio::sync::mpsc;

use super::*;
use crate::executor::Executor;

const CHANNEL_BUFFER_SIZE: usize = 128;

// TODO(pgao): support argument
// AllNodeScan is an leaf node and produces NodeId as results
#[derive(Debug)]
pub struct AllNodeScanExectuor {
    schema: Arc<Schema>,
}

impl AllNodeScanExectuor {
    pub fn new(schema: Arc<Schema>) -> Self {
        Self { schema }
    }
}

impl Executor for AllNodeScanExectuor {
    fn open(&self, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        let (tx, mut rx) = mpsc::channel::<Result<DataChunk, ExecError>>(CHANNEL_BUFFER_SIZE);
        let txn = ctx.tx().clone();
        // io task
        // TODO(pgao): io thread should be separated
        tokio::task::spawn_blocking(move || {
            let opts = NodeScanOptions { batch_size: 1024 };
            let mut iter = match txn.node_scan(opts) {
                Ok(iter) => iter,
                Err(e) => {
                    tracing::error!("node scan error: {:?}", e);
                    if tx.blocking_send(Err(e.into())).is_err() {
                        tracing::warn!("recv dropped, could not send scan error.");
                    }
                    return;
                }
            };
            loop {
                match iter.next_batch() {
                    Ok(Some(chunk)) => {
                        if tx.blocking_send(Ok(chunk)).is_err() {
                            break;
                        }
                    }
                    Ok(None) => {
                        break;
                    }
                    Err(e) => {
                        let _ = tx.blocking_send(Err(e.into()));
                        break;
                    }
                }
            }
        });

        let stream = try_stream! {
            while let Some(item) = rx.recv().await{
                yield item?;
            }
        }
        .boxed();
        Ok(stream)
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }

    fn name(&self) -> &'static str {
        "AllNodeScan"
    }
}
