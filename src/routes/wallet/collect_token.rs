use actix_web::{post, web, HttpResponse, Responder};
use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::contracts::{disperse_collect::DisperseCollect, erc20::ERC20};
use crate::contracts::provider::EthProvider;
use crate::variables::RPC_PROVIDER_URL;
use super::checks::{check_token_balance, check_allowance};

#[derive(Deserialize, Serialize)]
struct CollectTokenRequest {
    sender: Address,
    token: Address,
    recipient: Address,
    contributors: Vec<Address>,
    values: Vec<String>,
}

#[post("/collectToken")]
async fn collect_token(req: web::Json<CollectTokenRequest>, disperse_collect: web::Data<DisperseCollect>) -> impl Responder {
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
                return HttpResponse::BadRequest().json(json!({"status": "error", "message": "Failed to parse token" }));
            },
        }
    }

    for (contributor, value) in req.contributors.iter().zip(values.iter()) {
        if let Err(err) = check_token_balance(&erc20, *contributor, *value).await {
            return err;
        }
        if let Err(err) = check_allowance(&erc20, *contributor, disperse_collect.contract.address(), *value).await {
            return err;
        }
    }

    let tx = disperse_collect.create_collect_token_tx(req.sender, req.token, req.recipient, req.contributors.to_owned(), values).unwrap();
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
