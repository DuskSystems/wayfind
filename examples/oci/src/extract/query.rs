use super::FromRequestParts;
use crate::{
    response::{AppResponse, IntoResponse},
    state::SharedAppState,
};
use bytes::Bytes;
use http::{request::Parts, Response, StatusCode};
use http_body_util::Full;
use percent_encoding::percent_decode_str;
use serde_json::json;
use std::{collections::HashMap, str::Utf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("Invalid query string")]
    InvalidQuery,

    #[error("Failed to decode query parameter: {0}")]
    DecodingError(#[from] Utf8Error),
}

impl QueryError {
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidQuery | Self::DecodingError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for QueryError {
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
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Full::new(Bytes::from(body.to_string())))
            .unwrap()
    }
}

/// Access to the given request parameters.
/// Params are parsed and decoded.
///
/// TODO: Replace with `wayfind` native query parsing, once implemented.
pub struct Query(pub HashMap<String, String>);

impl FromRequestParts for Query {
    type Rejection = QueryError;

    async fn from_request_parts(
        parts: &mut Parts,
        _: &SharedAppState,
    ) -> Result<Self, Self::Rejection> {
        let mut params = HashMap::new();

        let Some(query_string) = parts.uri.query() else {
            return Ok(Self(params));
        };

        for pair in query_string.split('&') {
            if pair.is_empty() {
                return Err(QueryError::InvalidQuery);
            }

            let (key, value) = pair.split_once('=').ok_or(QueryError::InvalidQuery)?;

            let decoded_key = percent_decode_str(key)
                .decode_utf8()
                .map_err(QueryError::DecodingError)?;

            let decoded_value = percent_decode_str(value)
                .decode_utf8()
                .map_err(QueryError::DecodingError)?;

            params
                .entry(decoded_key.to_string())
                .or_insert_with(|| decoded_value.to_string());
        }

        Ok(Self(params))
    }
}
