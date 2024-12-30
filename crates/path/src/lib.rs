#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use crate::vec::SortedVec;
use errors::{constraint::ConstraintError, DeleteError, InsertError, SearchError};
use id::PathIdGenerator;
use node::Node;
use parser::{Parser, Part};
use smallvec::{smallvec, SmallVec};
use state::RootState;
use std::{
    collections::HashMap,
    fmt::Display,
    net::{Ipv4Addr, Ipv6Addr},
    sync::Arc,
};

// FIXME: Actually make use of key, like in method router.

pub mod constraints;
pub mod delete;
pub mod display;
pub mod errors;
pub mod find;
pub mod id;
pub mod insert;
pub mod node;
pub mod optimize;
pub mod parser;
pub mod search;
pub mod state;
pub mod vec;

pub use constraints::PathConstraint;
pub use id::PathId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathData<'r> {
    pub id: PathId,
    pub route: &'r str,
    pub expanded: Option<Arc<str>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PathMatch<'r, 'p> {
    pub id: PathId,
    pub route: &'r str,
    pub expanded: Option<&'r str>,
    pub parameters: PathParameters<'r, 'p>,
}

pub type PathParameters<'r, 'p> = SmallVec<[(&'r str, &'p str); 4]>;

#[derive(Clone)]
pub struct StoredConstraint {
    pub type_name: &'static str,
    pub check: fn(&str) -> bool,
}

#[derive(Clone)]
pub struct PathRouter<'r> {
    pub root: Node<'r, RootState>,
    pub constraints: HashMap<&'r str, StoredConstraint>,
    pub id: PathIdGenerator,
}

impl<'r> PathRouter<'r> {
    #[must_use]
    pub fn new() -> Self {
        let mut router = Self {
            root: Node {
                state: RootState::new(),
                data: None,

                static_children: SortedVec::default(),
                dynamic_children: SortedVec::default(),
                dynamic_children_shortcut: false,
                wildcard_children: SortedVec::default(),
                wildcard_children_shortcut: false,
                end_wildcard_children: SortedVec::default(),

                priority: 0,
                needs_optimization: false,
            },
            constraints: HashMap::default(),
            id: PathIdGenerator::default(),
        };

        router.constraint::<u8>().unwrap();
        router.constraint::<u16>().unwrap();
        router.constraint::<u32>().unwrap();
        router.constraint::<u64>().unwrap();
        router.constraint::<u128>().unwrap();
        router.constraint::<usize>().unwrap();
        router.constraint::<i8>().unwrap();
        router.constraint::<i16>().unwrap();
        router.constraint::<i32>().unwrap();
        router.constraint::<i64>().unwrap();
        router.constraint::<i128>().unwrap();
        router.constraint::<isize>().unwrap();
        router.constraint::<f32>().unwrap();
        router.constraint::<f64>().unwrap();
        router.constraint::<bool>().unwrap();
        router.constraint::<Ipv4Addr>().unwrap();
        router.constraint::<Ipv6Addr>().unwrap();

        router
    }

    pub fn constraint<C: PathConstraint>(&mut self) -> Result<(), ConstraintError> {
        if let Some(existing) = self.constraints.get(C::NAME) {
            return Err(ConstraintError::DuplicateName {
                name: C::NAME,
                existing_type: existing.type_name,
                new_type: std::any::type_name::<C>(),
            });
        }

        self.constraints.insert(
            C::NAME,
            StoredConstraint {
                type_name: std::any::type_name::<C>(),
                check: C::check,
            },
        );

        Ok(())
    }

    pub fn conflicts(&self, route: &str) -> Result<Option<PathId>, DeleteError> {
        let parsed = Parser::new(route.as_bytes())?;

        // Check if any expansion conflicts
        for mut route_variant in parsed.routes {
            if let Some(data) = self.root.find(&mut route_variant) {
                return Ok(Some(data.id));
            }
        }

        Ok(None)
    }

    pub fn insert(&mut self, route: &'r str) -> Result<PathId, InsertError> {
        let mut parsed = Parser::new(route.as_bytes())?;

        // Check for invalid constraints.
        for route in &parsed.routes {
            for part in &route.parts {
                if let Part::Dynamic {
                    constraint: Some(name),
                    ..
                }
                | Part::Wildcard {
                    constraint: Some(name),
                    ..
                } = part
                {
                    if !self.constraints.contains_key(name.as_str()) {
                        return Err(InsertError::UnknownConstraint {
                            constraint: name.to_string(),
                        });
                    }
                }
            }
        }

        // Check for conflicts.
        let mut ids = vec![];
        for route in &parsed.routes {
            let raw = String::from_utf8_lossy(&route.raw);

            let Ok(Some(found)) = self.conflicts(&raw) else {
                continue;
            };

            ids.push(found);
        }

        ids.sort();
        ids.dedup();

        if !ids.is_empty() {
            let Some(first) = ids.first() else {
                unreachable!()
            };

            if ids.iter().any(|id| id != first) {
                return Err(InsertError::OverlappingRoutes { ids });
            }

            return Ok(*first);
        }

        // No conflicts, proceed with new insert.
        let id = self.id.generate();

        if parsed.routes.len() > 1 {
            for mut parsed_route in parsed.routes {
                let expanded = Arc::from(String::from_utf8_lossy(&parsed_route.raw));
                self.root.insert(
                    &mut parsed_route,
                    PathData {
                        id,
                        route,
                        expanded: Some(expanded),
                    },
                );
            }
        } else if let Some(parsed_route) = parsed.routes.first_mut() {
            self.root.insert(
                parsed_route,
                PathData {
                    id,
                    route,
                    expanded: None,
                },
            );
        };

        self.root.optimize();
        Ok(id)
    }

    pub fn find(&self, route: &str) -> Result<Option<PathId>, DeleteError> {
        let parsed = Parser::new(route.as_bytes())?;

        let mut id = None;
        let mut mismatch = None;
        let mut missing = false;

        for mut route_variant in parsed.routes {
            if let Some(data) = self.root.find(&mut route_variant) {
                if data.route != route {
                    mismatch = Some(data.route.to_owned());
                }

                if let Some(existing_id) = id {
                    if existing_id != data.id {
                        return Err(DeleteError::Mismatch {
                            route: route.to_owned(),
                            inserted: data.route.to_owned(),
                        });
                    }
                } else {
                    id = Some(data.id);
                }
            } else {
                missing = true;
            }
        }

        if let Some(inserted) = mismatch {
            return Err(DeleteError::Mismatch {
                route: route.to_owned(),
                inserted,
            });
        }

        if missing {
            return Ok(None);
        }

        Ok(id)
    }

    pub fn delete(&mut self, route: &str) {
        let Ok(parsed) = Parser::new(route.as_bytes()) else {
            return;
        };

        let Ok(data) = self.find(route) else {
            return;
        };

        if data.is_none() {
            return;
        }

        for mut expanded_route in parsed.routes {
            self.root.delete(&mut expanded_route);
        }

        self.root.optimize();
    }

    pub fn search<'p>(&'r self, path: &'p [u8]) -> Result<Option<PathMatch<'r, 'p>>, SearchError> {
        let mut parameters = smallvec![];
        let Some((data, _)) = self.root.search(path, &mut parameters, &self.constraints)? else {
            return Ok(None);
        };

        Ok(Some(PathMatch {
            id: data.id,
            route: data.route,
            expanded: data.expanded.as_deref(),
            parameters,
        }))
    }
}

impl Display for PathRouter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}
