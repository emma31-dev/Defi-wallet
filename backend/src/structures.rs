use turso::Connection;
use tokio::sync::mpsc::Sender;
use alloy::rpc::types::Log;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: Connection,
    pub event_sender: Sender<Log>,
}
