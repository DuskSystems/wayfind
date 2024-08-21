use std::net::{Ipv4Addr, Ipv6Addr};

pub trait Constraint {
    const NAME: &'static str;

    fn check(segment: &str) -> bool;
}

impl Constraint for u8 {
    const NAME: &'static str = "u8";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for u16 {
    const NAME: &'static str = "u16";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for u32 {
    const NAME: &'static str = "u32";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for u64 {
    const NAME: &'static str = "u64";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for u128 {
    const NAME: &'static str = "u128";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for usize {
    const NAME: &'static str = "usize";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for i8 {
    const NAME: &'static str = "i8";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for i16 {
    const NAME: &'static str = "i16";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for i32 {
    const NAME: &'static str = "i32";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for i64 {
    const NAME: &'static str = "i64";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for i128 {
    const NAME: &'static str = "i128";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for isize {
    const NAME: &'static str = "isize";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for f32 {
    const NAME: &'static str = "f32";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for f64 {
    const NAME: &'static str = "f64";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for bool {
    const NAME: &'static str = "bool";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for Ipv4Addr {
    const NAME: &'static str = "ipv4";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}

impl Constraint for Ipv6Addr {
    const NAME: &'static str = "ipv6";

    fn check(segment: &str) -> bool {
        segment.parse::<Self>().is_ok()
    }
}
