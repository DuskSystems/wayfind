#[derive(Clone, Default)]
pub struct PathIdGenerator {
    id: usize,
}

impl PathIdGenerator {
    pub fn next(&mut self) -> PathId {
        self.id += 1;
        PathId(self.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PathId(pub usize);
