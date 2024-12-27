//! Error types for [`wayfind`](crate).
//!
//! All errors contain a user-friendly display method.

pub(crate) mod delete;
pub use delete::DeleteError;

pub(crate) mod encoding;
pub use encoding::EncodingError;

pub(crate) mod insert;
pub use insert::InsertError;

pub(crate) mod request;
pub use request::RequestError;

pub(crate) mod route;
pub use route::RouteError;

pub(crate) mod search;
pub use search::SearchError;

pub use crate::router::path::errors::{
    PathConstraintError, PathDeleteError, PathInsertError, PathRouteError, PathSearchError,
};

pub use crate::router::method::errors::{MethodDeleteError, MethodInsertError, MethodSearchError};
