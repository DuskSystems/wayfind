use super::FromRequestParts;
use crate::state::SharedAppState;
use http::request::Parts;
use std::convert::Infallible;

/// Access to the given request method.
pub struct Method(pub http::Method);

impl FromRequestParts for Method {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        _: &SharedAppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(parts.method.clone()))
    }
}
