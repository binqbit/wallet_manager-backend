
#[macro_use]
extern crate lazy_static;

use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};

pub mod utils;
pub mod contracts;
pub mod routes;

use contracts::disperse_collect::DisperseCollect;
pub use utils::variables;
use utils::variables::{check_env, DISPERSE_COLLECT_CONTRACT_ADDRESS, PORT, RPC_PROVIDER_URL};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    check_env();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::CONTENT_TYPE])
            .max_age(3600);
        let disperse_collect = DisperseCollect::new(
                &RPC_PROVIDER_URL,
                &DISPERSE_COLLECT_CONTRACT_ADDRESS,
            ).expect("Failed to create DisperseCollect instance");
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(disperse_collect))
            .service(routes::routes())
    })
    .bind(("0.0.0.0", *PORT))?
    .run()
    .await
}