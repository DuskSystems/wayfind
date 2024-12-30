//! Error types for [`wayfind`](crate).
//!
//! All errors contain a user-friendly display method.

pub(crate) mod delete;
pub use delete::DeleteError;

pub(crate) mod encoding;
pub use encoding::{EncodingError, PercentEncodingError, PunycodeEncodingError};

pub(crate) mod insert;
pub use insert::InsertError;

pub(crate) mod request;
pub use request::RequestError;

pub(crate) mod route;
pub use route::RouteError;

pub(crate) mod search;
pub use search::SearchError;

pub use crate::router::authority::errors::{
    AuthorityConstraintError, AuthorityDeleteError, AuthorityInsertError, AuthoritySearchError,
    AuthorityTemplateError,
};
pub use crate::router::method::errors::{MethodDeleteError, MethodInsertError, MethodSearchError};
pub use crate::router::path::errors::{
    PathConstraintError, PathDeleteError, PathInsertError, PathSearchError, PathTemplateError,
};
