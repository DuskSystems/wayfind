use crate::vec::SortedVec;
use errors::{
    AuthorityConstraintError, AuthorityDeleteError, AuthorityInsertError, AuthoritySearchError,
};
use id::AuthorityIdGenerator;
use node::Node;
use parser::{Parser, Part};
use smallvec::{smallvec, SmallVec};
use state::RootState;
use std::{collections::HashMap, fmt::Display};

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

pub use constraints::AuthorityConstraint;
pub use id::AuthorityId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityData<'r> {
    pub id: AuthorityId,
    pub authority: &'r str,
}

#[derive(Debug, Eq, PartialEq)]
pub struct AuthorityMatch<'r, 'p> {
    pub id: AuthorityId,
    pub authority: &'r str,
    pub parameters: AuthorityParameters<'r, 'p>,
}

pub type AuthorityParameters<'r, 'p> = SmallVec<[(&'r str, &'p str); 4]>;

#[derive(Clone)]
pub struct StoredConstraint {
    pub type_name: &'static str,
    pub check: fn(&str) -> bool,
}

#[derive(Clone)]
pub struct AuthorityRouter<'r> {
    pub root: Node<'r, RootState>,
    pub constraints: HashMap<&'r str, StoredConstraint>,
    pub id: AuthorityIdGenerator,
}

impl<'r> AuthorityRouter<'r> {
    #[must_use]
    pub fn new() -> Self {
        #[allow(unused_mut)]
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
            id: AuthorityIdGenerator::default(),
        };

        // TODO
        // router.constraint::<u8>().unwrap();
        // router.constraint::<u16>().unwrap();
        // router.constraint::<u32>().unwrap();
        // router.constraint::<u64>().unwrap();
        // router.constraint::<u128>().unwrap();
        // router.constraint::<usize>().unwrap();
        // router.constraint::<i8>().unwrap();
        // router.constraint::<i16>().unwrap();
        // router.constraint::<i32>().unwrap();
        // router.constraint::<i64>().unwrap();
        // router.constraint::<i128>().unwrap();
        // router.constraint::<isize>().unwrap();
        // router.constraint::<f32>().unwrap();
        // router.constraint::<f64>().unwrap();
        // router.constraint::<bool>().unwrap();
        // router.constraint::<Ipv4Addr>().unwrap();
        // router.constraint::<Ipv6Addr>().unwrap();

        router
    }

    pub fn constraint<C: AuthorityConstraint>(&mut self) -> Result<(), AuthorityConstraintError> {
        if let Some(existing) = self.constraints.get(C::NAME) {
            return Err(AuthorityConstraintError::DuplicateName {
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

    pub(crate) fn conflicts(
        &self,
        authority: &str,
    ) -> Result<Option<AuthorityId>, AuthorityDeleteError> {
        let mut parsed = Parser::new(authority.as_bytes())?;

        if let Some(data) = self.root.find(&mut parsed.route) {
            return Ok(Some(data.id));
        }

        Ok(None)
    }

    pub(crate) fn insert(
        &mut self,
        authority: &'r str,
    ) -> Result<AuthorityId, AuthorityInsertError> {
        let mut parsed = Parser::new(authority.as_bytes())?;

        // Check for invalid constraints
        for part in &parsed.route.parts {
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
                    return Err(AuthorityInsertError::UnknownConstraint {
                        constraint: name.to_string(),
                    });
                }
            }
        }

        // Check for conflicts
        if let Ok(Some(id)) = self.conflicts(authority) {
            return Ok(id);
        }

        // No conflicts, proceed with new insert
        let id = self.id.next();

        self.root
            .insert(&mut parsed.route, AuthorityData { id, authority });

        self.root.optimize();
        Ok(id)
    }

    pub(crate) fn find(
        &self,
        authority: &str,
    ) -> Result<Option<AuthorityId>, AuthorityDeleteError> {
        let mut parsed = Parser::new(authority.as_bytes())?;

        if let Some(data) = self.root.find(&mut parsed.route) {
            if data.authority != authority {
                return Err(AuthorityDeleteError::Mismatch {
                    authority: authority.to_owned(),
                    inserted: data.authority.to_owned(),
                });
            }
            Ok(Some(data.id))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn delete(&mut self, authority: &str) {
        let Ok(mut parsed) = Parser::new(authority.as_bytes()) else {
            return;
        };

        let Ok(Some(_)) = self.find(authority) else {
            return;
        };

        self.root.delete(&mut parsed.route);
        self.root.optimize();
    }

    pub(crate) fn search<'p>(
        &'r self,
        authority: &'p [u8],
    ) -> Result<Option<AuthorityMatch<'r, 'p>>, AuthoritySearchError> {
        let mut parameters = smallvec![];
        let Some((data, _)) = self
            .root
            .search(authority, &mut parameters, &self.constraints)?
        else {
            return Ok(None);
        };

        Ok(Some(AuthorityMatch {
            id: data.id,
            authority: data.authority,
            parameters,
        }))
    }
}

impl Display for AuthorityRouter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}
