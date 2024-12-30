//! Error types for [`wayfind`](crate).
//!
//! All errors contain a user-friendly display method.

pub(crate) mod delete;
pub use delete::DeleteError;

pub(crate) mod encoding;
pub use encoding::EncodingError;
pub use wayfind_percent::errors::DecodingError as PercentDecodingError;
pub use wayfind_punycode::errors::DecodingError as PunycodeDecodingError;

pub(crate) mod insert;
pub use insert::InsertError;

pub(crate) mod request;
pub use request::RequestError;

pub(crate) mod route;
pub use route::RouteError;

pub(crate) mod search;
pub use search::SearchError;

pub use wayfind_authority::errors::{
    ConstraintError as AuthorityConstraintError, DeleteError as AuthorityDeleteError,
    InsertError as AuthorityInsertError, SearchError as AuthoritySearchError,
    TemplateError as AuthorityTemplateError,
};
pub use wayfind_method::errors::{
    DeleteError as MethodDeleteError, InsertError as MethodInsertError,
    SearchError as MethodSearchError,
};
pub use wayfind_path::errors::{
    ConstraintError as PathConstraintError, DeleteError as PathDeleteError,
    InsertError as PathInsertError, SearchError as PathSearchError,
    TemplateError as PathTemplateError,
};
