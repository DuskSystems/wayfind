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
pub trait Constraint {
    /// The name of the constraint.
    ///
    /// Must be unique within a given router.
    /// Try and avoid generic constraint names like `id`.
    const NAME: &'static str;

    /// Checks if a given path part matches this constraint.
    fn check(part: &str) -> bool;
}
