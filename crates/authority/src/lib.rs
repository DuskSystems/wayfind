#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use constraints::AuthorityConstraint;
use errors::{AuthorityConstraintError, AuthorityDeleteError, AuthorityInsertError};
use id::AuthorityIdGenerator;
use parser::ParsedAuthority;
use smallvec::{smallvec, SmallVec};
use std::{collections::HashMap, fmt::Display, str::Utf8Error, sync::Arc};
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

pub use id::AuthorityId;

pub type AuthorityParameters<'r, 'p> = SmallVec<[(&'r str, &'p str); 4]>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityConfig;

impl Config for AuthorityConfig {
    type Data = AuthorityData;

    const DELIMITER: u8 = b'.';
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityData {
    pub id: AuthorityId,
    pub authority: Arc<str>,
}

impl Data for AuthorityData {
    fn id(&self) -> Option<usize> {
        self.id.0
    }

    fn priority(&self) -> usize {
        self.authority.len() + (self.authority.bytes().filter(|&b| b == b'.').count() * 100)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct AuthorityMatch<'r, 'p> {
    pub id: AuthorityId,
    pub authority: Arc<str>,
    pub parameters: AuthorityParameters<'r, 'p>,
}

#[derive(Clone)]
pub struct AuthorityRouter {
    pub root: Node<AuthorityConfig, RootState>,
    pub constraints: HashMap<Arc<str>, StoredConstraint>,
    pub id: AuthorityIdGenerator,
}

impl AuthorityRouter {
    #[must_use]
    pub fn new() -> Self {
        #[allow(unused_mut)]
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
            id: AuthorityIdGenerator::default(),
        };

        // TODO

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
        authority: &ParsedAuthority,
    ) -> Result<Option<AuthorityId>, AuthorityDeleteError> {
        if let Some(data) = self.root.find(key, &authority.template) {
            return Ok(Some(data.id));
        }

        Ok(None)
    }

    pub fn insert(
        &mut self,
        key: Option<usize>,
        authority: &ParsedAuthority,
    ) -> Result<AuthorityId, AuthorityInsertError> {
        // Check for invalid constraints
        for part in &authority.template.parts {
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
        if let Ok(Some(id)) = self.conflicts(key, authority) {
            return Ok(id);
        }

        // No conflicts, proceed with new insert
        let id = self.id.generate();

        self.root.insert(
            key,
            &authority.template,
            AuthorityData {
                id,
                authority: Arc::clone(&authority.input),
            },
        );

        self.root.optimize();
        Ok(id)
    }

    pub fn find(
        &self,
        key: Option<usize>,
        authority: &ParsedAuthority,
    ) -> Result<Option<AuthorityId>, AuthorityDeleteError> {
        if let Some(data) = self.root.find(key, &authority.template) {
            if data.authority != authority.input {
                return Err(AuthorityDeleteError::Mismatch {
                    authority: authority.input.to_string(),
                    inserted: data.authority.to_string(),
                });
            }
            Ok(Some(data.id))
        } else {
            Ok(None)
        }
    }

    pub fn delete(&mut self, key: Option<usize>, authority: &ParsedAuthority) {
        let Ok(Some(_)) = self.find(key, authority) else {
            return;
        };

        self.root.delete(key, &authority.template);
        self.root.optimize();
    }

    pub fn search<'r, 'p>(
        &'r self,
        key: Option<usize>,
        authority: &'p [u8],
    ) -> Result<Option<AuthorityMatch<'r, 'p>>, Utf8Error>
    where
        'p: 'r,
    {
        let mut parameters: SmallVec<[(&str, &[u8]); 4]> = smallvec![];
        let Some((data, _)) = self
            .root
            .search(key, authority, &mut parameters, &self.constraints)
        else {
            return Ok(None);
        };

        let parameters = parameters
            .into_iter()
            .map(|(name, value)| std::str::from_utf8(value).map(|value| (name, value)))
            .collect::<Result<_, Utf8Error>>()?;

        Ok(Some(AuthorityMatch {
            id: data.id,
            authority: Arc::clone(&data.authority),
            parameters,
        }))
    }
}

impl Display for AuthorityRouter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}
