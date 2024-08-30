use crate::{extract::method::Method, response::IntoResponse};
use http::StatusCode;

pub async fn handle_root_get(Method(method): Method) -> impl IntoResponse {
    tracing::info!(
        oci = "end-1",
        path = "/v2",
        method = %method,
        "Handling request"
    );

    StatusCode::OK
}
