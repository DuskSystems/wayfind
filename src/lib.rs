//! # `wayfind`
//!
//! A speedy, flexible router.
//!
//! ## Showcase
//!
//! ```rust
//! use std::error::Error;
//!
//! use wayfind::Router;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let mut router = Router::new();
//!     router.insert("/pet", 1)?;
//!     router.insert("/pet/", 2)?;
//!     router.insert("/pet/findByStatus", 3)?;
//!     router.insert("/pet/findByTags", 4)?;
//!     router.insert("/pet/<pet>", 5)?;
//!     router.insert("/pet/<petId>/uploadImage", 6)?;
//!     router.insert("/store/inventory", 7)?;
//!     router.insert("/store/order", 8)?;
//!     router.insert("/store/order/<orderId>", 9)?;
//!     router.insert("/user", 10)?;
//!     router.insert("/user/createWithList", 11)?;
//!     router.insert("/user/login", 12)?;
//!     router.insert("/user/logout", 13)?;
//!     router.insert("/user/<username>", 14)?;
//!     router.insert("/<*catch_all>", 15)?;
//!
//!     let search = router.search("/pet").unwrap();
//!     assert_eq!(*search.data, 1);
//!
//!     let search = router.search("/pet/123/uploadImage").unwrap();
//!     assert_eq!(*search.data, 6);
//!     assert_eq!(search.parameters[0], ("petId", "123"));
//!
//!     let search = router.search("/store/order").unwrap();
//!     assert_eq!(*search.data, 8);
//!
//!     let search = router.search("/store/order/456").unwrap();
//!     assert_eq!(*search.data, 9);
//!     assert_eq!(search.parameters[0], ("orderId", "456"));
//!
//!     let search = router.search("/user/alice").unwrap();
//!     assert_eq!(*search.data, 14);
//!     assert_eq!(search.parameters[0], ("username", "alice"));
//!
//!     let search = router.search("/unknown/path").unwrap();
//!     assert_eq!(*search.data, 15);
//!     assert_eq!(search.parameters[0], ("catch_all", "unknown/path"));
//!
//!     Ok(())
//! }
//! ```
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
//!     assert_eq!(*search.data, 1);
//!     assert_eq!(search.template, "/hello");
//!
//!     let search = router.search("/hello/world").unwrap();
//!     assert_eq!(*search.data, 2);
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
//!     assert_eq!(*search.data, 1);
//!     assert_eq!(search.template, "/users/<id>");
//!     assert_eq!(search.parameters[0], ("id", "123"));
//!
//!     let search = router.search("/users/123/files/my.document.pdf").unwrap();
//!     assert_eq!(*search.data, 2);
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
//!     assert_eq!(*search.data, 1);
//!     assert_eq!(search.template, "/files/<*slug>/delete");
//!     assert_eq!(search.parameters[0], ("slug", "documents/reports/annual.pdf"));
//!
//!     let search = router.search("/any/other/path").unwrap();
//!     assert_eq!(*search.data, 2);
//!     assert_eq!(search.template, "/<*catch_all>");
//!     assert_eq!(search.parameters[0], ("catch_all", "any/other/path"));
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Priority
//!
//! Routes are matched using a hierarchical priority system.
//!
//! It is an imperfect process, but for most scenarios, is unlikely to cause problems.
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
//! When comparing routes at the same node level and of the same kind, we use two factors:
//!
//! 1. Depth - routes with more slashes take precedence
//! 2. Length - if depths are equal, longer routes take precedence
//!
//! ## Display
//!
//! The router can be printed as a tree, via a [`Display`](std::fmt::Display) implementation.
//!
//! All nodes within the tree that can be matched against will be denoted with an asterisk.
//!
//! This doesn't handle split multi-byte characters well.
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
