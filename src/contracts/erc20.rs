use std::fs::read_to_string;

use ethers::{
    abi::Abi,
    contract::Contract,
    providers::{Http, Provider},
    types::{transaction::eip2718::TypedTransaction, Address, TransactionRequest, U256},
};

use crate::utils::web3::{parse_u256, u256_to_string};

use super::provider::EthProvider;

pub struct ERC20 {
    pub provider: EthProvider,
    pub contract: Contract<Provider<Http>>,
}

impl ERC20 {
    pub fn new(rpc_url: &str, contract_address: Address) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = EthProvider::new(rpc_url)?;
        let abi = Abi::load(read_to_string("./config/abi/erc20.json")?.as_bytes())?;
        let contract = Contract::new(contract_address, abi, provider.provider.clone());
        Ok(Self {
            provider,
            contract,
        })
    }

    pub fn create_transfer_tx(&self, sender: Address, recipient: Address, amount: U256) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
        let data = self.contract.encode("transfer", (recipient, amount))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data)
            .from(sender);
        Ok(tx)
    }

    pub fn create_approve_tx(&self, sender: Address, spender: Address, amount: U256) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
        let data = self.contract.encode("approve", (spender, amount))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data)
            .from(sender);
        Ok(tx)
    }

    pub fn create_transfer_from_tx(&self, sender: Address, from: Address, to: Address, amount: U256) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
        let data = self.contract.encode("transferFrom", (from, to, amount))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data)
            .from(sender);
        Ok(tx)
    }

    pub fn create_balance_of_tx(&self, owner: Address) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
        let data = self.contract.encode("balanceOf", (owner,))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data);
        Ok(tx)
    }

    pub fn create_allowance_tx(&self, owner: Address, spender: Address) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
        let data = self.contract.encode("allowance", (owner, spender))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data);
        Ok(tx)
    }

    pub fn create_total_supply_tx(&self) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
        let data = self.contract.encode("totalSupply", ())?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data);
        Ok(tx)
    }

    pub fn create_decimals_tx(&self) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
        let data = self.contract.encode("decimals", ())?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data);
        Ok(tx)
    }

    pub async fn query_balance_of(&self, owner: Address) -> Result<U256, Box<dyn std::error::Error>> {
        let tx: TransactionRequest = self.create_balance_of_tx(owner)?;
        let result = self.provider.query_transaction(TypedTransaction::Legacy(tx)).await?;
        Ok(U256::from_big_endian(&result))
    }

    pub async fn query_allowance(&self, owner: Address, spender: Address) -> Result<U256, Box<dyn std::error::Error>> {
        let tx: TransactionRequest = self.create_allowance_tx(owner, spender)?;
        let result = self.provider.query_transaction(TypedTransaction::Legacy(tx)).await?;
        Ok(U256::from_big_endian(&result))
    }

    pub async fn query_total_supply(&self) -> Result<U256, Box<dyn std::error::Error>> {
        let tx: TransactionRequest = self.create_total_supply_tx()?;
        let result = self.provider.query_transaction(TypedTransaction::Legacy(tx)).await?;
        Ok(U256::from_big_endian(&result))
    }

    pub async fn query_decimals(&self) -> Result<U256, Box<dyn std::error::Error>> {
        let tx: TransactionRequest = self.create_decimals_tx()?;
        let result = self.provider.query_transaction(TypedTransaction::Legacy(tx)).await?;
        Ok(U256::from_big_endian(&result))
    }

    pub async fn token_to_string(&self, amount: U256) -> Result<String, Box<dyn std::error::Error>> {
        let decimals = self.query_decimals().await?;
        Ok(u256_to_string(amount, decimals))
    }

    pub async fn parse_token(&self, amount: &str) -> Result<U256, Box<dyn std::error::Error>> {
        let decimals = self.query_decimals().await?;
        parse_u256(amount, decimals)
    }
}
