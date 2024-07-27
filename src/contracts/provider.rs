use ethers::{
    core::k256::ecdsa::SigningKey, providers::{Http, Middleware, Provider}, signers::Wallet, types::{transaction::eip2718::TypedTransaction, Address, Bytes, Signature, TransactionReceipt, TransactionRequest, H256, U256}, utils::hex
};
use std::sync::Arc;
use std::convert::TryFrom;

pub struct EthProvider {
    pub provider: Arc<Provider<Http>>,
}

impl EthProvider {
    pub fn new(rpc_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        Ok(Self {
            provider: Arc::new(provider)
        })
    }

    pub fn create_wallet(private_key: &str) -> Result<Wallet<SigningKey>, Box<dyn std::error::Error>> {
        let wallet: Wallet<SigningKey> = private_key.parse()?;
        Ok(wallet)
    }

    pub async fn get_balance(&self, address: Address) -> Result<U256, Box<dyn std::error::Error>> {
        let balance = self.provider.get_balance(address, None).await?;
        Ok(balance)
    }

    pub async fn get_nonce(&self, address: Address) -> Result<U256, Box<dyn std::error::Error>> {
        let nonce = self.provider.get_transaction_count(address, None).await?;
        Ok(nonce)
    }

    pub async fn estimate_gas(&self, tx: &TransactionRequest) -> Result<U256, Box<dyn std::error::Error>> {
        let gas = self.provider.estimate_gas(&TypedTransaction::Legacy(tx.clone()), None).await?;
        Ok(gas)
    }

    pub async fn get_transaction_count(&self, address: Address) -> Result<U256, Box<dyn std::error::Error>> {
        let count = self.provider.get_transaction_count(address, None).await?;
        Ok(count)
    }

    pub async fn get_gas_price(&self) -> Result<U256, Box<dyn std::error::Error>> {
        let gas_price = self.provider.get_gas_price().await?;
        Ok(gas_price)
    }

    pub async fn get_transaction(&self, tx_hash: H256) -> Result<Option<TransactionReceipt>, Box<dyn std::error::Error>> {
        let receipt = self.provider.get_transaction_receipt(tx_hash).await?;
        Ok(receipt)
    }

    pub async fn query_transaction(&self, tx: TypedTransaction) -> Result<Bytes, Box<dyn std::error::Error>> {
        let result = self.provider.call(&tx, None).await?;
        Ok(result)
    }

    pub fn sign_transaction(signer: &Wallet<SigningKey>, tx: &TypedTransaction) -> Result<Signature, Box<dyn std::error::Error>> {
        let sign = signer.sign_transaction_sync(tx)?;
        Ok(sign)
    }

    pub async fn send_signed_transaction(&self, signed_tx: Vec<u8>) -> Result<Option<H256>, Box<dyn std::error::Error>> {
        self.provider
            .send_raw_transaction(Bytes::from(signed_tx))
            .await?
            .await
            .map(|receipt|
                receipt.map(|r| r.transaction_hash)
            )
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)
    }

    pub async fn send_transaction(&self, wallet: &Wallet<SigningKey>, tx: &TypedTransaction) -> Result<Option<H256>, Box<dyn std::error::Error>> {
        let sign = Self::sign_transaction(wallet, tx)?;
        let signed_tx = tx.rlp_signed(&sign).to_vec();
        let tx_hash = self.send_signed_transaction(signed_tx).await?;
        Ok(tx_hash)
    }
    
    pub async fn prepare_tx(&self, tx: TransactionRequest, sender: Address) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
        let gas = self.estimate_gas(&tx).await?;
        let nonce = self.get_nonce(sender).await?;
        let gas_price = self.get_gas_price().await?;
        
        Ok(tx
            .gas(gas)
            .nonce(nonce)
            .gas_price(gas_price))
    }

    pub fn create_hex_tx(tx: &TransactionRequest) -> String {
        format!("0x{}", hex::encode(tx.rlp()))
    }

    pub fn create_hex_tx_from_signed(tx: &TransactionRequest, sign: &Signature) -> String {
        format!("0x{}", hex::encode(tx.rlp_signed(sign)))
    }
}