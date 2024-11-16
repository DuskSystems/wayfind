#![doc = include_str!("../README.md")]

pub(crate) mod decode;

pub mod errors;

pub(crate) mod map;

pub(crate) mod request;
pub use request::{Request, RequestBuilder};

pub(crate) mod route;
pub use route::{Route, RouteBuilder};

pub(crate) mod routers;
pub use routers::method::MethodId;
pub use routers::path::{PathConstraint, PathId, PathParameters};
pub use routers::{Match, MethodMatch, PathMatch, Router};
