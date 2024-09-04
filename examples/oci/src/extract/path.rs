use super::FromRequestParts;
use crate::response::{AppResponse, IntoResponse};
use crate::state::SharedAppState;
use bytes::Bytes;
use http::Response;
use http::{request::Parts, StatusCode};
use http_body_util::Full;
use serde_json::json;
use std::{fmt::Display, str::FromStr, sync::Arc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PathError {
    #[error("Missing path parameters")]
    MissingPathParams,

    #[error("Failed to parse path parameter: {0}")]
    ParseError(String),

    #[error("Wrong number of path parameters. Expected {expected}, got {actual}")]
    WrongNumberOfParameters { expected: usize, actual: usize },
}

impl PathError {
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::MissingPathParams => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ParseError(_) | Self::WrongNumberOfParameters { .. } => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for PathError {
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

/// Private marker trait needed, so that the single impl doesn't conflict with the tuples.
mod private {
    pub trait Marker {}
    impl Marker for String {}
}

#[derive(Debug, Clone)]
pub struct PathInner(pub Arc<Vec<(String, String)>>);

/// Access to the given request path parameters.
/// Only supports Stringable types, for now.
/// Order of parameters should match definition in routes.
#[derive(Debug, Clone)]
pub struct Path<T>(pub T);

impl<T> FromRequestParts for Path<T>
where
    T: FromPath,
{
    type Rejection = PathError;

    async fn from_request_parts(
        parts: &mut Parts,
        _: &SharedAppState,
    ) -> Result<Self, Self::Rejection> {
        let params = parts.extensions.get::<PathInner>().unwrap();
        T::from_path(&params.0).map(Path)
    }
}

/// Trait for types that can be constructed from path parameters.
pub trait FromPath: Sized {
    fn from_path(params: &[(String, String)]) -> Result<Self, PathError>;
}

impl<T1> FromPath for T1
where
    T1: private::Marker + FromStr,
    T1::Err: Display,
{
    fn from_path(params: &[(String, String)]) -> Result<Self, PathError> {
        let mut params = params.iter();
        if params.len() != 1 {
            return Err(PathError::WrongNumberOfParameters {
                expected: 1,
                actual: params.len(),
            });
        }

        params
            .next()
            .ok_or(PathError::MissingPathParams)?
            .1
            .parse::<T1>()
            .map_err(|err| PathError::ParseError(err.to_string()))
    }
}

impl<T1, T2> FromPath for (T1, T2)
where
    T1: FromStr,
    T1::Err: Display,
    T2: FromStr,
    T2::Err: Display,
{
    fn from_path(params: &[(String, String)]) -> Result<Self, PathError> {
        let mut params = params.iter();
        if params.len() != 2 {
            return Err(PathError::WrongNumberOfParameters {
                expected: 2,
                actual: params.len(),
            });
        }
        Ok((
            params
                .next()
                .ok_or(PathError::MissingPathParams)?
                .1
                .parse::<T1>()
                .map_err(|err| PathError::ParseError(err.to_string()))?,
            params
                .next()
                .ok_or(PathError::MissingPathParams)?
                .1
                .parse::<T2>()
                .map_err(|err| PathError::ParseError(err.to_string()))?,
        ))
    }
}
