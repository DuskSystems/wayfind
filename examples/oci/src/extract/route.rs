use super::FromRequestParts;
use crate::state::SharedAppState;
use http::request::Parts;
use std::convert::Infallible;

#[derive(Debug, Clone)]
pub struct RouteInner(pub String);

/// Access to the given request route.
pub struct Route(pub String);

impl FromRequestParts for Route {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        _: &SharedAppState,
    ) -> Result<Self, Self::Rejection> {
        let route = parts.extensions.get::<RouteInner>().unwrap();
        Ok(Self(route.0.clone()))
    }
}
