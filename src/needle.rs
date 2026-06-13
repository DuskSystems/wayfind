use core::num::NonZeroUsize;

use crate::storage::Storage;

/// Cached rightmost positions for `Contains` checks.
pub(crate) struct NeedleCache {
    entries: Storage<(usize, Option<NonZeroUsize>), 8>,
}

impl NeedleCache {
    pub(crate) const fn new() -> Self {
        Self {
            entries: Storage::new(),
        }
    }

    /// The rightmost position of the needle, cached after first lookup.
    pub(crate) fn rightmost(&mut self, id: usize, needle: &[u8], path: &str) -> Option<usize> {
        if let Some((_, cached)) = self
            .entries
            .as_slice()
            .iter()
            .copied()
            .find(|&(entry, _)| entry == id)
        {
            return cached.map(|position| position.get() - 1);
        }

        let position = memchr::memmem::rfind(path.as_bytes(), needle);
        self.entries
            .push((id, position.and_then(|found| NonZeroUsize::new(found + 1))));

        position
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
