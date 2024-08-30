use regex::Regex;
use std::sync::LazyLock;
use wayfind::Constraint;

/// Regex for validating path, lifted from Distribution Specification.
/// Note the addition of boundaries `^` and `$`, to ensure the entire segment matches.
static NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-z0-9]+((\.|_|__|-+)[a-z0-9]+)*(\/[a-z0-9]+((\.|_|__|-+)[a-z0-9]+)*)*$")
        .expect("failed to build regex")
});

/// Constraint ensures that the `<name>` parameter in URLs adheres to the OCI Distribution Specification for repository names.
pub struct NameConstraint;

impl Constraint for NameConstraint {
    const NAME: &'static str = "name";

    fn check(segment: &str) -> bool {
        NAME_REGEX.is_match(segment)
    }
}
