
use actix_web::{web, Scope};

mod erc20_transfer;
mod erc20_approve;
mod erc20_transfer_from;
mod erc20_balance_of;
mod erc20_allowance;
mod checks;

pub fn route() -> Scope {
    web::scope("/token")
        .service(erc20_transfer::transfer)
        .service(erc20_approve::approve)
        .service(erc20_transfer_from::transfer_from)
        .service(erc20_balance_of::balance_of)
        .service(erc20_allowance::allowance)
}
