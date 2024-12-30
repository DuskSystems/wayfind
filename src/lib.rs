#![doc = include_str!("../README.md")]

pub(crate) mod chain;
pub use chain::DataChain;

pub mod errors;

pub(crate) mod request;
pub use request::{Request, RequestBuilder};

pub(crate) mod route;
pub use route::{Route, RouteBuilder};

pub(crate) mod router;
pub use router::{AuthorityMatch, Match, MethodMatch, PathMatch, Router};
pub use wayfind_authority::AuthorityId;
pub use wayfind_method::MethodId;
pub use wayfind_path::{PathConstraint, PathId, PathParameters};
