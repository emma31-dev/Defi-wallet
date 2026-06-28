use alloy::{
    primitives::{Address, utils::format_ether},
    providers::{Provider, ProviderBuilder},
    sol,
    sol_types::SolEvent,
};
use anyhow::{Context, Result};
use futures::StreamExt;
use std::str::FromStr;
use tracing::{error, info, warn};

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
    let rpc_url = &config.rpc_endpoint;
    let provider = ProviderBuilder::new()
        .connect(rpc_url)
        .await
        .context("Failed to connect to rpc provider")?;

    let address = Address::from_str(&config.contract_add)
        .context("Failed to serialize address for environmental variables")?;
    let contract = Wallet::new(address, &provider);
    // Subscribe to Transfer events (or any custom event)
    let filter = contract.event_filter::<Wallet::Deposit>().filter;
    let subscription = provider
        .watch_logs(&filter)
        .await
        .context("Failed to subscribe to event")?;
    let mut stream = subscription.into_stream().flat_map(futures::stream::iter);

    // Optional: fetch past deposits
    // if let Ok(past) = provider.get_logs(&filter).await {
    //     info!("Found {} past deposits", past.len());
    //     for log in past {
    //         if let Ok(data) = Wallet::Deposit::decode_log(&log.inner) {
    //             let amount = format_ether(data._amount);
    //             debug!("Past Deposit detected. Address: {}; Amount: {}ETH", data._user, amount);
    //         };
    //     }
    // }

    // Loop forever, processing events
    while let Some(log) = stream.next().await {
        match Wallet::Deposit::decode_log(&log.inner) {
            Ok(data) => {
                let amount = format_ether(data._amount);
                info!(
                    "New deposit detected. Address: {}; Amount: {}ETH",
                    data._user, amount
                );
            }
            Err(e) => {
                warn!("Error decoding deposit log: {:?}", e);
            }
        };
    }

    Ok(())
}

pub async fn withdraw_listener(config: &EnvVariables) -> Result<()> {
    let rpc_url = &config.rpc_endpoint;
    let provider = ProviderBuilder::new()
        .connect(rpc_url)
        .await
        .context("Failed to connect to provider")?;

    let address = Address::from_str(&config.contract_add).context("Failed to serialize address")?;
    let contract = Wallet::new(address, &provider);
    // 2. Subscribe to Transfer events (or any custom event)
    let filter = contract.event_filter::<Wallet::Withdraw>().filter;
    let subscription = provider.watch_logs(&filter).await?;
    let mut stream = subscription.into_stream().flat_map(futures::stream::iter);

    // Optional: fetch past deposits
    // if let Ok(past) = provider.get_logs(&filter).await {
    //     info!("Found {} past deposits", past.len());
    //     for log in past {
    //         if let Ok(data) = Wallet::Deposit::decode_log(&log.inner) {
    //             let amount = format_ether(data._amount);
    //             info!("New deposit detected. Address: {}; Amount: {}ETH", data._user, amount);
    //         };
    //     }
    // }

    // 3. Loop forever, processing events
    while let Some(log) = stream.next().await {
        match Wallet::Withdraw::decode_log(&log.inner) {
            Ok(data) => {
                let amount = format_ether(data._amount);
                info!(
                    "New Withdrawal detected. Address: {}; Amount: {}ETH",
                    data._user, amount
                );
            }
            Err(e) => {
                error!("Error decoding withdrawal log: {:?}", e);
            }
        };
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
    let filter = contract.event_filter::<Wallet::Transfer>().filter;
    let subscription = provider.watch_logs(&filter).await?;
    let mut stream = subscription.into_stream().flat_map(futures::stream::iter);

    // Optional: fetch past deposits
    // if let Ok(past) = provider.get_logs(&filter).await {
    //     info!("Found {} past transfers", past.len());
    //     for log in past {
    //         if let Ok(data) = Wallet::Deposit::decode_log(&log.inner) {
    //             let amount = format_ether(data._amount);
    //             info!("New deposit detected. Address: {}; Amount: {}ETH", data._user, amount);
    //         };
    //     }
    // }

    // 3. Loop forever, processing events
    while let Some(log) = stream.next().await {
        match Wallet::Transfer::decode_log(&log.inner) {
            Ok(data) => {
                let amount = format_ether(data._amount);
                info!(
                    "New transfer detected. From: {}; To: {}; Amount: {}ETH",
                    data._from, data._to, amount
                );
            }
            Err(e) => {
                error!("Error decoding transfer log: {:?}", e);
            }
        };
    }

    Ok(())
}
