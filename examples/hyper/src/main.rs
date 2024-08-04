#![allow(clippy::unused_async)]

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{body::Incoming, header, service::service_fn, Request, Response, StatusCode};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use std::{
    convert::Infallible,
    future::Future,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    pin::Pin,
    sync::Arc,
};
use tokio::{net::TcpListener, task::JoinSet};
use wayfind::{matches::Parameter, router::Router};

type BoxFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Response<BoxBody<Bytes, Infallible>>, anyhow::Error>> + Send + 'a>>;

type HandlerFn = Arc<dyn for<'a> Fn(&'a str, &'a [Parameter<'a>]) -> BoxFuture<'a> + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut router: Router<HandlerFn> = Router::new();
    router.insert(
        "/",
        Arc::new(move |path, parameters| Box::pin(index_route(path, parameters))),
    )?;
    router.insert(
        "/hello/{name}",
        Arc::new(move |path, parameters| Box::pin(hello_route(path, parameters))),
    )?;
    router.insert(
        "{catch_all:*}",
        Arc::new(move |path, parameters| Box::pin(not_found(path, parameters))),
    )?;

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1337);
    let listener = TcpListener::bind(&socket).await?;
    println!("Listening on http://{socket}");

    let router = Arc::new(router);
    let mut join_set = JoinSet::new();

    loop {
        let Ok((stream, _)) = listener.accept().await else {
            continue;
        };

        let router = Arc::clone(&router);
        join_set.spawn(async move {
            let _ = Builder::new(TokioExecutor::new())
                .serve_connection(
                    TokioIo::new(stream),
                    service_fn(move |request: Request<Incoming>| {
                        let router = Arc::clone(&router);
                        async move {
                            let path = request.uri().path();
                            let matches = router
                                .matches(path)
                                .expect("Failed to match!");

                            let handler = &matches.data.value;
                            let parameters = &matches.parameters;
                            handler(path, parameters).await
                        }
                    }),
                )
                .await;
        });
    }
}

async fn index_route(
    _: &'_ str,
    _: &'_ [Parameter<'_>],
) -> Result<Response<BoxBody<Bytes, Infallible>>, anyhow::Error> {
    let json = serde_json::json!({
        "hello": "world"
    });

    let body = Full::new(Bytes::from(json.to_string()));
    let response = Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .body(body.boxed())?;

    Ok(response)
}

async fn hello_route(
    _: &'_ str,
    parameters: &'_ [Parameter<'_>],
) -> Result<Response<BoxBody<Bytes, Infallible>>, anyhow::Error> {
    let name = String::from_utf8_lossy(parameters[0].value);
    let json = serde_json::json!({
        "hello": name,
    });

    let body = Full::new(Bytes::from(json.to_string()));
    let response = Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .body(body.boxed())?;

    Ok(response)
}

async fn not_found(
    path: &'_ str,
    _: &'_ [Parameter<'_>],
) -> Result<Response<BoxBody<Bytes, Infallible>>, anyhow::Error> {
    let json = serde_json::json!({
        "error": "route_not_found",
        "route": path,
    });

    let body = Full::new(Bytes::from(json.to_string()));
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(body.boxed())?;

    Ok(response)
}
