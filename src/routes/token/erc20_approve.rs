use actix_web::{post, web, HttpResponse, Responder};
use ethers::types::Address;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::contracts::erc20::ERC20;
use crate::contracts::provider::EthProvider;
use crate::variables::RPC_PROVIDER_URL;

#[derive(Deserialize, Serialize)]
struct ApproveRequest {
    token: Address,
    sender: Address,
    spender: Address,
    amount: String,
}

#[post("/approve")]
async fn approve(req: web::Json<ApproveRequest>) -> impl Responder {
    let erc20 = match ERC20::new(&RPC_PROVIDER_URL, req.token) {
        Ok(erc20) => erc20,
        Err(err) => {
            eprintln!("Failed to create ERC20 contract: {err:?}");
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create ERC20 contract" }));
        },
    };

    let amount = match erc20.parse_token(&req.amount).await {
        Ok(amount) => amount,
        Err(err) => {
            eprintln!("Failed to parse amount: {err:?}");
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to parse amount" }));
        },
    };

    let tx = erc20.create_approve_tx(req.sender, req.spender, amount).unwrap();
    let tx = match erc20.provider.prepare_tx(tx, req.sender).await {
        Ok(tx) => tx,
        Err(err) => {
            eprintln!("Failed to prepare transaction: {err:?}");
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to prepare transaction" }));
        },
    };
    let tx_hex = EthProvider::create_hex_tx(&tx);

    HttpResponse::Ok().json(json!({"status": "success", "tx": tx, "tx_hex": tx_hex }))
}
