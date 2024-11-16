#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions
)]

use anyhow::Error;
use constraints::name::NameConstraint;
use hyper::service::service_fn;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use router::AppRouter;
use state::AppState;
use std::{convert::Infallible, sync::Arc};
use tokio::{net::TcpListener, task::JoinSet};
use wayfind::RouteBuilder;

pub mod constraints;
pub mod extract;
pub mod handler;
pub mod response;
pub mod router;
pub mod routes;
pub mod state;
pub mod types;

pub async fn start_server(listener: TcpListener) -> Result<(), Error> {
    let state = Arc::new(AppState::new());

    let mut router = AppRouter::new();
    router.constraint::<NameConstraint>();

    // end-1
    let route = RouteBuilder::new()
        .route("/v2(/)")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, routes::root::handle_root_get);

    // end-2
    let route = RouteBuilder::new()
        .route("/v2/{*name:name}/blobs/{digest}(/)")
        .methods(vec!["GET", "HEAD"])
        .build()?;
    router.insert(&route, routes::blob::handle_blob_pull);

    // end-3
    let route = RouteBuilder::new()
        .route("/v2/{*name:name}/manifests/{reference}(/)")
        .methods(vec!["GET", "HEAD"])
        .build()?;
    router.insert(&route, routes::manifest::handle_manifest_pull);

    // end-4a / end-4b
    let route = RouteBuilder::new()
        .route("/v2/{*name:name}/blobs/uploads(/)")
        .methods(vec!["POST"])
        .build()?;
    router.insert(&route, routes::blob::handle_blob_push_post);

    // end-6
    let route = RouteBuilder::new()
        .route("/v2/{*name:name}/blobs/uploads/{reference}(/)")
        .methods(vec!["PUT"])
        .build()?;
    router.insert(&route, routes::blob::handle_blob_push_put);

    // end-7
    let route = RouteBuilder::new()
        .route("/v2/{*name:name}/manifests/{reference}(/)")
        .methods(vec!["PUT"])
        .build()?;
    router.insert(&route, routes::manifest::handle_manifest_put);

    // end-8a
    let route = RouteBuilder::new()
        .route("/v2/{*name:name}/tags/list(/)")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, routes::tags::handle_tags_get);

    // end-9
    let route = RouteBuilder::new()
        .route("/v2/{*name:name}/manifests/{reference}(/)")
        .methods(vec!["DELETE"])
        .build()?;
    router.insert(&route, routes::manifest::handle_manifest_delete);

    // end-10
    let route = RouteBuilder::new()
        .route("/v2/{*name:name}/blobs/{digest}(/)")
        .methods(vec!["DELETE"])
        .build()?;
    router.insert(&route, routes::blob::handle_blob_delete);

    println!("{}", router.inner);
    let router = Arc::new(router);

    tracing::info!(
        address = %listener.local_addr()?,
        "listening on http"
    );

    let mut join_set = JoinSet::new();

    loop {
        let (stream, peer_addr) = match listener.accept().await {
            Ok(x) => x,
            Err(err) => {
                tracing::error!(
                    error = %err,
                    "failed to accept connection"
                );

                continue;
            }
        };

        let router_clone = Arc::clone(&router);
        let state_clone = Arc::clone(&state);

        // Spawn a new task for each connection
        let serve_connection = async move {
            tracing::info!(
                peer_addr = %peer_addr,
                "handling a request"
            );

            let service = service_fn(move |req| {
                let router = Arc::clone(&router_clone);
                let state = Arc::clone(&state_clone);
                async move {
                    let response = router.handle(req, Arc::clone(&state)).await;
                    Ok::<_, Infallible>(response)
                }
            });

            let result = Builder::new(TokioExecutor::new())
                .serve_connection(TokioIo::new(stream), service)
                .await;

            if let Err(err) = result {
                tracing::error!(
                    error = %err,
                    peer_addr = %peer_addr,
                    "error serving connection"
                );
            }

            tracing::info!(
                peer_addr = %peer_addr,
                "handled a request"
            );
        };

        join_set.spawn(serve_connection);
    }
}
