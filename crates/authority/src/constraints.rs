pub trait AuthorityConstraint: Send + Sync {
    const NAME: &'static str;

    fn check(segment: &str) -> bool;
}

// TODO
