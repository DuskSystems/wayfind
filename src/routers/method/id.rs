use std::fmt::Display;

#[derive(Clone, Default)]
pub struct MethodIdGenerator {
    id: usize,
}

impl MethodIdGenerator {
    pub fn next(&mut self) -> MethodId {
        self.id += 1;
        MethodId(Some(self.id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MethodId(pub Option<usize>);

impl Display for MethodId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(id) = self.0 {
            write!(f, "{id}")
        } else {
            write!(f, "*")
        }
    }
}
