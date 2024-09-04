use crate::{
    extract::{AppRequest, FromRequest, FromRequestParts},
    response::{AppResponse, IntoResponse},
    state::SharedAppState,
};
use std::{future::Future, pin::Pin};

/// Trait for request handlers in the application.
///
/// Allows for flexible handler signatures with different numbers of parameters.
/// Only the final parameter in a handler can consume the request body.
pub trait Handler<T>: Clone + Send + Sized + 'static {
    type Future: Future<Output = AppResponse> + Send + 'static;

    fn call(self, req: AppRequest, state: SharedAppState) -> Self::Future;
}

impl<F, Fut, Res> Handler<()> for F
where
    F: FnOnce() -> Fut + Clone + Send + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoResponse,
{
    type Future = Pin<Box<dyn Future<Output = AppResponse> + Send>>;

    fn call(self, _: AppRequest, _: SharedAppState) -> Self::Future {
        Box::pin(async move { self().await.into_response() })
    }
}

impl<F, Fut, Res, M, T1> Handler<(M, T1)> for F
where
    F: FnOnce(T1) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoResponse,
    T1: FromRequest<M> + Send,
{
    type Future = Pin<Box<dyn Future<Output = AppResponse> + Send>>;

    fn call(self, req: AppRequest, state: SharedAppState) -> Self::Future {
        Box::pin(async move {
            let (parts, body) = req.into_parts();
            let req = AppRequest::from_parts(parts, body);

            let t1 = match T1::from_request(req, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            self(t1).await.into_response()
        })
    }
}

impl<F, Fut, Res, M, T1, T2> Handler<(M, T1, T2)> for F
where
    F: FnOnce(T1, T2) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoResponse,
    T1: FromRequestParts + Send,
    T2: FromRequest<M> + Send,
{
    type Future = Pin<Box<dyn Future<Output = AppResponse> + Send>>;

    fn call(self, req: AppRequest, state: SharedAppState) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();

            let t1 = match T1::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let req = AppRequest::from_parts(parts, body);

            let t2 = match T2::from_request(req, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            self(t1, t2).await.into_response()
        })
    }
}

impl<F, Fut, Res, M, T1, T2, T3> Handler<(M, T1, T2, T3)> for F
where
    F: FnOnce(T1, T2, T3) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoResponse,
    T1: FromRequestParts + Send,
    T2: FromRequestParts + Send,
    T3: FromRequest<M> + Send,
{
    type Future = Pin<Box<dyn Future<Output = AppResponse> + Send>>;

    fn call(self, req: AppRequest, state: SharedAppState) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();

            let t1 = match T1::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let t2 = match T2::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let req = AppRequest::from_parts(parts, body);

            let t3 = match T3::from_request(req, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            self(t1, t2, t3).await.into_response()
        })
    }
}

impl<F, Fut, Res, M, T1, T2, T3, T4> Handler<(M, T1, T2, T3, T4)> for F
where
    F: FnOnce(T1, T2, T3, T4) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoResponse,
    T1: FromRequestParts + Send,
    T2: FromRequestParts + Send,
    T3: FromRequestParts + Send,
    T4: FromRequest<M> + Send,
{
    type Future = Pin<Box<dyn Future<Output = AppResponse> + Send>>;

    fn call(self, req: AppRequest, state: SharedAppState) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();

            let t1 = match T1::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let t2 = match T2::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let t3 = match T3::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let req = AppRequest::from_parts(parts, body);

            let t4 = match T4::from_request(req, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            self(t1, t2, t3, t4).await.into_response()
        })
    }
}

impl<F, Fut, Res, M, T1, T2, T3, T4, T5> Handler<(M, T1, T2, T3, T4, T5)> for F
where
    F: FnOnce(T1, T2, T3, T4, T5) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoResponse,
    T1: FromRequestParts + Send,
    T2: FromRequestParts + Send,
    T3: FromRequestParts + Send,
    T4: FromRequestParts + Send,
    T5: FromRequest<M> + Send,
{
    type Future = Pin<Box<dyn Future<Output = AppResponse> + Send>>;

    fn call(self, req: AppRequest, state: SharedAppState) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();

            let t1 = match T1::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let t2 = match T2::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let t3 = match T3::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let t4 = match T4::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let req = AppRequest::from_parts(parts, body);

            let t5 = match T5::from_request(req, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            self(t1, t2, t3, t4, t5).await.into_response()
        })
    }
}

impl<F, Fut, Res, M, T1, T2, T3, T4, T5, T6> Handler<(M, T1, T2, T3, T4, T5, T6)> for F
where
    F: FnOnce(T1, T2, T3, T4, T5, T6) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoResponse,
    T1: FromRequestParts + Send,
    T2: FromRequestParts + Send,
    T3: FromRequestParts + Send,
    T4: FromRequestParts + Send,
    T5: FromRequestParts + Send,
    T6: FromRequest<M> + Send,
{
    type Future = Pin<Box<dyn Future<Output = AppResponse> + Send>>;

    fn call(self, req: AppRequest, state: SharedAppState) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();

            let t1 = match T1::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let t2 = match T2::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let t3 = match T3::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let t4 = match T4::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let t5 = match T5::from_request_parts(&mut parts, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            let req = AppRequest::from_parts(parts, body);

            let t6 = match T6::from_request(req, &state).await {
                Ok(value) => value,
                Err(rejection) => return rejection.into_response(),
            };

            self(t1, t2, t3, t4, t5, t6).await.into_response()
        })
    }
}
