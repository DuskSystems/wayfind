use std::error::Error;
use wayfind::{constraints::Constraint, errors::constraint::ConstraintError, router::Router};

struct ConstraintA;
impl Constraint for ConstraintA {
    const NAME: &'static str = "my_constraint";

    fn check(_segment: &str) -> bool {
        true
    }
}

struct ConstraintB;
impl Constraint for ConstraintB {
    const NAME: &'static str = "my_constraint";

    fn check(_segment: &str) -> bool {
        true
    }
}

#[test]
fn constraint_duplicate_name_error() -> Result<(), Box<dyn Error>> {
    let mut router: Router<usize> = Router::new();
    router.constraint::<ConstraintA>()?;

    let error = router.constraint::<ConstraintB>().err().unwrap();
    assert_eq!(
        error,
        ConstraintError::DuplicateName {
            name: "my_constraint",
            existing_type: "errors::ConstraintA",
            new_type: "errors::ConstraintB",
        }
    );

    insta::assert_snapshot!(error, @r###"
    error: duplicate constraint name

    The constraint name 'my_constraint' is already in use:
        - existing constraint type: 'errors::ConstraintA'
        - new constraint type: 'errors::ConstraintB'

    help: each constraint must have a unique name

    try:
        - Check if you have accidentally added the same constraint twice
        - Ensure different constraints have different names
    "###);

    Ok(())
}
