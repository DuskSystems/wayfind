use core::sync::atomic::{AtomicUsize, Ordering};

static ID: AtomicUsize = AtomicUsize::new(0);

/// A unique ID for a route within the router.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RouteId(usize);

impl RouteId {
    pub fn new() -> Self {
        Self(ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for RouteId {
    fn default() -> Self {
        Self::new()
    }
}
