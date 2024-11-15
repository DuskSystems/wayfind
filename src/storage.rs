use crate::id::RouteId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageKind {
    Inline,
    Router(RouteId),
}

/// Where the route data is stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Storage<T> {
    Inline(T),
    Router(RouteId),
}
