use std::fmt::Display;

#[derive(Clone, Default)]
pub struct AuthorityIdGenerator {
    id: usize,
}

impl AuthorityIdGenerator {
    pub fn next(&mut self) -> AuthorityId {
        self.id += 1;
        AuthorityId(Some(self.id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorityId(pub Option<usize>);

impl Display for AuthorityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(id) = self.0 {
            write!(f, "{id}")
        } else {
            write!(f, "*")
        }
    }
}
