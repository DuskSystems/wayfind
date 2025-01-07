use std::net::{Ipv4Addr, Ipv6Addr};

/// A constraint that can be used for custom path routing logic.
///
/// Constraints can be registered within a [`Router`](crate::Router) via the [`constraint`](crate::Router::constraint) function.
///
/// # Example
///
/// ```rust
/// use wayfind::Constraint;
///
/// struct HelloConstraint;
/// impl Constraint for HelloConstraint {
///     const NAME: &'static str = "hello";
///
///     fn check(part: &str) -> bool {
///         part == "hello"
///     }
/// }
/// ```
pub trait Constraint: Send + Sync {
    /// The name of the constraint.
    ///
    /// Must be unique within a given router.
    /// Try and avoid generic constraint names like `id`.
    const NAME: &'static str;

    /// Checks if a given path part matches this constraint.
    fn check(part: &str) -> bool;
}

impl Constraint for u8 {
    const NAME: &'static str = "u8";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for u16 {
    const NAME: &'static str = "u16";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for u32 {
    const NAME: &'static str = "u32";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for u64 {
    const NAME: &'static str = "u64";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for u128 {
    const NAME: &'static str = "u128";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for usize {
    const NAME: &'static str = "usize";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for i8 {
    const NAME: &'static str = "i8";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for i16 {
    const NAME: &'static str = "i16";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for i32 {
    const NAME: &'static str = "i32";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for i64 {
    const NAME: &'static str = "i64";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for i128 {
    const NAME: &'static str = "i128";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for isize {
    const NAME: &'static str = "isize";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for f32 {
    const NAME: &'static str = "f32";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for f64 {
    const NAME: &'static str = "f64";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for bool {
    const NAME: &'static str = "bool";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for Ipv4Addr {
    const NAME: &'static str = "ipv4";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}

impl Constraint for Ipv6Addr {
    const NAME: &'static str = "ipv6";

    fn check(part: &str) -> bool {
        part.parse::<Self>().is_ok()
    }
}
