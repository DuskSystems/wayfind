#![doc = include_str!("../README.md")]

pub(crate) mod constraints;
pub use constraints::Constraint;

pub(crate) mod decode;

pub mod errors;

pub(crate) mod expander;

pub(crate) mod node;
pub use node::search::{Match, Parameter};

pub(crate) mod path;
pub use path::Path;

pub(crate) mod parser;

pub(crate) mod router;
pub use router::Router;
