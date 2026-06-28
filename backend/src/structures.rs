// use alloy::{primitives::Address, sol_types::sol_data::Uint};
use turso::Connection;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: Connection,
    pub config: EnvVariables,
    // pub event_sender: Sender<Log>,
}

// pub struct LogData {
//     pub token: Address,
//     pub from: Address,
//     pub to: Address,
//     pub amount: Uint<256>,
// }

#[derive(Clone)]
pub struct EnvVariables {
    pub database_url: String,
    pub contract_add: String,
    pub rpc_endpoint: String,
    pub socket: String,
}

impl EnvVariables {
    pub fn new() -> Self {
        dotenvy::dotenv().expect("Failed to load .env variables");
        EnvVariables {
            database_url: std::env::var("DATABASE_URL").unwrap(),
            contract_add: std::env::var("CONTRACT_ADDRESS").unwrap(),
            rpc_endpoint: std::env::var("RPC_ENDPOINT").unwrap(),
            socket: std::env::var("SERVER_ADDRESS").unwrap(),
        }
    }
}
