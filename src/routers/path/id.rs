use std::fmt::Display;

#[derive(Clone, Default)]
pub struct PathIdGenerator {
    id: usize,
}

impl PathIdGenerator {
    pub fn next(&mut self) -> PathId {
        let id = PathId(self.id);
        self.id += 1;
        id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PathId(pub usize);

impl Display for PathId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
