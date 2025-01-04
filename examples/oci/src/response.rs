use std::convert::Infallible;

use bytes::Bytes;
use http::{Response, StatusCode};
use http_body_util::Full;

/// Represents a HTTP response with a full body of bytes.
pub type AppResponse = Response<Full<Bytes>>;

/// Trait for types that can be converted into an `AppResponse`.
pub trait IntoResponse {
    fn into_response(self) -> AppResponse;
}

impl IntoResponse for AppResponse {
    fn into_response(self) -> AppResponse {
        self
    }
}

impl IntoResponse for Infallible {
    fn into_response(self) -> AppResponse {
        unreachable!()
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> AppResponse {
        Response::builder()
            .status(self)
            .body(Full::new(Bytes::new()))
            .unwrap()
    }
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: IntoResponse,
    E: IntoResponse,
{
    fn into_response(self) -> AppResponse {
        match self {
            Ok(response) => response.into_response(),
            Err(err) => err.into_response(),
        }
    }
}
