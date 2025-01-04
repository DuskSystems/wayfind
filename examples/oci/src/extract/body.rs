use bytes::Bytes;
use http::{header::CONTENT_TYPE, Request, Response, StatusCode};
use http_body_util::{BodyExt, Full};
use serde_json::json;
use thiserror::Error;

use crate::{
    response::{AppResponse, IntoResponse},
    state::SharedAppState,
};

use super::FromRequest;

#[derive(Debug, Error)]
pub enum BodyError {
    #[error("Failed to read request body: {0}")]
    Hyper(#[from] hyper::Error),
}

impl BodyError {
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::Hyper(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for BodyError {
    fn into_response(self) -> AppResponse {
        let status = self.status_code();
        let body = json!({
            "error": {
                "code": status.as_u16().to_string(),
                "message": self.to_string()
            }
        });

        Response::builder()
            .status(status)
            .header(CONTENT_TYPE, "application/json")
            .body(Full::new(Bytes::from(body.to_string())))
            .unwrap()
    }
}

/// Access to the given request body.
/// Consumes entire body into bytes.
/// Likely inefficient, but works for simple use cases.
pub struct Body(pub Bytes);

impl FromRequest for Body {
    type Rejection = BodyError;

    async fn from_request(
        req: Request<hyper::body::Incoming>,
        _: &SharedAppState,
    ) -> Result<Self, Self::Rejection> {
        let body = req.into_body();
        let bytes = body.collect().await.map_err(BodyError::Hyper)?.to_bytes();

        Ok(Self(bytes))
    }
}
