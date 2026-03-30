/// Node needs optimization.
const NEEDS_OPTIMIZATION: u8 = 1 << 0;

/// All dynamic children are full segments.
const DYNAMIC_SEGMENT_ONLY: u8 = 1 << 1;

/// All wildcard children are full segments.
const WILDCARD_SEGMENT_ONLY: u8 = 1 << 2;

/// Bitflags for node state.
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub(crate) struct Flags(u8);

impl Flags {
    #[inline(always)]
    pub(crate) const fn needs_optimization(&self) -> bool {
        self.0 & NEEDS_OPTIMIZATION != 0
    }

    #[inline(always)]
    pub(crate) const fn set_needs_optimization(&mut self, value: bool) {
        self.set(NEEDS_OPTIMIZATION, value);
    }

    #[inline(always)]
    pub(crate) const fn dynamic_segment_only(&self) -> bool {
        self.0 & DYNAMIC_SEGMENT_ONLY != 0
    }

    #[inline(always)]
    pub(crate) const fn set_dynamic_segment_only(&mut self, value: bool) {
        self.set(DYNAMIC_SEGMENT_ONLY, value);
    }

    #[inline(always)]
    pub(crate) const fn wildcard_segment_only(&self) -> bool {
        self.0 & WILDCARD_SEGMENT_ONLY != 0
    }

    #[inline(always)]
    pub(crate) const fn set_wildcard_segment_only(&mut self, value: bool) {
        self.set(WILDCARD_SEGMENT_ONLY, value);
    }

    #[inline(always)]
    const fn set(&mut self, flag: u8, value: bool) {
        if value {
            self.0 |= flag;
        } else {
            self.0 &= !flag;
        }
    }
}
