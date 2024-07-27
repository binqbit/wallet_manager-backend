use actix_web::{post, web, HttpResponse, Responder};
use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::contracts::{disperse_collect::DisperseCollect, erc20::ERC20};
use crate::contracts::provider::EthProvider;
use crate::variables::RPC_PROVIDER_URL;
use super::checks::{check_token_balance, check_allowance};

#[derive(Deserialize, Serialize)]
struct DisperseTokenRequest {
    sender: Address,
    token: Address,
    recipients: Vec<Address>,
    values: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct DisperseTokenByPercentRequest {
    sender: Address,
    token: Address,
    recipients: Vec<Address>,
    percentages: Vec<u8>,
}

#[post("/disperseToken")]
async fn disperse_token(req: web::Json<DisperseTokenRequest>, disperse_collect: web::Data<DisperseCollect>) -> impl Responder {
    let erc20 = match ERC20::new(&RPC_PROVIDER_URL, req.token) {
        Ok(erc20) => erc20,
        Err(err) => {
            eprintln!("Failed to create ERC20 contract: {err:?}");
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create ERC20 contract" }));
        },
    };

    let mut values: Vec<U256> = vec![];
    for value in req.values.iter() {
        match erc20.parse_token(value).await {
            Ok(value) => values.push(value),
            Err(err) => {
                eprintln!("Failed to parse token: {err:?}");
                return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to parse token" }));
            },
        }
    }
    let total_value: U256 = values.iter().cloned().fold(U256::zero(), |acc, v| acc + v);

    if let Err(err) = check_token_balance(&erc20, req.sender, total_value).await {
        return err;
    }

    if let Err(err) = check_allowance(&erc20, req.sender, disperse_collect.contract.address(), total_value).await {
        return err;
    }

    let tx = disperse_collect.create_disperse_token_tx(req.sender, req.token, req.recipients.to_owned(), values).unwrap();
    let tx = match disperse_collect.provider.prepare_tx(tx, req.sender).await {
        Ok(tx) => tx,
        Err(err) => {
            eprintln!("Failed to prepare transaction: {err:?}");
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to prepare transaction" }));
        },
    };
    let tx_hex = EthProvider::create_hex_tx(&tx);

    HttpResponse::Ok().json(json!({"status": "success", "tx": tx, "tx_hex": tx_hex }))
}

#[post("/disperseTokenByPercent")]
async fn disperse_token_by_percent(req: web::Json<DisperseTokenByPercentRequest>, disperse_collect: web::Data<DisperseCollect>) -> impl Responder {
    let erc20 = match ERC20::new(&RPC_PROVIDER_URL, req.token) {
        Ok(erc20) => erc20,
        Err(err) => {
            eprintln!("Failed to create ERC20 contract: {err:?}");
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create ERC20 contract" }));
        },
    };

    let percentages: Vec<U256> = req.percentages.iter().map(|&p| U256::from(p)).collect();
    let total_percentage: U256 = percentages.iter().cloned().fold(U256::zero(), |acc, p| acc + p);
    let value = match erc20.query_allowance(req.sender, disperse_collect.contract.address()).await {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to query allowance: {err:?}");
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to query allowance" }));
        },
    };

    if value == U256::zero() {
        return HttpResponse::BadRequest().json(json!({"status": "error", "message": "Allowance is zero" }));
    }

    if total_percentage > U256::from(100) {
        return HttpResponse::BadRequest().json(json!({"status": "error", "message": "Total percentage exceeds 100"}));
    }

    if let Err(err) = check_token_balance(&erc20, req.sender, value).await {
        return err;
    }

    let tx = disperse_collect.create_disperse_token_by_percent_tx(req.sender, req.token, req.recipients.to_owned(), percentages).unwrap();
    let tx = match disperse_collect.provider.prepare_tx(tx, req.sender).await {
        Ok(tx) => tx,
        Err(err) => {
            eprintln!("Failed to prepare transaction: {err:?}");
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to prepare transaction" }));
        },
    };
    let tx_hex = EthProvider::create_hex_tx(&tx);

    HttpResponse::Ok().json(json!({"status": "success", "tx": tx, "tx_hex": tx_hex }))
}
