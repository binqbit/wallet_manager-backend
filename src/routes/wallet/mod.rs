use actix_web::{web, Scope};

mod disperse_ether;
mod disperse_token;
mod collect_ether;
mod collect_token;
mod checks;

pub fn route() -> Scope {
    web::scope("/wallet")
        .service(disperse_ether::disperse_ether)
        .service(disperse_ether::disperse_ether_by_percent)
        .service(disperse_token::disperse_token)
        .service(disperse_token::disperse_token_by_percent)
        .service(collect_ether::collect_ether)
        .service(collect_token::collect_token)
}
