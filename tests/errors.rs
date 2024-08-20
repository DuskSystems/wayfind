use std::error::Error;
use wayfind::{
    constraints::Constraint,
    errors::{constraint::ConstraintError, decode::DecodeError},
    path::Path,
    router::Router,
};

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

#[test]
fn decode_invalid_enconding_error() {
    let error = Path::new("/hello%20world%GG").err().unwrap();
    assert_eq!(
        error,
        DecodeError::InvalidEncoding {
            input: "/hello%20world%GG".to_string(),
            position: 14,
            character: [b'%', b'G', b'G']
        }
    );

    insta::assert_snapshot!(error, @r###"
    error: invalid percent-encoding

       Input: /hello%20world%GG
                            ^^^

    Expected: '%' followed by two hexadecimal digits (a-F, 0-9)
       Found: '%GG'
    "###);
}
