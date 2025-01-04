use bytes::Bytes;
use http::{
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    Response, StatusCode,
};
use http_body_util::Full;
use serde_json::json;
use thiserror::Error;

use crate::{
    extract::{body::Body, method::Method, path::Path, template::Template},
    response::{AppResponse, IntoResponse},
    state::{AppStateError, SharedAppState},
    types::digest::Digest,
};

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error(transparent)]
    AppStateError(#[from] AppStateError),
}

impl ManifestError {
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::AppStateError(err) => err.status_code(),
        }
    }
}

impl IntoResponse for ManifestError {
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

pub async fn handle_manifest_pull(
    state: SharedAppState,
    Template(template): Template,
    Method(method): Method,
    Path((name, reference)): Path<(String, String)>,
) -> Result<impl IntoResponse, ManifestError> {
    tracing::info!(
        oci = "end-3",
        template = ?template,
        method = ?method,
        name = ?name,
        reference = ?reference,
        "Handling request"
    );

    let manifest = state.get_manifest(&name, &reference)?;

    if method == http::Method::GET {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "application/vnd.oci.image.manifest.v1+json")
            .header(CONTENT_LENGTH, manifest.len().to_string())
            .body(Full::new(Bytes::from(manifest)))
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "application/vnd.oci.image.manifest.v1+json")
            .header(CONTENT_LENGTH, manifest.len().to_string())
            .body(Full::new(Bytes::new()))
            .unwrap())
    }
}

pub async fn handle_manifest_put(
    state: SharedAppState,
    Template(template): Template,
    Method(method): Method,
    Path((name, reference)): Path<(String, String)>,
    Body(body): Body,
) -> Result<impl IntoResponse, ManifestError> {
    tracing::info!(
        oci = "end-7",
        template = ?template,
        method = ?method,
        name = ?name,
        reference = ?reference,
        "Handling request"
    );

    let digest = Digest::sha256(&body);
    state.add_manifest(name, reference, &digest, &body, None);

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header(CONTENT_TYPE, "application/vnd.oci.image.manifest.v1+json")
        .header(CONTENT_LENGTH, body.len().to_string())
        .body(Full::new(Bytes::new()))
        .unwrap())
}

pub async fn handle_manifest_delete(
    state: SharedAppState,
    Template(template): Template,
    Method(method): Method,
    Path((name, reference)): Path<(String, String)>,
) -> Result<impl IntoResponse, ManifestError> {
    tracing::info!(
        oci = "end-7",
        template = ?template,
        method = ?method,
        name = ?name,
        reference = ?reference,
        "Handling request"
    );

    state.delete_manifest(&name, &reference)?;
    Ok(StatusCode::ACCEPTED)
}
