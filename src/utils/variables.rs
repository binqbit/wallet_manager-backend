use std::env;

lazy_static! {
    pub static ref PORT: u16 =
        env::var("PORT").unwrap_or("8000".to_string()).parse().expect("PORT must be a number.");
    pub static ref RPC_PROVIDER_URL: String =
        env::var("RPC_PROVIDER_URL").expect("RPC_PROVIDER_URL environment variable is not set.");
    pub static ref DISPERSE_COLLECT_CONTRACT_ADDRESS: String =
        env::var("DISPERSE_COLLECT_CONTRACT_ADDRESS").expect("DISPERSE_COLLECT_CONTRACT_ADDRESS environment variable is not set.");
}

pub fn check_env() {
    dotenv::dotenv().ok();
    println!("env PORT = {}", *PORT);
    println!("env RPC_PROVIDER_URL = {}", *RPC_PROVIDER_URL);
    println!("env DISPERSE_COLLECT_CONTRACT_ADDRESS = {}", *DISPERSE_COLLECT_CONTRACT_ADDRESS);
}
