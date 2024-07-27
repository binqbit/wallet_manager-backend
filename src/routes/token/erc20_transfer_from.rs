use actix_web::{post, web, HttpResponse, Responder};
use ethers::types::Address;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{contracts::erc20::ERC20, variables::RPC_PROVIDER_URL};
use crate::contracts::provider::EthProvider;
use super::checks::{check_token_balance, check_allowance};

#[derive(Deserialize, Serialize)]
struct TransferFromRequest {
    token: Address,
    sender: Address,
    from: Address,
    to: Address,
    amount: String,
}

#[post("/transferFrom")]
async fn transfer_from(req: web::Json<TransferFromRequest>) -> impl Responder {
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

    if let Err(err) = check_token_balance(&erc20, req.from, amount).await {
        return err;
    }

    if let Err(err) = check_allowance(&erc20, req.from, req.sender, amount).await {
        return err;
    }

    let tx = erc20.create_transfer_from_tx(req.sender, req.from, req.to, amount).unwrap();
    let tx_hex = EthProvider::create_hex_tx(&tx);

    HttpResponse::Ok().json(json!({"status": "success", "tx": tx, "tx_hex": tx_hex }))
}