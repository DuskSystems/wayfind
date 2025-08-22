//! # `wayfind`
//!
//! A speedy, flexible router.
//!
//! ## Showcase
//!
//! ```rust
//! use std::error::Error;
//!
//! use wayfind::{Constraint, Router};
//!
//! struct NumberConstraint;
//! impl Constraint for NumberConstraint {
//!     const NAME: &'static str = "number";
//!
//!     fn check(part: &str) -> bool {
//!         part.parse::<usize>().is_ok()
//!     }
//! }
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let mut router = Router::new();
//!     router.constraint::<NumberConstraint>()?;
//!
//!     router.insert("/pet(/)", 1)?;
//!     router.insert("/pet/findByStatus(/)", 2)?;
//!     router.insert("/pet/findByTags(/)", 3)?;
//!     router.insert("/pet/<pet>(/)", 4)?;
//!     router.insert("/pet/<petId:number>/uploadImage(/)", 5)?;
//!     router.insert("/store/inventory(/)", 6)?;
//!     router.insert("/store/order(/<orderId:number>)(/)", 7)?;
//!     router.insert("/user(/)", 8)?;
//!     router.insert("/user/createWithList(/)", 9)?;
//!     router.insert("/user/login(/)", 10)?;
//!     router.insert("/user/logout(/)", 11)?;
//!     router.insert("/user/<username>(/)", 12)?;
//!     router.insert("/<*catch_all>", 13)?;
//!
//!     let search = router.search("/pet").unwrap();
//!     assert_eq!(*search.data, 1);
//!
//!     let search = router.search("/pet/").unwrap();
//!     assert_eq!(*search.data, 1);
//!
//!     let search = router.search("/pet/123/uploadImage").unwrap();
//!     assert_eq!(*search.data, 5);
//!     assert_eq!(search.parameters[0], ("petId", "123"));
//!
//!     let search = router.search("/store/order").unwrap();
//!     assert_eq!(*search.data, 7);
//!
//!     let search = router.search("/store/order/456").unwrap();
//!     assert_eq!(*search.data, 7);
//!     assert_eq!(search.parameters[0], ("orderId", "456"));
//!
//!     let search = router.search("/user/alice").unwrap();
//!     assert_eq!(*search.data, 12);
//!     assert_eq!(search.parameters[0], ("username", "alice"));
//!
//!     let search = router.search("/unknown/path").unwrap();
//!     assert_eq!(*search.data, 13);
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
//! ## Constraints
//!
//! Constraints allow for custom logic to be injected into the routing process.
//!
//! They act as an escape-hatch for when you need to disambiguate routes.
//!
//! If a constraint check fails, the routing process will still continue, trying subsequent templates.
//!
//! Both dynamic and wildcard parameters support constraints.
//!
//! Examples:
//! - Dynamic constraint: `/<name:constraint>`
//! - Wildcard constraint: `/<*name:constraint>`
//!
//! ### Adding Constraints
//!
//! Constraints can be created using the [`Constraint`] trait.
//!
//! To register them, call the [`constraint`](crate::Router::constraint) function on router.
//!
//! ```rust
//! use std::error::Error;
//!
//! use wayfind::{Router, Constraint};
//!
//! struct NamespaceConstraint;
//! impl Constraint for NamespaceConstraint {
//!     const NAME: &'static str = "namespace";
//!
//!     fn check(part: &str) -> bool {
//!         part
//!             .split('/')
//!             .all(|part| {
//!                 part
//!                     .chars()
//!                     .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-')
//!             })
//!     }
//! }
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let mut router: Router<usize> = Router::new();
//!     router.constraint::<NamespaceConstraint>()?;
//!     router.insert("/<*user:namespace>", 1)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! Constraints must be registered first, before inserting any templates that references it.
//!
//! They cannot be removed once registered.
//!
//! ## Optional Groups
//!
//! Optional groups allow for parts of a route to be absent.
//!
//! They are commonly used for:
//! - optional IDs: `/users(/<id>)`
//! - optional trailing slashes: `/users(/)`
//! - optional file extensions: `/images/<name>(.<extension>)`
//!
//! They work via 'expanding' the route into equivalent, simplified routes.
//!
//! `/release/v<major>(.<minor>(.<patch>))` equivalent
//! - `/release/v<major>.<minor>.<patch>`
//! - `/release/v<major>.<minor>`
//! - `/release/v<major>`
//!
//! ### Example
//!
//! ```rust
//! use std::error::Error;
//!
//! use wayfind::Router;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let mut router = Router::new();
//!     router.insert("/users(/<id>)", 1)?;
//!     router.insert("/files/<*slug>/<file>(.<extension>)", 2)?;
//!
//!     let search = router.search("/users").unwrap();
//!     assert_eq!(*search.data, 1);
//!     assert_eq!(search.template, "/users(/<id>)");
//!     assert_eq!(search.expanded, Some("/users"));
//!
//!     let search = router.search("/users/123").unwrap();
//!     assert_eq!(*search.data, 1);
//!     assert_eq!(search.template, "/users(/<id>)");
//!     assert_eq!(search.expanded, Some("/users/<id>"));
//!     assert_eq!(search.parameters[0], ("id", "123"));
//!
//!     let search = router.search("/files/documents/folder/report.pdf").unwrap();
//!     assert_eq!(*search.data, 2);
//!     assert_eq!(search.template, "/files/<*slug>/<file>(.<extension>)");
//!     assert_eq!(search.expanded, Some("/files/<*slug>/<file>.<extension>"));
//!     assert_eq!(search.parameters[0], ("slug", "documents/folder"));
//!     assert_eq!(search.parameters[1], ("file", "report"));
//!     assert_eq!(search.parameters[2], ("extension", "pdf"));
//!
//!     let search = router.search("/files/documents/folder/readme").unwrap();
//!     assert_eq!(*search.data, 2);
//!     assert_eq!(search.template, "/files/<*slug>/<file>(.<extension>)");
//!     assert_eq!(search.expanded, Some("/files/<*slug>/<file>"));
//!     assert_eq!(search.parameters[0], ("slug", "documents/folder"));
//!     assert_eq!(search.parameters[1], ("file", "readme"));
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
//! In the event on an unexpected match, constraints can be used.
//!
//! ### 1. Kind
//!
//! From highest priority to lowest, we walk the current nodes children in this order:
//! - statics
//! - dynamics with constraints
//! - dynamics
//! - wildcards with constraints
//! - wildcards
//! - end wildcards with constraints
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
//! ## Escaping
//!
//! Special characters in the template can be escaped using a backslash.
//!
//! Examples:
//! - `/items/\<id\>`
//! - `/items/\(test\)`
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

mod constraints;
pub use constraints::Constraint;

pub mod errors;

mod node;

mod nodes;

mod parser;

mod router;
pub use router::{Match, Parameters, Router};

mod state;

mod storage;
