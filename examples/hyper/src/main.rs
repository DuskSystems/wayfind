#![allow(clippy::unused_async)]

use bytes::{Buf, Bytes};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{body::Incoming as IncomingBody, header, service::service_fn, Method, Request, Response, StatusCode};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use std::{
    convert::Infallible,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::{net::TcpListener, task::JoinSet};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1337);
    let listener = TcpListener::bind(&socket).await?;
    println!("Listening on http://{socket}");

    let mut join_set = JoinSet::new();
    loop {
        let Ok((stream, _)) = listener.accept().await else {
            continue;
        };

        join_set.spawn(async move {
            let _ = Builder::new(TokioExecutor::new())
                .serve_connection(TokioIo::new(stream), service_fn(route_request))
                .await;
        });
    }
}

async fn route_request(request: Request<IncomingBody>) -> Result<Response<BoxBody<Bytes, Infallible>>, anyhow::Error> {
    // TODO: Use router here!
    match (request.method(), request.uri().path()) {
        (&Method::GET, "/") => api_get_response().await,
        (&Method::POST, "/") => api_post_response(request).await,
        _ => {
            let json = serde_json::json!({ "route": "not_found" });

            let body = Full::new(Bytes::from(json.to_string()));
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body.boxed())?;

            Ok(response)
        }
    }
}

async fn api_post_response(
    request: Request<IncomingBody>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, anyhow::Error> {
    let buffer = request.collect().await?.aggregate();

    let mut json: serde_json::Value = serde_json::from_reader(buffer.reader())?;
    json["test"] = serde_json::Value::from("test_value");

    let body = Full::new(Bytes::from(json.to_string()));
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(body.boxed())?;

    Ok(response)
}

async fn api_get_response() -> Result<Response<BoxBody<Bytes, Infallible>>, anyhow::Error> {
    let json = serde_json::json!({ "hello": "world" });

    let body = Full::new(Bytes::from(json.to_string()));
    let response = Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .body(body.boxed())?;

    Ok(response)
}
