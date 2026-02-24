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
//! use core::error::Error;
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
//! ### Dynamic
//!
//! Dynamic parameters can match any byte, **excluding** the path delimiter `/`.
//!
//! We support:
//! - whole segment parameters: `/<name>/`
//! - inline parameters: `/<name>.txt`
//!
//! #### Example
//!
//! ```rust
//! use core::error::Error;
//!
//! use wayfind::Router;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let mut router = Router::new();
//!     router.insert("/users/<id>", 1)?;
//!     router.insert("/users/<id>/files/<filename>.pdf", 2)?;
//!
//!     let search = router.search("/users/123").unwrap();
//!     assert_eq!(search.data, &1);
//!     assert_eq!(search.template, "/users/<id>");
//!     assert_eq!(search.parameters[0], ("id", "123"));
//!
//!     let search = router.search("/users/123/files/my.document.pdf").unwrap();
//!     assert_eq!(search.data, &2);
//!     assert_eq!(search.template, "/users/<id>/files/<filename>.pdf");
//!     assert_eq!(search.parameters[0], ("id", "123"));
//!     assert_eq!(search.parameters[1], ("filename", "my.document"));
//!
//!     Ok(())
//! }
//!```
//!
//! ### Wildcard
//!
//! Wildcard parameters can match any byte, **including** the path delimiter `/`.
//!
//! We support:
//! - inline wildcards: `/<*path>.html`
//! - mid-route wildcards: `/api/<*path>/help`
//! - end-route catch-all: `/<*catch_all>`
//!
//! #### Example
//!
//! ```rust
//! use core::error::Error;
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
//! When searching, each node tries its children in priority order:
//! 1. Static: `/users`
//! 2. Dynamic: `/<id>`
//! 3. Wildcard: `/<*path>`
//!
//! All parameters are greedy, consuming as much of the path as possible.
//!
//! ## Display
//!
//! The router can be printed as a tree, via a [`Display`](core::fmt::Display) implementation.

#![no_std]
extern crate alloc;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme_doctests {}

pub mod errors;

mod node;

mod parser;

mod router;
pub use router::{Match, Router};

mod state;
