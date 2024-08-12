use super::Constraint;

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
