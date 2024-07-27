use actix_web::{post, web, HttpResponse, Responder};
use ethers::types::Address;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::contracts::erc20::ERC20;
use crate::contracts::provider::EthProvider;
use crate::variables::RPC_PROVIDER_URL;
use super::checks::check_token_balance;

#[derive(Deserialize, Serialize)]
struct TransferRequest {
    token: Address,
    sender: Address,
    recipient: Address,
    amount: String,
}

#[post("/transfer")]
async fn transfer(req: web::Json<TransferRequest>) -> impl Responder {
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

    if let Err(err) = check_token_balance(&erc20, req.sender, amount).await {
        return err;
    }

    let tx = erc20.create_transfer_tx(req.sender, req.recipient, amount).unwrap();
    let tx_hex = EthProvider::create_hex_tx(&tx);

    HttpResponse::Ok().json(json!({"status": "success", "tx": tx, "tx_hex": tx_hex }))
}
