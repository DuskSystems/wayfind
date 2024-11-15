#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub(crate) mod decode;

pub mod errors;

pub(crate) mod id;

pub(crate) mod request;
pub use request::{Request, RequestBuilder};

pub(crate) mod route;
pub use route::{Route, RouteBuilder};

pub(crate) mod routers;
pub use routers::path::{PathConstraint, PathMatch, PathParameters};
pub use routers::{Match, Router};

pub(crate) mod storage;
