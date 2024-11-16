#![doc = include_str!("../README.md")]

pub(crate) mod chain;
pub use chain::DataChain;

pub(crate) mod decode;

pub mod errors;

pub(crate) mod request;
pub use request::{Request, RequestBuilder};

pub(crate) mod route;
pub use route::{Route, RouteBuilder};

pub(crate) mod router;
pub use router::method::MethodId;
pub use router::path::{PathConstraint, PathId, PathParameters};
pub use router::{Match, MethodMatch, PathMatch, Router};

pub(crate) mod vec;
