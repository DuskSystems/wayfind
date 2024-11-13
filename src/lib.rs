#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub(crate) mod constraints;
pub use constraints::Constraint;

pub(crate) mod decode;

pub mod errors;

pub(crate) mod node;

pub(crate) mod path;
pub use path::Path;

pub(crate) mod parser;

pub(crate) mod routable;
pub use routable::{Routable, RoutableBuilder};

pub(crate) mod router;
pub use router::{Match, Parameters, Router};

pub(crate) mod state;
