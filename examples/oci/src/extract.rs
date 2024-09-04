use crate::{response::IntoResponse, state::SharedAppState};
use http::{request::Parts, Request};
use hyper::body::Incoming;
use std::convert::Infallible;
use std::future::Future;

pub mod body;
pub mod method;
pub mod path;
pub mod query;
pub mod route;

/// Type alias for the expected HTTP request.
pub type AppRequest = Request<Incoming>;

/// Marker types for controlling `FromRequest` implementations.
/// See `Handler` for more details.
mod private {
    #[derive(Debug, Clone, Copy)]
    pub enum ViaParts {}

    #[derive(Debug, Clone, Copy)]
    pub enum ViaRequest {}
}

/// Trait for extracting data from request parts (headers, method, URI, ...)
///
/// All `FromRequestParts` implementations can also be extracted via `FromRequest` too.
pub trait FromRequestParts: Sized {
    type Rejection: IntoResponse;

    fn from_request_parts(
        parts: &mut Parts,
        state: &SharedAppState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send;
}

impl FromRequestParts for SharedAppState {
    type Rejection = Infallible;

    async fn from_request_parts(
        _: &mut Parts,
        state: &SharedAppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(state.clone())
    }
}

/// Trait for extracting data from a full request, consuming it in the process.
pub trait FromRequest<M = private::ViaRequest>: Sized {
    type Rejection: IntoResponse;

    fn from_request(
        req: AppRequest,
        state: &SharedAppState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send;
}

impl<T> FromRequest<private::ViaParts> for T
where
    T: FromRequestParts,
{
    type Rejection = <Self as FromRequestParts>::Rejection;

    async fn from_request(
        req: AppRequest,
        state: &SharedAppState,
    ) -> Result<Self, Self::Rejection> {
        let (mut parts, _) = req.into_parts();
        T::from_request_parts(&mut parts, state).await
    }
}

impl FromRequest<()> for AppRequest {
    type Rejection = Infallible;

    async fn from_request(req: AppRequest, _: &SharedAppState) -> Result<Self, Self::Rejection> {
        Ok(req)
    }
}
