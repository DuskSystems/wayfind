use alloc::vec::Vec;

/// A cached needle position packed into a `usize`.
#[derive(Clone, Copy, Eq, PartialEq)]
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
    entries: Vec<CachedPosition>,
}

impl NeedleCache {
    pub(crate) const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// The rightmost position of the needle, cached after first lookup.
    pub(crate) fn rightmost(&mut self, id: usize, needle: &[u8], path: &str) -> Option<usize> {
        if id >= self.entries.len() {
            self.entries.resize(id + 1, CachedPosition::NOT_COMPUTED);
        }

        let entry = &mut self.entries[id];
        if *entry == CachedPosition::NOT_COMPUTED {
            *entry = match memchr::memmem::rfind(path.as_bytes(), needle) {
                Some(position) => CachedPosition(position),
                None => CachedPosition::NOT_FOUND,
            };
        }

        entry.get()
    }
}
