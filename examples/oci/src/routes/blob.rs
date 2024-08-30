use crate::{
    extract::{body::Body, method::Method, path::Path, query::Query},
    response::{AppResponse, IntoResponse},
    state::{AppStateError, SharedAppState},
    types::digest::{Digest, DigestError},
};
use bytes::Bytes;
use http::{
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    Response, StatusCode,
};
use http_body_util::Full;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum BlobError {
    #[error(transparent)]
    AppStateError(#[from] AppStateError),

    #[error(transparent)]
    DigestError(#[from] DigestError),

    #[error("Digest mismatch. Expected: {expected} - Actual: {actual}")]
    DigestMismatch { expected: String, actual: String },

    #[error("Missing digest parameter")]
    MissingDigestParameter,

    #[error("Invalid upload reference")]
    InvalidUploadReference,
}

impl BlobError {
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::AppStateError(err) => err.status_code(),
            Self::DigestError(err) => err.status_code(),
            Self::DigestMismatch { .. }
            | Self::MissingDigestParameter
            | Self::InvalidUploadReference => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for BlobError {
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

pub async fn handle_blob_pull(
    state: SharedAppState,
    Method(method): Method,
    Path((name, digest)): Path<(String, String)>,
) -> Result<impl IntoResponse, BlobError> {
    tracing::info!(
        oci = "end-2",
        path = "/v2/{*name:name}/blobs/{digest}",
        method = %method,
        name = %name,
        digest = %digest,
        "Handling request"
    );

    let digest = Digest::try_from(digest.as_ref())?;
    let blob = state.get_blob(&name, &digest)?;

    if method == http::Method::GET {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "application/octet-stream")
            .header(CONTENT_LENGTH, blob.len().to_string())
            .body(Full::new(Bytes::from(blob)))
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "application/octet-stream")
            .header(CONTENT_LENGTH, blob.len().to_string())
            .body(Full::new(Bytes::new()))
            .unwrap())
    }
}

pub async fn handle_blob_push_post(
    state: SharedAppState,
    Method(method): Method,
    Path(name): Path<String>,
    Query(query): Query,
    Body(body): Body,
) -> Result<impl IntoResponse, BlobError> {
    if let Some(digest) = query.get("digest") {
        tracing::info!(
            oci = "end-4b",
            path = "/v2/{*name:name}/blobs/uploads?digest={digest}",
            method = %method,
            name = %name,
            digest = %digest,
            "Handling request"
        );

        let digest = Digest::try_from(digest.as_ref())?;
        let actual_digest = Digest::sha256(&body);
        if actual_digest != digest {
            return Err(BlobError::DigestMismatch {
                expected: digest.to_string(),
                actual: actual_digest.to_string(),
            });
        }

        state.add_blob(name.clone(), digest.clone(), body.to_vec());

        Ok(Response::builder()
            .status(StatusCode::CREATED)
            .header("Location", format!("/v2/{name}/blobs/{digest}"))
            .body(Full::new(Bytes::new()))
            .unwrap())
    } else {
        tracing::info!(
            oci = "end-4a",
            path = "/v2/{*name:name}/blobs/uploads",
            method = %method,
            name = %name,
            "Handling request"
        );

        let upload_id = state.start_upload(name.clone());

        Ok(Response::builder()
            .status(StatusCode::ACCEPTED)
            .header("Location", format!("/v2/{name}/blobs/uploads/{upload_id}"))
            .body(Full::new(Bytes::new()))
            .unwrap())
    }
}

pub async fn handle_blob_push_put(
    state: SharedAppState,
    Method(method): Method,
    Path((name, reference)): Path<(String, String)>,
    Query(query): Query,
    Body(body): Body,
) -> Result<impl IntoResponse, BlobError> {
    tracing::info!(
        oci = "end-6",
        path = "/v2/{*name:name}/blobs/uploads/{reference}",
        method = %method,
        name = %name,
        reference = %reference,
        "Handling request"
    );

    let digest = query
        .get("digest")
        .ok_or(BlobError::MissingDigestParameter)?;
    let digest = Digest::try_from(digest.as_ref())?;

    let upload_id = Uuid::parse_str(&reference).map_err(|_| BlobError::InvalidUploadReference)?;
    state.update_upload(upload_id, &body)?;
    state.complete_upload(upload_id, digest.clone())?;

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Location", format!("/v2/{name}/blobs/{digest}"))
        .header(CONTENT_TYPE, "application/octet-stream")
        .body(Full::new(Bytes::new()))
        .unwrap())
}

pub async fn handle_blob_delete(
    state: SharedAppState,
    Method(method): Method,
    Path((name, digest)): Path<(String, String)>,
) -> Result<impl IntoResponse, BlobError> {
    tracing::info!(
        oci = "end-10",
        path = "/v2/{*name:name}/blobs/{digest}",
        method = %method,
        name = %name,
        digest = %digest,
        "Handling request"
    );

    let digest = Digest::try_from(digest.as_ref())?;
    state.delete_blob(&name, &digest)?;
    Ok(StatusCode::ACCEPTED)
}
