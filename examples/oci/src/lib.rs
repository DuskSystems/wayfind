#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

use anyhow::Error;
use constraints::name::NameConstraint;
use http::Method;
use hyper::service::service_fn;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use router::AppRouter;
use state::AppState;
use std::{convert::Infallible, sync::Arc};
use tokio::{net::TcpListener, task::JoinSet};

pub mod constraints;
pub mod extract;
pub mod handler;
pub mod response;
pub mod router;
pub mod routes;
pub mod state;
pub mod types;

#[allow(clippy::too_many_lines)]
pub async fn start_server(listener: TcpListener) -> Result<(), Error> {
    tracing::info!(
        address = %listener.local_addr()?,
        "listening on http"
    );

    let state = Arc::new(AppState::new());

    // TODO: Enable `wayfind` method routing, when implemented.
    let mut router = AppRouter::new();
    router.constraint::<NameConstraint>();

    // end-1
    router.route(Method::GET, "/v2{/}", routes::root::handle_root_get);

    // end-2
    router.route(
        Method::GET,
        "/v2/{*name:name}/blobs/{digest}{/}",
        routes::blob::handle_blob_pull,
    );
    router.route(
        Method::HEAD,
        "/v2/{*name:name}/blobs/{digest}{/}",
        routes::blob::handle_blob_pull,
    );

    // end-3
    router.route(
        Method::GET,
        "/v2/{*name:name}/manifests/{reference}{/}",
        routes::manifest::handle_manifest_pull,
    );
    router.route(
        Method::HEAD,
        "/v2/{*name:name}/manifests/{reference}{/}",
        routes::manifest::handle_manifest_pull,
    );

    // end-4a / end-4b
    router.route(
        Method::POST,
        "/v2/{*name:name}/blobs/uploads{/}",
        routes::blob::handle_blob_push_post,
    );

    // end-6
    router.route(
        Method::PUT,
        "/v2/{*name:name}/blobs/uploads/{reference}{/}",
        routes::blob::handle_blob_push_put,
    );

    // end-7
    router.route(
        Method::PUT,
        "/v2/{*name:name}/manifests/{reference}{/}",
        routes::manifest::handle_manifest_put,
    );

    // end-8a
    router.route(
        Method::GET,
        "/v2/{*name:name}/tags/list{/}",
        routes::tags::handle_tags_get,
    );

    // end-9
    router.route(
        Method::DELETE,
        "/v2/{*name:name}/manifests/{reference}{/}",
        routes::manifest::handle_manifest_delete,
    );

    // end-10
    router.route(
        Method::DELETE,
        "/v2/{*name:name}/blobs/{digest}{/}",
        routes::blob::handle_blob_delete,
    );

    let router = Arc::new(router);

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
