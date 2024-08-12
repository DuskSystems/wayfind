pub mod numeric;

pub trait Constraint {
    const NAME: &'static str;

    fn check(segment: &str) -> bool;
}
