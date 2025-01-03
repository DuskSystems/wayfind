use std::fmt::Display;

#[derive(Clone, Default)]
pub struct PathIdGenerator {
    id: usize,
}

impl PathIdGenerator {
    pub fn generate(&mut self) -> PathId {
        self.id += 1;
        PathId(Some(self.id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathId(pub Option<usize>);

impl Display for PathId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(id) = self.0 {
            write!(f, "{id}")
        } else {
            write!(f, "*")
        }
    }
}
