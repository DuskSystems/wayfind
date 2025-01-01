use similar_asserts::assert_eq;
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

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /test [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let route = RouteBuilder::new().route("/tests").build()?;
    let delete = router.delete(&route);
    assert_eq!(delete, Err(DeleteError::NotFound));

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /test [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let route = RouteBuilder::new().route("(/test)").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::Path(PathDeleteError::Mismatch {
            path: "(/test)".to_owned(),
            inserted: "/test".to_owned(),
        }))
    );

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /test [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let route = RouteBuilder::new().route("/test").build()?;
    let delete = router.delete(&route)?;
    assert_eq!(delete, 1);

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    Empty
    === Method
    Empty
    === Chains
    Empty
    ");

    Ok(())
}

#[test]
fn test_delete_mismatch() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("(/test)").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    / [*:1]
    ╰─ test [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let route = RouteBuilder::new().route("/test").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::Path(PathDeleteError::Mismatch {
            path: "/test".to_owned(),
            inserted: "(/test)".to_owned()
        }))
    );

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    / [*:1]
    ╰─ test [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let route = RouteBuilder::new().route("/").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::Path(PathDeleteError::Mismatch {
            path: "/".to_owned(),
            inserted: "(/test)".to_owned()
        }))
    );

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    / [*:1]
    ╰─ test [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let route = RouteBuilder::new().route("(/test)").build()?;
    let delete = router.delete(&route)?;
    assert_eq!(delete, 1);

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    Empty
    === Method
    Empty
    === Chains
    Empty
    ");

    Ok(())
}

#[test]
fn test_delete_overlap() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/a(/b)").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /a [*:1]
    ╰─ /b [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let route = RouteBuilder::new().route("/a").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::Path(PathDeleteError::Mismatch {
            path: "/a".to_owned(),
            inserted: "/a(/b)".to_owned(),
        }))
    );

    let route = RouteBuilder::new().route("/a(/b(/c))").build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::Path(PathDeleteError::Mismatch {
            path: "/a(/b(/c))".to_owned(),
            inserted: "/a(/b)".to_owned(),
        }))
    );

    let route = RouteBuilder::new().route("/a(/b)").build()?;
    let delete = router.delete(&route)?;
    assert_eq!(delete, 1);

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    Empty
    === Method
    Empty
    === Chains
    Empty
    ");

    Ok(())
}

#[test]
fn test_delete_empty() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/{id}data").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /
    ╰─ {id}
       ╰─ data [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let route = RouteBuilder::new().route("/{id}").build()?;
    let delete = router.delete(&route);
    assert_eq!(delete, Err(DeleteError::NotFound));

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /
    ╰─ {id}
       ╰─ data [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    Ok(())
}
