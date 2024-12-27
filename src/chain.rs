use crate::{MethodId, PathId};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataChain {
    pub path: PathId,
    pub method: MethodId,
}

impl Display for DataChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.path, self.method)
    }
}
