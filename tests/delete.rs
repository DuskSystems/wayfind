use std::error::Error;
use wayfind::{
    errors::{DeleteError, PathDeleteError},
    RouteBuilder, Router,
};

#[test]
fn test_delete() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/test").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router.path, @"/test [*]");

    let route = RouteBuilder::new().route("/tests").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::PathDeleteError(PathDeleteError::NotFound {
            route: "/tests".to_owned()
        }))
    );

    insta::assert_snapshot!(router.path, @"/test [*]");

    let route = RouteBuilder::new().route("(/test)").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::PathDeleteError(PathDeleteError::Multiple(
            vec![
                PathDeleteError::RouteMismatch {
                    route: "(/test)".to_owned(),
                    inserted: "/test".to_owned()
                },
                PathDeleteError::NotFound {
                    route: "(/test)".to_owned()
                }
            ]
        )))
    );

    insta::assert_snapshot!(router.path, @"/test [*]");

    let route = RouteBuilder::new().route("/test").build()?;
    router.delete(&route)?;
    insta::assert_snapshot!(router.path, @"");

    Ok(())
}

#[test]
fn test_delete_mismatch() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("(/test)").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router.path, @r"
    / [*]
    ╰─ test [*]
    ");

    let route = RouteBuilder::new().route("/test").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::PathDeleteError(
            PathDeleteError::RouteMismatch {
                route: "/test".to_owned(),
                inserted: "(/test)".to_owned()
            }
        ))
    );

    insta::assert_snapshot!(router.path, @r"
    / [*]
    ╰─ test [*]
    ");

    let route = RouteBuilder::new().route("/").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::PathDeleteError(
            PathDeleteError::RouteMismatch {
                route: "/".to_owned(),
                inserted: "(/test)".to_owned()
            }
        ))
    );

    insta::assert_snapshot!(router.path, @r"
    / [*]
    ╰─ test [*]
    ");

    let route = RouteBuilder::new().route("(/test)").build()?;
    router.delete(&route)?;
    insta::assert_snapshot!(router.path, @"");

    Ok(())
}

#[test]
fn test_delete_empty() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/{id}data").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router.path, @r"
    /
    ╰─ {id}
       ╰─ data [*]
    ");

    let route = RouteBuilder::new().route("/{id}").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::PathDeleteError(PathDeleteError::NotFound {
            route: "/{id}".to_owned()
        }))
    );

    insta::assert_snapshot!(router.path, @r"
    /
    ╰─ {id}
       ╰─ data [*]
    ");

    Ok(())
}
