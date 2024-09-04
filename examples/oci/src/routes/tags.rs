use crate::{
    extract::{method::Method, path::Path, route::Route},
    response::{AppResponse, IntoResponse},
    state::{AppStateError, SharedAppState},
};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, Response, StatusCode};
use http_body_util::Full;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error(transparent)]
    AppStateError(#[from] AppStateError),
}

impl RegistryError {
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::AppStateError(err) => err.status_code(),
        }
    }
}

impl IntoResponse for RegistryError {
    fn into_response(self) -> AppResponse {
        let status = self.status_code();
        let body = json!({
            "error": {
                "code": status.as_u16(),
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

pub async fn handle_tags_get(
    state: SharedAppState,
    Route(route): Route,
    Method(method): Method,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, RegistryError> {
    tracing::info!(
        oci = "end-8a",
        route = ?route,
        method = ?method,
        name = ?name,
        "Handling request"
    );

    let tags = state.list_tags(&name)?;
    let response = json!({
        "name": name,
        "tags": tags
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(Full::new(Bytes::from(response.to_string())))
        .unwrap())
}
