use std::sync::atomic::{AtomicU64, Ordering};

static ROUTABLE_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RoutableId(u64);

impl RoutableId {
    pub(crate) fn next() -> Self {
        Self(ROUTABLE_ID.fetch_add(1, Ordering::Relaxed))
    }
}
