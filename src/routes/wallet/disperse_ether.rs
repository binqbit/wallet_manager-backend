use actix_web::{post, web, HttpResponse, Responder};
use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::contracts::disperse_collect::DisperseCollect;
use crate::contracts::provider::EthProvider;
use crate::utils::web3::parse_ether;
use super::checks::check_balance;

#[derive(Deserialize, Serialize)]
struct DisperseEtherRequest {
    sender: Address,
    recipients: Vec<Address>,
    values: Vec<String>,
    value: String,
}

#[derive(Deserialize, Serialize)]
struct DisperseEtherByPercentRequest {
    sender: Address,
    recipients: Vec<Address>,
    percentages: Vec<u8>,
    value: String,
}

#[post("/disperseEther")]
async fn disperse_ether(req: web::Json<DisperseEtherRequest>, disperse_collect: web::Data<DisperseCollect>) -> impl Responder {
    let mut values: Vec<U256> = vec![];
    for value in req.values.iter() {
        match parse_ether(value) {
            Ok(value) => values.push(value),
            Err(err) => {
                eprintln!("Failed to parse ether: {err:?}");
                return HttpResponse::BadRequest().json(json!({"status": "error", "message": "Failed to parse ether" }));
            },
        }
    }
    let value = match parse_ether(&req.value) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to parse ether: {err:?}");
            return HttpResponse::BadRequest().json(json!({"status": "error", "message": "Failed to parse ether" }));
        },
    };

    if let Err(err) = check_balance(&disperse_collect.provider, req.sender, value).await {
        return err;
    }

    let tx = disperse_collect.create_disperse_ether_tx(req.sender, req.recipients.to_owned(), values, value).unwrap();
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

#[post("/disperseEtherByPercent")]
async fn disperse_ether_by_percent(req: web::Json<DisperseEtherByPercentRequest>, disperse_collect: web::Data<DisperseCollect>) -> impl Responder {
    let percentages: Vec<U256> = req.percentages.iter().map(|&p| U256::from(p)).collect();
    let value = match parse_ether(&req.value) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to parse ether: {err:?}");
            return HttpResponse::BadRequest().json(json!({"status": "error", "message": "Failed to parse ether" }));
        },
    };

    if let Err(err) = check_balance(&disperse_collect.provider, req.sender, value).await {
        return err;
    }

    let tx = disperse_collect.create_disperse_ether_by_percent_tx(req.sender, req.recipients.to_owned(), percentages, value).unwrap();
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
