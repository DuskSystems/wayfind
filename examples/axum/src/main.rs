#![allow(clippy::multiple_crate_versions)]

use serde_json::{json, Value};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use wayfind_axum::{extract::Path, routing::get, Json, Router};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let app = Router::new()
        .route("/", get(index_route))
        .route("/hello/{name}", get(hello_route))
        .route("/{*catch_all}", get(not_found));

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1337);
    let listener = TcpListener::bind(&socket).await?;
    println!("Listening on http://{socket}");

    wayfind_axum::serve(listener, app).await?;
    Ok(())
}

async fn index_route() -> Json<Value> {
    Json(json!({
        "hello": "world"
    }))
}

async fn hello_route(Path(name): Path<String>) -> Json<Value> {
    Json(json!({
        "hello": name,
    }))
}

async fn not_found(Path(path): Path<String>) -> Json<Value> {
    Json(json!({
        "error": "route_not_found",
        "route": path,
    }))
}
