// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Wallet {
    address public owner;
    uint256 private pool_balance;
    mapping (address => uint256) public user_balances;

    event Tranfer(uint256 _amount, address _from, address _to);
    event Deposit(uint256 _amount, address _user);
    event Withdraw(uint256 _amount, address _user);

    constructor(address _owner) {
        owner = _owner;
    }

    modifier onlyOwner() {
        require(msg.sender == owner);
        _;
    }

    modifier hasMoney(uint256 _amount) {
        require(user_balances[msg.sender] > 0, "Wallet must have money");
        require(_amount <= user_balances[msg.sender], "Amount must not be greater than balance");
        _;
    }

    function deposit() public payable {
        require(msg.value > 0);
        user_balances[msg.sender] += msg.value;
        pool_balance += msg.value;
        emit Deposit(msg.value, msg.sender);
    }

    function withdraw(uint256 _amount) public payable hasMoney(_amount) {
        user_balances[msg.sender] -= _amount;
        pool_balance -= msg.value;
        (bool success, ) = msg.sender.call{ value: _amount }("");
        require(success, "Withdrawal failed");
        emit Withdraw(_amount, msg.sender);
    }

    function transfer(uint256 _amount, address _to) public hasMoney(_amount) {
        user_balances[msg.sender] -= _amount;
        user_balances[_to] += _amount;
    }

    function getBalance() public view returns (uint256) {
        return user_balances[msg.sender];
    }

    receive() external payable {}
    
    fallback() external {
        revert("Function does not exist");
    }

    function getPoolBalance() public onlyOwner view returns (uint256) {
        return pool_balance;
    }
}
