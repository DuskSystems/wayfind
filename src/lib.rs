#![doc = include_str!("../README.md")]

pub(crate) mod constraints;
pub use constraints::Constraint;

pub(crate) mod decode;

pub mod errors;

pub(crate) mod node;

pub(crate) mod parser;

pub(crate) mod routable;
pub use routable::{Routable, RoutableBuilder};

pub(crate) mod router;
pub use router::{Match, Parameter, Router};
