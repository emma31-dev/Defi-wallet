use alloy::{primitives::Address, sol_types::sol_data::Uint};
use turso::Connection;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: Connection,
    // pub event_sender: Sender<Log>,
}

pub struct LogData {
    pub token: Address,
    pub from: Address,
    pub to: Address,
    pub amount: Uint<256>,
}
