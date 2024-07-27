use actix_web::HttpResponse;
use ethers::types::{Address, U256};
use serde_json::json;

use crate::contracts::{provider::EthProvider, erc20::ERC20};

pub async fn check_balance(provider: &EthProvider, address: Address, required_balance: U256) -> Result<(), HttpResponse> {
    let balance = provider.get_balance(address).await.map_err(|e| {
        HttpResponse::InternalServerError().json(json!({"status": "error", "message": e.to_string()}))
    })?;

    if balance < required_balance {
        return Err(HttpResponse::BadRequest().json(json!({"status": "error", "message": "Insufficient balance"})));
    }

    Ok(())
}

pub async fn check_token_balance(erc20: &ERC20, owner: Address, required_balance: U256) -> Result<(), HttpResponse> {
    let balance = erc20.query_balance_of(owner).await.map_err(|e| {
        HttpResponse::InternalServerError().json(json!({"status": "error", "message": e.to_string()}))
    })?;

    if balance < required_balance {
        return Err(HttpResponse::BadRequest().json(json!({"status": "error", "message": "Insufficient token balance"})));
    }

    Ok(())
}

pub async fn check_allowance(erc20: &ERC20, owner: Address, spender: Address, required_allowance: U256) -> Result<(), HttpResponse> {
    let allowance = erc20.query_allowance(owner, spender).await.map_err(|e| {
        HttpResponse::InternalServerError().json(json!({"status": "error", "message": e.to_string()}))
    })?;

    if allowance < required_allowance {
        return Err(HttpResponse::BadRequest().json(json!({"status": "error", "message": "Insufficient allowance"})));
    }

    Ok(())
}
