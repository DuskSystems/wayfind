#![doc = include_str!("../README.md")]

pub(crate) mod decode;

pub mod errors;

pub(crate) mod node;

pub(crate) mod path;
pub use path::Path;

pub(crate) mod parser;

pub(crate) mod router;
pub use router::{Match, Parameter, Parameters, Router};
