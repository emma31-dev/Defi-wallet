// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;
import {Ownable} from "../lib/openzeppelin-contracts/contracts/access/Ownable.sol";

contract Wallet {
    address public owner;
    mapping (address => uint256) private pool_balances;
    mapping (address => mapping (address => uint256)) public user_balances;

    event Tranfer(address token, uint256 _amount, address _from, address _to);
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
