use actix_web::{post, web, HttpResponse, Responder};
use ethers::{types::{transaction::eip2718::TypedTransaction, TransactionRequest}, utils::hex};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::contracts::{disperse_collect::DisperseCollect, provider::EthProvider};

#[derive(Deserialize, Serialize)]
struct SendSignedTransactionRequest {
    signed_tx: String,
}

#[derive(Deserialize, Serialize)]
struct SignTransactionRequest {
    tx: TransactionRequest,
    private_key: String,
}

#[post("/sendSignedTransaction")]
async fn send_signed_transaction(req: web::Json<SendSignedTransactionRequest>, disperse_collect: web::Data<DisperseCollect>) -> impl Responder {
    let signed_tx = match hex::decode(&req.signed_tx) {
        Ok(signed_tx) => signed_tx,
        Err(err) => {
            eprintln!("Failed to decode signed transaction: {err:?}");
            return HttpResponse::BadRequest().json(json!({"status": "error", "message": "Failed to decode signed transaction" }));
        },
    };
    let tx_hash = match disperse_collect.provider.send_signed_transaction(signed_tx).await {
        Ok(tx_hash) => tx_hash,
        Err(err) => {
            eprintln!("Failed to send signed transaction: {err:?}");
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to send signed transaction" }));
        },
    };

    HttpResponse::Ok().json(json!({"status": "success", "tx_hash": tx_hash }))
}

// sign transaction with private key (for testing)
#[post("/signTransaction")]
async fn sign_transaction(req: web::Json<SignTransactionRequest>) -> impl Responder {
    let wallet = match EthProvider::create_wallet(&req.private_key) {
        Ok(wallet) => wallet,
        Err(err) => {
            eprintln!("Failed to create wallet: {err:?}");
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create wallet" }));
        },
    };

    let tx = TypedTransaction::Legacy(req.tx.clone());

    match EthProvider::sign_transaction(&wallet, &tx) {
        Ok(sign) => {
            let signed_tx = EthProvider::create_hex_tx_from_signed(&req.tx, &sign);
            HttpResponse::Ok().json(json!({"status": "success", "signed_tx": signed_tx }))
        },
        Err(err) => {
            eprintln!("Failed to sign transaction: {err:?}");
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to sign transaction" }))
        },
    }
}