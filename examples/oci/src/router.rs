use crate::{
    extract::{path::PathInner, route::RouteInner, AppRequest},
    handler::Handler,
    response::{AppResponse, IntoResponse},
    state::SharedAppState,
};
use bytes::Bytes;
use http::{Method, Response, StatusCode};
use http_body_util::Full;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};
use wayfind::{Constraint, RoutableBuilder};

/// Type alias for async handlers.
type ArcHandler = Arc<
    dyn Fn(AppRequest, SharedAppState) -> Pin<Box<dyn Future<Output = AppResponse> + Send>>
        + Send
        + Sync,
>;

pub struct AppRouter<'r> {
    /// Maps HTTP methods to their respective `wayfind` Routers.
    /// TODO: Replace with native `wayfind` method routing, when implemented.
    routes: HashMap<Method, wayfind::Router<'r, ArcHandler>>,
}

impl<'r> AppRouter<'r> {
    /// Creates a new `AppRouter` with empty route tables for all HTTP methods.
    #[must_use]
    pub fn new() -> Self {
        let mut router = Self {
            routes: HashMap::new(),
        };

        for method in [
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::HEAD,
            Method::OPTIONS,
            Method::CONNECT,
            Method::PATCH,
            Method::TRACE,
        ] {
            router.routes.insert(method, wayfind::Router::new());
        }

        router
    }

    /// Registers a constraint to all route tables.
    pub fn constraint<C: Constraint>(&mut self) {
        for router in self.routes.values_mut() {
            router.constraint::<C>().unwrap();
        }
    }

    /// Adds a new route with the specified method, path, and handler.
    pub fn route<H, T>(&mut self, method: Method, path: &'r str, handler: H)
    where
        H: Handler<T> + Send + Sync + 'static,
    {
        let handler: ArcHandler = Arc::new(move |req, state| {
            let handler = handler.clone();
            Box::pin(async move { handler.call(req, state).await })
        });

        if let Some(router) = self.routes.get_mut(&method) {
            let route = RoutableBuilder::new().route(path).build().unwrap();
            router.insert(&route, handler).unwrap();
        } else {
            let mut new_router = wayfind::Router::new();
            let route = RoutableBuilder::new().route(path).build().unwrap();
            new_router.insert(&route, handler).unwrap();
            self.routes.insert(method, new_router);
        }
    }

    /// Handles an incoming request by routing it to the appropriate handler.
    pub async fn handle(&self, mut req: AppRequest, state: SharedAppState) -> AppResponse {
        let method = req.method();
        let path = req.uri().path().to_owned();

        let Ok(path) = wayfind::Path::new(&path) else {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from("Not Found")))
                .unwrap();
        };

        let Some(router) = self.routes.get(method) else {
            return StatusCode::METHOD_NOT_ALLOWED.into_response();
        };

        let Ok(Some(search)) = router.search(&path) else {
            return StatusCode::NOT_FOUND.into_response();
        };

        let route = search.route.to_owned();
        let parameters: Vec<(String, String)> = search
            .parameters
            .into_iter()
            .map(|p| (p.0.to_owned(), p.1.to_owned()))
            .collect();

        req.extensions_mut().insert(RouteInner(route));
        req.extensions_mut().insert(PathInner(parameters));

        let handler = search.data;
        handler(req, state).await
    }
}

impl<'r> Default for AppRouter<'r> {
    fn default() -> Self {
        Self::new()
    }
}
