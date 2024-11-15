use std::error::Error;
use wayfind::{errors::DeleteError, RoutableBuilder, Router};

#[test]
fn test_delete() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/test").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @"/test [*]");

    let route = RoutableBuilder::new().route("/tests").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::NotFound {
            route: "/tests".to_owned()
        })
    );

    insta::assert_snapshot!(router, @"/test [*]");

    let route = RoutableBuilder::new().route("(/test)").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::Multiple(vec![
            DeleteError::RouteMismatch {
                route: "(/test)".to_owned(),
                inserted: "/test".to_owned()
            },
            DeleteError::NotFound {
                route: "(/test)".to_owned()
            }
        ]))
    );

    insta::assert_snapshot!(router, @"/test [*]");

    let route = RoutableBuilder::new().route("/test").build()?;
    router.delete(&route)?;
    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn test_delete_mismatch() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("(/test)").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ test [*]
    ");

    let route = RoutableBuilder::new().route("/test").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::RouteMismatch {
            route: "/test".to_owned(),
            inserted: "(/test)".to_owned()
        })
    );

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ test [*]
    ");

    let route = RoutableBuilder::new().route("/").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::RouteMismatch {
            route: "/".to_owned(),
            inserted: "(/test)".to_owned()
        })
    );

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ test [*]
    ");

    let route = RoutableBuilder::new().route("(/test)").build()?;
    router.delete(&route)?;
    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn test_delete_empty() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/{id}data").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ {id}
       ╰─ data [*]
    ");

    let route = RoutableBuilder::new().route("/{id}").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::NotFound {
            route: "/{id}".to_owned()
        })
    );

    insta::assert_snapshot!(router, @r"
    /
    ╰─ {id}
       ╰─ data [*]
    ");

    Ok(())
}
