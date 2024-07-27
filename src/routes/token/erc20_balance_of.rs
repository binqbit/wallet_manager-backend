use actix_web::{post, web, HttpResponse, Responder};
use ethers::types::Address;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{contracts::erc20::ERC20, variables::RPC_PROVIDER_URL};

#[derive(Deserialize, Serialize)]
struct BalanceOfRequest {
    token: Address,
    owner: Address,
}

#[post("/balanceOf")]
async fn balance_of(req: web::Json<BalanceOfRequest>) -> impl Responder {
    let erc20 = match ERC20::new(&RPC_PROVIDER_URL, req.token) {
        Ok(erc20) => erc20,
        Err(err) => {
            eprintln!("Failed to create ERC20 contract: {err:?}");
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create ERC20 contract" }));
        },
    };

    let balance = match erc20.query_balance_of(req.owner).await {
        Ok(res) => match erc20.token_to_string(res).await {
            Ok(res) => res,
            Err(err) => {
                eprintln!("Failed to convert balance to string: {err:?}");
                return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to convert balance to string" }));
            },
        },
        Err(err) => {
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": err.to_string()}));
        },
    };

    HttpResponse::Ok().json(json!({"status": "success", "balance": balance }))
}
