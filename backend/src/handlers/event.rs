use std::str::FromStr;

use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    sol,
};
use anyhow::{Context, Result};
use futures::StreamExt;
use tracing::info;

use crate::structures::EnvVariables;

sol!(
    #[sol(rpc)] // <-- Important! Generates the necessary `MyContract` struct and function methods.
    #[sol(bytecode = "0x1234")]
    contract Wallet {
        event Transfer(uint256 _amount, address _from, address _to);
        event Deposit(uint256 _amount, address _user);
        event Withdraw(uint256 _amount, address _user);
    }
);

pub async fn deposit_listener(config: &EnvVariables) -> Result<()> {
    let ws_url = &config.rpc_endpoint;
    let provider = ProviderBuilder::new()
        .connect(ws_url)
        .await
        .context("Failed to connect to provider")?;

    let address = Address::from_str(&config.contract_add).context("Failed to serialize address")?;
    let contract = Wallet::new(address, &provider);
    // 2. Subscribe to Transfer events (or any custom event)
    let filter = contract.event_filter::<Wallet::Deposit>().filter;
    info!("Has topics: {}", filter.has_topics());
    let subscription = provider.watch_logs(&filter).await?;
    info!("Subscribed to Deposit events");
    let mut stream = subscription.into_stream().flat_map(futures::stream::iter);

    // Optional: fetch past deposits
    if let Ok(past) = provider.get_logs(&filter).await {
        info!("Found {} past deposits", past.len());
        for log in past {
            info!("Past deposit data: {}", log.inner.address);
        }
    }

    // 3. Loop forever, processing events
    while let Some(log) = stream.next().await {
        info!("New deposit detected {:?}", log.inner.data);
    }

    Ok(())
}

pub async fn withdraw_listener(config: &EnvVariables) -> Result<()> {
    let ws_url = &config.rpc_endpoint;
    let provider = ProviderBuilder::new()
        .connect(ws_url)
        .await
        .context("Failed to connect to provider")?;

    let address = Address::from_str(&config.contract_add).context("Failed to serialize address")?;
    let contract = Wallet::new(address, &provider);
    // 2. Subscribe to Transfer events (or any custom event)
    let filter = contract.Deposit_filter().filter;
    let subscription = provider.watch_logs(&filter).await?;
    let mut stream = subscription.into_stream().flat_map(futures::stream::iter);

    // 3. Loop forever, processing events
    while let Some(log) = stream.next().await {
        info!("New deposit detected {:?}", log.inner.data);
    }

    Ok(())
}

pub async fn transfer_listener(config: &EnvVariables) -> Result<()> {
    let ws_url = &config.rpc_endpoint;
    let provider = ProviderBuilder::new()
        .connect(ws_url)
        .await
        .context("Failed to connect to provider")?;

    let address = Address::from_str(&config.contract_add).context("Failed to serialize address")?;
    let contract = Wallet::new(address, &provider);
    // 2. Subscribe to Transfer events (or any custom event)
    let filter = contract.Deposit_filter().filter;
    let subscription = provider.watch_logs(&filter).await?;
    let mut stream = subscription
        .into_stream()
        .flat_map(futures::stream::iter)
        .take(5);

    // 3. Loop forever, processing events
    while let Some(log) = stream.next().await {
        info!("New deposit detected {:?}", log.inner.data);
    }

    Ok(())
}
