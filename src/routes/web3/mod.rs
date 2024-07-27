use actix_web::{web, Scope};

mod send_signed_transaction;

pub fn route() -> Scope {
    web::scope("/web3")
        .service(send_signed_transaction::send_signed_transaction)
        .service(send_signed_transaction::sign_transaction)
}
