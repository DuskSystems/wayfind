use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use http::{Method, StatusCode};
use wayfind::Constraint;

use crate::{
    extract::{path::PathInner, template::TemplateInner, AppRequest},
    handler::Handler,
    response::{AppResponse, IntoResponse},
    state::SharedAppState,
};

/// Type alias for async handlers.
type ArcHandler = Arc<
    dyn Fn(AppRequest, SharedAppState) -> Pin<Box<dyn Future<Output = AppResponse> + Send>>
        + Send
        + Sync,
>;

pub struct AppRouter {
    /// Maps HTTP methods to their respective `wayfind` Routers.
    routes: HashMap<Method, wayfind::Router<ArcHandler>>,
}

impl AppRouter {
    /// Creates a new `AppRouter` with empty route tables for all HTTP methods.
    #[must_use]
    pub fn new() -> Self {
        let mut router = Self {
            routes: HashMap::default(),
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
    pub fn route<H, T>(&mut self, method: Method, path: &str, handler: H)
    where
        H: Handler<T> + Send + Sync + 'static,
    {
        let handler: ArcHandler = Arc::new(move |req, state| {
            let handler = handler.clone();
            Box::pin(async move { handler.call(req, state).await })
        });

        if let Some(router) = self.routes.get_mut(&method) {
            router.insert(path, handler).unwrap();
        } else {
            let mut new_router = wayfind::Router::new();
            new_router.insert(path, handler).unwrap();
            self.routes.insert(method, new_router);
        }
    }

    /// Handles an incoming request by routing it to the appropriate handler.
    pub async fn handle(&self, mut req: AppRequest, state: SharedAppState) -> AppResponse {
        let method = req.method();
        let path = req.uri().path().to_owned();

        let Some(router) = self.routes.get(method) else {
            return StatusCode::METHOD_NOT_ALLOWED.into_response();
        };

        let Some(search) = router.search(&path) else {
            return StatusCode::NOT_FOUND.into_response();
        };

        let template = search.template.to_owned();
        let parameters: Vec<(String, String)> = search
            .parameters
            .into_iter()
            .map(|p| (p.0.to_owned(), p.1.to_owned()))
            .collect();

        req.extensions_mut().insert(TemplateInner(template));
        req.extensions_mut().insert(PathInner(parameters));

        let handler = search.data;
        handler(req, state).await
    }
}
