#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use errors::{PathConstraintError, PathDeleteError, PathInsertError};
use id::PathIdGenerator;
use parser::ParsedPath;
use smallvec::{smallvec, SmallVec};
use std::{
    collections::HashMap,
    fmt::Display,
    net::{Ipv4Addr, Ipv6Addr},
    str::Utf8Error,
    sync::Arc,
};
use wayfind_storage::Storage;
use wayfind_tree::{
    node::{Config, Data, Node},
    parser::Part,
    search::StoredConstraint,
    state::RootState,
    vec::SortedVec,
};

pub mod constraints;
pub mod errors;
pub mod id;
pub mod parser;

pub use constraints::PathConstraint;
pub use id::PathId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathConfig;

impl Config for PathConfig {
    type Data = PathData;

    const DELIMITER: u8 = b'/';
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathData {
    pub id: PathId,
    pub route: Arc<str>,
    pub expanded: Option<Arc<str>>,
}

impl Data for PathData {
    fn id(&self) -> Option<usize> {
        self.id.0
    }

    fn priority(&self) -> usize {
        self.expanded.as_ref().map_or_else(
            || self.route.len() + (self.route.bytes().filter(|&b| b == b'/').count() * 100),
            |expanded| expanded.len() + (expanded.bytes().filter(|&b| b == b'/').count() * 100),
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct PathMatch<'r, 'p> {
    pub id: PathId,
    pub route: Arc<str>,
    pub expanded: Option<Arc<str>>,
    pub parameters: PathParameters<'r, 'p>,
}

pub type PathParameters<'r, 'p> = SmallVec<[(&'r str, &'p str); 4]>;

#[derive(Clone)]
pub struct PathRouter {
    pub root: Node<PathConfig, RootState>,
    pub constraints: HashMap<Arc<str>, StoredConstraint>,
    pub id: PathIdGenerator,
}

impl PathRouter {
    #[must_use]
    pub fn new() -> Self {
        let mut router = Self {
            root: Node {
                state: RootState::new(),
                data: Storage::default(),

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

    pub fn constraint<C: PathConstraint>(&mut self) -> Result<(), PathConstraintError> {
        if let Some(existing) = self.constraints.get(C::NAME) {
            return Err(PathConstraintError::DuplicateName {
                name: C::NAME,
                existing_type: existing.type_name,
                new_type: std::any::type_name::<C>(),
            });
        }

        self.constraints.insert(
            C::NAME.into(),
            StoredConstraint {
                type_name: std::any::type_name::<C>(),
                check: C::check,
            },
        );

        Ok(())
    }

    pub fn conflicts(
        &self,
        key: Option<usize>,
        path: &ParsedPath,
    ) -> Result<Option<PathId>, PathDeleteError> {
        // Check if any expansion conflicts
        for route in &path.routes {
            if let Some(data) = self.root.find(key, route) {
                return Ok(Some(data.id));
            }
        }

        Ok(None)
    }

    pub fn insert(
        &mut self,
        key: Option<usize>,
        path: &ParsedPath,
    ) -> Result<PathId, PathInsertError> {
        // Check for invalid constraints.
        for route in &path.routes {
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
                        return Err(PathInsertError::UnknownConstraint {
                            constraint: name.to_string(),
                        });
                    }
                }
            }
        }

        // Check for conflicts.
        let mut ids = vec![];
        for route in &path.routes {
            // FIXME
            let raw = ParsedPath::new(&route.raw).unwrap();
            let Ok(Some(found)) = self.conflicts(key, &raw) else {
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
                return Err(PathInsertError::Overlapping {
                    ids: ids.into_iter().filter_map(|id| id.0).collect(),
                });
            }

            return Ok(*first);
        }

        // No conflicts, proceed with new insert.
        let id = self.id.generate();

        if path.routes.len() > 1 {
            for parsed_route in &path.routes {
                self.root.insert(
                    key,
                    parsed_route,
                    PathData {
                        id,
                        route: Arc::clone(&path.input),
                        expanded: Some(Arc::clone(&parsed_route.raw)),
                    },
                );
            }
        } else if let Some(parsed_route) = path.routes.first() {
            self.root.insert(
                key,
                parsed_route,
                PathData {
                    id,
                    route: Arc::clone(&path.input),
                    expanded: None,
                },
            );
        };

        self.root.optimize();
        Ok(id)
    }

    pub fn find(
        &self,
        key: Option<usize>,
        path: &ParsedPath,
    ) -> Result<Option<PathId>, PathDeleteError> {
        let mut id = None;
        let mut mismatch = None;
        let mut missing = false;

        for route in &path.routes {
            if let Some(data) = self.root.find(key, route) {
                if data.route != path.input {
                    mismatch = Some(Arc::clone(&data.route));
                }

                if let Some(existing_id) = id {
                    if existing_id != data.id {
                        return Err(PathDeleteError::Mismatch {
                            path: path.input.to_string(),
                            inserted: data.route.to_string(),
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
            return Err(PathDeleteError::Mismatch {
                path: path.input.to_string(),
                inserted: inserted.to_string(),
            });
        }

        if missing {
            return Ok(None);
        }

        Ok(id)
    }

    pub fn delete(&mut self, key: Option<usize>, path: &ParsedPath) {
        let Ok(data) = self.find(key, path) else {
            return;
        };

        if data.is_none() {
            return;
        }

        for expanded in &path.routes {
            self.root.delete(key, expanded);
        }

        self.root.optimize();
    }

    pub fn search<'r, 'p>(
        &'r self,
        key: Option<usize>,
        path: &'p [u8],
    ) -> Result<Option<PathMatch<'r, 'p>>, Utf8Error>
    where
        'p: 'r,
    {
        let mut parameters: SmallVec<[(&str, &[u8]); 4]> = smallvec![];
        let Some((data, _)) = self
            .root
            .search(key, path, &mut parameters, &self.constraints)
        else {
            return Ok(None);
        };

        let parameters = parameters
            .into_iter()
            .map(|(name, value)| std::str::from_utf8(value).map(|value| (name, value)))
            .collect::<Result<_, Utf8Error>>()?;

        Ok(Some(PathMatch {
            id: data.id,
            route: Arc::clone(&data.route),
            expanded: data.expanded.as_ref().map(Arc::clone),
            parameters,
        }))
    }
}

impl Display for PathRouter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}
