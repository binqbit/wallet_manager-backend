use actix_web::Scope;


mod wallet;
mod token;
mod web3;


pub fn routes() -> Vec<Scope> {
    vec![
        wallet::route(),
        token::route(),
        web3::route(),
    ]
}