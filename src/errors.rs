//! Error types for [`wayfind`](crate).
//!
//! All errors contain a user-friendly display method.

pub(crate) mod constraint;
pub use constraint::ConstraintError;

pub(crate) mod delete;
pub use delete::DeleteError;

pub(crate) mod insert;
pub use insert::InsertError;

pub(crate) mod path;
pub use path::PathError;

pub(crate) mod route;
pub use route::RouteError;
