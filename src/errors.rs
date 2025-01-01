//! Error types for [`wayfind`](crate).
//!
//! All errors contain a user-friendly display method.

pub(crate) mod delete;
pub use delete::DeleteError;

pub(crate) mod encoding;
pub use encoding::EncodingError;
pub use wayfind_percent::errors::PercentDecodingError;
pub use wayfind_punycode::errors::PunycodeDecodingError;

pub(crate) mod insert;
pub use insert::InsertError;

pub(crate) mod request;
pub use request::RequestError;

pub(crate) mod route;
pub use route::RouteError;

pub(crate) mod search;
pub use search::SearchError;

pub use wayfind_authority::errors::*;
pub use wayfind_method::errors::*;
pub use wayfind_path::errors::*;
