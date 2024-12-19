#![doc = include_str!("../README.md")]

pub(crate) mod chain;
pub use chain::DataChain;

pub(crate) mod decode;

pub mod errors;

pub(crate) mod request;
pub use request::{Request, RequestBuilder};

pub(crate) mod route;
pub use route::{Route, RouteBuilder};

pub(crate) mod routers;
pub use routers::authority::AuthorityId;
pub use routers::method::MethodId;
pub use routers::path::{PathConstraint, PathId, PathParameters};
pub use routers::{Match, MethodMatch, PathMatch, Router};

pub(crate) mod vec;
