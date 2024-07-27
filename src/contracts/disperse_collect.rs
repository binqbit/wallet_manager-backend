use std::fs::read_to_string;

use ethers::{
    abi::Abi,
    contract::Contract,
    providers::{Http, Provider}, 
    types::{Address, TransactionRequest, U256},
};

use super::provider::EthProvider;

pub struct DisperseCollect {
    pub provider: EthProvider,
    pub contract: Contract<Provider<Http>>,
}

impl DisperseCollect {
    pub fn new(rpc_url: &str, contract_address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = EthProvider::new(rpc_url)?;
        let address = contract_address.parse::<Address>()?;
        let abi = Abi::load(read_to_string("./config/abi/disperse_collect.json")?.as_bytes())?;
        let contract = Contract::new(address, abi, provider.provider.clone());
        Ok(Self {
            provider,
            contract,
        })
    }

    pub fn create_disperse_ether_tx(&self, sender: Address, recipients: Vec<Address>, values: Vec<U256>, value: U256) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
        let data = self.contract.encode("disperseEther", (recipients, values))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data)
            .value(value)
            .from(sender);
        Ok(tx)
    }

    pub fn create_disperse_ether_by_percent_tx(&self, sender: Address, recipients: Vec<Address>, percentages: Vec<U256>, value: U256) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
        let data = self.contract.encode("disperseEtherByPercent", (recipients, percentages))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data)
            .value(value)
            .from(sender);
        Ok(tx)
    }

    pub fn create_disperse_token_tx(&self, sender: Address, token: Address, recipients: Vec<Address>, values: Vec<U256>) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
        let data = self.contract.encode("disperseToken", (token, recipients, values))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data)
            .from(sender);
        Ok(tx)
    }

    pub fn create_disperse_token_by_percent_tx(&self, sender: Address, token: Address, recipients: Vec<Address>, percentages: Vec<U256>) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
        let data = self.contract.encode("disperseTokenByPercent", (token, recipients, percentages))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data)
            .from(sender);
        Ok(tx)
    }

    pub fn create_collect_ether_tx(&self, sender: Address, recipient: Address, value: U256) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
        let data = self.contract.encode("collectEther", (recipient,))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data)
            .value(value)
            .from(sender);
        Ok(tx)
    }

    pub fn create_collect_token_tx(&self, sender: Address, token: Address, recipient: Address, contributors: Vec<Address>, values: Vec<U256>) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
        let data = self.contract.encode("collectToken", (token, recipient, contributors, values))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data)
            .from(sender);
        Ok(tx)
    }
}
