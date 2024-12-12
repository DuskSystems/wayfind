use crate::routers::path::id::PathId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DataChain {
    pub path: PathId,
}
