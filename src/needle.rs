use crate::storage::Storage;

/// A cached needle position packed into a `usize`.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct CachedPosition(usize);

impl CachedPosition {
    const NOT_COMPUTED: Self = Self(usize::MAX);
    const NOT_FOUND: Self = Self(usize::MAX - 1);

    const fn get(self) -> Option<usize> {
        match self {
            Self::NOT_COMPUTED | Self::NOT_FOUND => None,
            Self(position) => Some(position),
        }
    }
}

/// Cached rightmost positions for `Contains` checks.
pub(crate) struct NeedleCache {
    entries: Storage<CachedPosition, 8>,
}

impl NeedleCache {
    pub(crate) const fn new() -> Self {
        Self {
            entries: Storage::new(),
        }
    }

    /// The rightmost position of the needle, cached after first lookup.
    pub(crate) fn rightmost(&mut self, id: usize, needle: &[u8], path: &str) -> Option<usize> {
        let entry = self.entries.slot(id, CachedPosition::NOT_COMPUTED)?;
        if *entry == CachedPosition::NOT_COMPUTED {
            *entry = match memchr::memmem::rfind(path.as_bytes(), needle) {
                Some(position) => CachedPosition(position),
                None => CachedPosition::NOT_FOUND,
            };
        }

        entry.get()
    }
}

#[cfg(test)]
mod tests {
    use similar_asserts::assert_eq;

    use super::*;

    #[test]
    fn found() {
        let mut cache = NeedleCache::new();
        assert_eq!(cache.rightmost(0, b"/users", "/users/users/1"), Some(6));
        assert_eq!(cache.rightmost(0, b"/users", "/users/users/1"), Some(6));
    }

    #[test]
    fn missing() {
        let mut cache = NeedleCache::new();
        assert_eq!(cache.rightmost(0, b"/posts", "/users/1"), None);
        assert_eq!(cache.rightmost(0, b"/posts", "/users/1"), None);
    }
}
