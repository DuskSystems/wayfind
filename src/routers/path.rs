use crate::vec::SortedVec;
use errors::{constraint::PathConstraintError, PathDeleteError, PathInsertError, PathSearchError};
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

    pub fn constraint<C: PathConstraint>(&mut self) -> Result<(), PathConstraintError> {
        if let Some(existing) = self.constraints.get(C::NAME) {
            return Err(PathConstraintError::DuplicateName {
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

    pub(crate) fn insert(&mut self, route: &'r str) -> Result<PathId, PathInsertError> {
        let mut parsed = Parser::new(route.as_bytes())?;
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
                        return Err(PathInsertError::UnknownConstraint {
                            constraint: name.to_string(),
                        });
                    }
                }
            }
        }

        let mut id = self.id.next();

        if parsed.expanded {
            let mut ids = vec![];

            for mut parsed_route in parsed.routes {
                let expanded = Arc::from(String::from_utf8_lossy(&parsed_route.raw));
                let new = self.root.insert(
                    &mut parsed_route,
                    PathData {
                        id,
                        route,
                        expanded: Some(expanded),
                    },
                );

                ids.push(new);
            }

            // Incosistent IDs, try and clean up.
            // TODO
            let first = ids.first().unwrap();
            #[allow(clippy::manual_assert)]
            if ids.iter().any(|other| first != other) {
                return Err(PathInsertError::DuplicateRoute { id: *first });
            }

            id = *first;
        } else if let Some(parsed_route) = parsed.routes.first_mut() {
            let new = self.root.insert(
                parsed_route,
                PathData {
                    id,
                    route,
                    expanded: None,
                },
            );

            id = new;
        };

        self.root.optimize();

        Ok(id)
    }

    pub(crate) fn find(&self, route: &str) -> Result<Option<PathId>, PathDeleteError> {
        let mut parsed = Parser::new(route.as_bytes())?;

        let mut datas = vec![];
        if let Some(mut route) = parsed.routes.pop() {
            match self.root.find(&mut route) {
                Some(data) => {
                    datas.push(data);
                }
                None => return Ok(None),
            }
        }

        let Some(first_data) = datas.first() else {
            return Ok(None);
        };

        // FIXME: We should check that the IDs match here too? e.g. 2 conflicting but expanded routes?
        if parsed.expanded != first_data.expanded.is_some() {
            return Err(PathDeleteError::RouteMismatch {
                route: route.to_owned(),
                inserted: first_data.route.to_owned(),
            });
        }

        Ok(Some(first_data.id))
    }

    pub(crate) fn delete(&mut self, route: &str) -> Result<PathId, PathDeleteError> {
        let mut parsed = Parser::new(route.as_bytes())?;

        let data = if parsed.routes.len() > 1 {
            let mut data: Option<PathData<'r>> = None;
            let mut errors = vec![];

            for mut expanded_route in parsed.routes {
                match self.root.delete(&mut expanded_route, true) {
                    Ok(result) => data = Some(result),
                    Err(err) => errors.push(err),
                }
            }

            if !errors.is_empty() {
                errors.dedup();

                if errors.len() == 1 {
                    let error = errors.remove(0);
                    return Err(error);
                }

                return Err(PathDeleteError::Multiple(errors));
            }

            data.unwrap()
        } else {
            let route = parsed.routes.first_mut().unwrap();
            self.root.delete(route, false)?
        };

        self.root.optimize();
        Ok(data.id)
    }

    pub(crate) fn search<'p>(
        &'r self,
        path: &'p [u8],
    ) -> Result<Option<PathMatch<'r, 'p>>, PathSearchError> {
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
