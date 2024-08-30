use super::FromRequestParts;
use crate::state::SharedAppState;
use http::{request::Parts, HeaderMap};
use std::convert::Infallible;

/// Access to the given request headers.
///
/// TODO: Replace with `wayfind` native headers parsing, once implemented.
pub struct Headers(pub HeaderMap);

impl FromRequestParts for Headers {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        _: &SharedAppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(parts.headers.clone()))
    }
}
