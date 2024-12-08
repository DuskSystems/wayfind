use std::{
    hash::{Hash, Hasher},
    sync::atomic::{AtomicUsize, Ordering},
};

static ID: AtomicUsize = AtomicUsize::new(0);

/// A unique ID for a route within the router.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouteId(usize);

impl RouteId {
    pub fn new() -> Self {
        Self(ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl Hash for RouteId {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_usize(self.0);
    }
}
