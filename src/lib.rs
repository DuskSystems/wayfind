//! # `wayfind`
//!
//! A speedy, flexible router.
//!
//! ## Syntax
//!
//! ### Static
//!
//! Static template parts are treated as-is.
//!
//! - No percent-decoding occurs.
//! - Templates are case-sensitive.
//!
//! The leading static part of a template must start with a `/`.
//!
//! #### Example
//!
//! ```rust
//! use std::error::Error;
//!
//! use wayfind::Router;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let mut router = Router::new();
//!     router.insert("/hello", 1)?;
//!     router.insert("/hello/world", 2)?;
//!
//!     let search = router.search("/hello").unwrap();
//!     assert_eq!(search.data, &1);
//!     assert_eq!(search.template, "/hello");
//!
//!     let search = router.search("/hello/world").unwrap();
//!     assert_eq!(search.data, &2);
//!     assert_eq!(search.template, "/hello/world");
//!
//!     let search = router.search("/world");
//!     assert!(search.is_none());
//!
//!     Ok(())
//! }
//!```
//!
//! ### Dynamic Routing
//!
//! Dynamic parameters can match any byte, **excluding** the path delimiter `/`.
//!
//! We support both:
//! - whole segment parameters: `/<name>/`
//! - inline parameters: `/<year>-<month>-<day>/`
//!
//! Dynamic parameters are greedy in nature, similar to a regex `.*`, and will attempt to match as many bytes as possible.
//!
//! #### Example
//!
//! ```rust
//! use std::error::Error;
//!
//! use wayfind::Router;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let mut router = Router::new();
//!     router.insert("/users/<id>", 1)?;
//!     router.insert("/users/<id>/files/<filename>.<extension>", 2)?;
//!
//!     let search = router.search("/users/123").unwrap();
//!     assert_eq!(search.data, &1);
//!     assert_eq!(search.template, "/users/<id>");
//!     assert_eq!(search.parameters[0], ("id", "123"));
//!
//!     let search = router.search("/users/123/files/my.document.pdf").unwrap();
//!     assert_eq!(search.data, &2);
//!     assert_eq!(search.template, "/users/<id>/files/<filename>.<extension>");
//!     assert_eq!(search.parameters[0], ("id", "123"));
//!     assert_eq!(search.parameters[1], ("filename", "my.document"));
//!     assert_eq!(search.parameters[2], ("extension", "pdf"));
//!
//!     Ok(())
//! }
//!```
//!
//! ### Wildcard Routing
//!
//! Wildcard parameters can match any byte, **including** the path delimiter `/`.
//!
//! We support both:
//! - inline wildcards: `/<*path>.html`
//! - mid-route wildcards: `/api/<*path>/help`
//! - end-route catch-all: `/<*catch_all>`
//!
//! Like dynamic parameters, wildcard parameters are also greedy in nature.
//!
//! #### Example
//!
//! ```rust
//! use std::error::Error;
//!
//! use wayfind::Router;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let mut router = Router::new();
//!     router.insert("/files/<*slug>/delete", 1)?;
//!     router.insert("/<*catch_all>", 2)?;
//!
//!     let search = router.search("/files/documents/reports/annual.pdf/delete").unwrap();
//!     assert_eq!(search.data, &1);
//!     assert_eq!(search.template, "/files/<*slug>/delete");
//!     assert_eq!(search.parameters[0], ("slug", "documents/reports/annual.pdf"));
//!
//!     let search = router.search("/any/other/path").unwrap();
//!     assert_eq!(search.data, &2);
//!     assert_eq!(search.template, "/<*catch_all>");
//!     assert_eq!(search.parameters[0], ("catch_all", "any/other/path"));
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Priority
//!
//! Templates are matched using a hierarchical priority system.
//!
//! ### 1. Kind
//!
//! From highest priority to lowest, we walk the current nodes children in this order:
//! - statics
//! - dynamics
//! - wildcards
//! - end wildcards
//!
//! In the event of multiple children of the same type, we walk them in alphabetical order, to ensure order remains predictable.
//!
//! ### 2. Structure
//!
//! When comparing templates at the same node level and of the same kind, we prefer the "more specific" template.
//!
//! ## Display
//!
//! The router can be printed as a tree, via a [`Display`](std::fmt::Display) implementation.
//!
//! ## Error Messages
//!
//! Where possible, we try to provide user-friendly error messages for all of our error enums.
//!
//! See [`Errors`](errors) for examples.

#![no_std]

extern crate alloc;

pub mod errors;

mod node;

mod parser;

mod router;
pub use router::{Match, Parameters, Router};

mod state;

mod storage;
