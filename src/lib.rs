#![doc = include_str!("../README.md")]

pub(crate) mod decode;

pub mod errors;

pub(crate) mod map;

pub(crate) mod request;
pub use request::{Request, RequestBuilder};

pub(crate) mod route;
pub use route::{Route, RouteBuilder};

pub(crate) mod routers;
pub use routers::path::{PathConstraint, PathParameters};
pub use routers::{Match, PathMatch, Router};

pub(crate) mod vec;
