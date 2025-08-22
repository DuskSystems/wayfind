use std::error::Error;

use similar_asserts::assert_eq;
use wayfind::{Router, errors::DeleteError};

#[test]
fn test_delete() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/test", 1)?;

    insta::assert_snapshot!(router, @"/test [*]");

    let delete = router.delete("/tests");
    assert_eq!(
        delete,
        Err(DeleteError::NotFound {
            template: "/tests".to_owned()
        })
    );

    insta::assert_snapshot!(router, @"/test [*]");

    let delete = router.delete("/test")?;
    assert_eq!(delete, 1);

    insta::assert_snapshot!(router, @"");

    Ok(())
}
