use std::convert::Infallible;

use http::request::Parts;

use crate::state::SharedAppState;

use super::FromRequestParts;

#[derive(Debug, Clone)]
pub struct TemplateInner(pub String);

/// Access to the given request template.
pub struct Template(pub String);

impl FromRequestParts for Template {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        _: &SharedAppState,
    ) -> Result<Self, Self::Rejection> {
        let template = parts.extensions.get::<TemplateInner>().unwrap();
        Ok(Self(template.0.clone()))
    }
}
