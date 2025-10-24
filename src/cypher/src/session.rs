use std::sync::Arc;

use mojito_catalog::Catalog;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
pub struct SessionContext {
    pub catalog: Arc<Catalog>,
    notification_tx: UnboundedSender<String>,
}
