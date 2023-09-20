//#![allow(unused_imports)]
use anyhow::Result;
//use thiserror::Error;

use axum_test::server;

use axum_test::auth;
use axum_test::secrets;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        std::env::set_var("RUST_LOG", "info");
        "info".to_string()
    });
    println!("Using log level: {}", rust_log);
    env_logger::init();

    dotenv().expect(".dot env file unable to load");

    let address = "0.0.0.0:3000";

    let token = auth::generate_token(
        "this is a long subject",
        &secrets::TOKEN_ISSUER.to_string(),
        secrets::TOKEN_EXPIRY_DAYS
            .to_string()
            .parse::<u64>()
            .unwrap(),
    )
    .unwrap();

    log::info!("Token is: {}", token);

    let handler = server::server(address).await?;

    log::info!("Server started");

    handler.await?;

    log::info!("Server Stopped!!");

    Ok(())
}
