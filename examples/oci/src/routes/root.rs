use http::StatusCode;

use crate::{
    extract::{method::Method, template::Template},
    response::IntoResponse,
};

pub async fn handle_root_get(
    Template(template): Template,
    Method(method): Method,
) -> impl IntoResponse {
    tracing::info!(
        oci = "end-1",
        template = ?template,
        method = ?method,
        "Handling request"
    );

    StatusCode::OK
}
