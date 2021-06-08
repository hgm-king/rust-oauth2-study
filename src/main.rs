use dotenv::dotenv;
use rust_oauth2_study::{
    config::Config,
    handlers::{hello_handler, shopify_handler},
    routes::{hello_route, shopify_route},
};
use std::net::SocketAddr;
use std::sync::Arc;
use warp::Filter;

pub mod api;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Arc::new(Config::new());

    let hello = hello!().with(warp::log("hello"));
    let shopify = shopify!(config.clone()).with(warp::log("shopify"));

    let end = hello.or(shopify);

    let socket_address = config
        .clone()
        .app_addr
        .parse::<SocketAddr>()
        .expect("Could not parse Addr");

    println!("Listening at {}", &config.app_addr);

    if config.clone().tls {
        println!("TLS Enabled!");

        warp::serve(end)
            .tls()
            .cert_path(config.clone().cert_path.as_ref().unwrap())
            .key_path(config.clone().key_path.as_ref().unwrap())
            .run(socket_address)
            .await;
    } else {
        warp::serve(end).run(socket_address).await;
    }
}
