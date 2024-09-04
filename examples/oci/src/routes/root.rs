use crate::{
    extract::{method::Method, route::Route},
    response::IntoResponse,
};
use http::StatusCode;

pub async fn handle_root_get(Route(route): Route, Method(method): Method) -> impl IntoResponse {
    tracing::info!(
        oci = "end-1",
        route = ?route,
        method = ?method,
        "Handling request"
    );

    StatusCode::OK
}
