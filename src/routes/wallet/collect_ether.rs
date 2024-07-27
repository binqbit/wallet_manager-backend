use actix_web::{post, web, HttpResponse, Responder};
use ethers::types::Address;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::contracts::disperse_collect::DisperseCollect;
use crate::contracts::provider::EthProvider;
use crate::utils::web3::parse_ether;
use super::checks::check_balance;

#[derive(Deserialize, Serialize)]
struct CollectEtherRequest {
    sender: Address,
    recipient: Address,
    value: String,
}

#[post("/collectEther")]
async fn collect_ether(req: web::Json<CollectEtherRequest>, disperse_collect: web::Data<DisperseCollect>) -> impl Responder {
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

    let tx = disperse_collect.create_collect_ether_tx(req.sender, req.recipient, value).unwrap();
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
