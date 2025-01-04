#![doc = include_str!("../README.md")]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod constraints;
pub use constraints::Constraint;

mod delete;

mod display;

pub mod errors;

mod find;

mod insert;

mod node;

mod optimize;

mod parser;

mod router;
pub use router::{Match, Parameters, Router};

mod search;

mod state;

mod vec;
