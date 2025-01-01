use crate::{
    extract::{path::PathInner, route::RouteInner, AppRequest},
    handler::Handler,
    response::{AppResponse, IntoResponse},
    state::SharedAppState,
};
use bytes::Bytes;
use http::{Response, StatusCode};
use http_body_util::Full;
use std::{future::Future, pin::Pin, sync::Arc};
use wayfind::{
    errors::{MethodSearchError, SearchError},
    PathConstraint, Route, Router,
};

/// Type alias for async handlers.
type ArcHandler = Arc<
    dyn Fn(AppRequest, SharedAppState) -> Pin<Box<dyn Future<Output = AppResponse> + Send>>
        + Send
        + Sync,
>;

pub struct AppRouter {
    /// Maps HTTP methods to their respective `wayfind` Routers.
    pub inner: Router<ArcHandler>,
}

impl AppRouter {
    /// Creates a new `AppRouter` with empty route tables for all HTTP methods.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Router::new(),
        }
    }

    /// Registers a constraint to all route tables.
    pub fn constraint<C: PathConstraint>(&mut self) {
        self.inner.path.constraint::<C>().unwrap();
    }

    /// Adds a new route with the specified method, path, and handler.
    pub fn insert<H, T>(&mut self, route: &Route<'_>, handler: H)
    where
        H: Handler<T> + Send + Sync + 'static,
    {
        let handler: ArcHandler = Arc::new(move |req, state| {
            let handler = handler.clone();
            Box::pin(async move { handler.call(req, state).await })
        });

        self.inner.insert(route, handler).unwrap();
    }

    pub async fn handle(&self, mut req: AppRequest, state: SharedAppState) -> AppResponse {
        let path = req.uri().path().to_owned();
        let method = req.method().as_str().to_owned();

        let Ok(request) = wayfind::RequestBuilder::new()
            .path(&path)
            .method(&method)
            .build()
        else {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from("Not Found")))
                .unwrap();
        };

        let result = self.inner.search(&request);
        match result {
            Ok(Some(search)) => {
                let route = search.path.route.to_string();
                let parameters: Vec<(String, String)> = search
                    .path
                    .parameters
                    .into_iter()
                    .map(|p| (p.0.to_owned(), p.1.to_owned()))
                    .collect();

                req.extensions_mut().insert(RouteInner(route));
                req.extensions_mut().insert(PathInner(parameters));

                let handler = search.data;
                handler(req, state).await
            }
            Err(SearchError::Method(MethodSearchError::NotAllowed)) => {
                StatusCode::METHOD_NOT_ALLOWED.into_response()
            }
            Ok(None) | Err(_) => StatusCode::NOT_FOUND.into_response(),
        }
    }
}
