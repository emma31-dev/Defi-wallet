use futures::StreamExt;
use tokio::sync::mpsc;
use alloy::{providers::{Provider, ProviderBuilder}, rpc::types::{Filter, Log}, sol, sol_types::SolEvent};
use anyhow::Result;

use crate::handlers::event::IERC20::Transfer;

sol!(
    interface IERC20 {
        event Transfer(address indexed from, address indexed to, uint256 value);
    }
);

pub async fn listen(mut rx: mpsc::Receiver<Log>) -> Result<()> {
    let ws_url = "wss://eth-mainnet.g.alchemy.com/v2/YOUR_API_KEY";
    // 1. Connect to the node via WebSocket
    let provider = ProviderBuilder::new().connect(ws_url).await?;

    // 2. Subscribe to Transfer events (or any custom event)
    let filter = Filter::new().event(&Transfer::SIGNATURE);
    let subscription = provider.watch_logs(&filter).await?;
    let mut stream = subscription.into_stream().map(futures::stream::iter).take(5);

    // 3. Loop forever, processing events
    while let Some(log) = stream.next().await {
        println!("{:?}", log);
    }

    Ok(())
}