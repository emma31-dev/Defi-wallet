use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    sol,
};
use anyhow::{Context, Result};
use futures::StreamExt;
use tracing::info;

sol!(
    #[sol(rpc)] // <-- Important! Generates the necessary `MyContract` struct and function methods.
    #[sol(bytecode = "0x1234")]
    contract Wallet {
        address public owner;
        mapping (address => uint256) private pool_balances;
        mapping (address => mapping (address => uint256)) public user_balances;

        event Transfer(address token, uint256 _amount, address _from, address _to);
        event Deposit(address token, uint256 _amount, address _user);
        event Withdraw(address token, uint256 _amount, address _user);

        constructor(address _owner) {
            owner = _owner;
        }

        modifier onlyOwner() {
            require(msg.sender == owner);
            _;
        }

        modifier hasMoney(address _token, uint256 _amount) {
            require(user_balances[msg.sender][_token] > 0, "Wallet must have money");
            require(_amount <= user_balances[msg.sender][_token], "Amount must not be greater than balance");
            _;
        }

        function deposit(address _token) public payable {
            require(msg.value > 0);
            user_balances[msg.sender][_token] += msg.value;
            pool_balances[_token] += msg.value;
            emit Deposit(_token, msg.value, msg.sender);
        }

        function withdraw(address _token, uint256 _amount) public payable hasMoney(_token, _amount) {
            user_balances[msg.sender][_token] -= _amount;
            pool_balances[_token] -= _amount;
            (bool success, ) = msg.sender.call{ value: _amount }("");
            require(success, "Withdrawal failed");
            emit Withdraw(_token, _amount, msg.sender);
        }

        function transfer(address _token, uint256 _amount, address _to) public hasMoney(_token, _amount) {
            user_balances[msg.sender][_token] -= _amount;
            user_balances[_to][_token] += _amount;
        }

        function getBalance(address _token) public view returns (uint256) {
            return user_balances[msg.sender][_token];
        }

        receive() external payable {}

        fallback() external {
            revert("Function does not exist");
        }

        function getPoolBalance(address token) public onlyOwner view returns (uint256) {
            return pool_balances[token];
        }
    }
);

pub async fn listen() -> Result<()> {
    let ws_url = "ws";
    let provider = ProviderBuilder::new().connect(ws_url).await.context("Failed to connect to provider")?;

    let address = Address::ZERO;
    let contract = Wallet::new(address, &provider);
    // 2. Subscribe to Transfer events (or any custom event)
    let filter = &contract.Deposit_filter().filter;
    let subscription = provider.watch_logs(filter).await?;
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
