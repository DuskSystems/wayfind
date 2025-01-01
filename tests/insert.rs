use similar_asserts::assert_eq;
use std::error::Error;
use wayfind::{
    errors::{InsertError, PathConstraintError, PathInsertError},
    AuthorityId, DataChain, MethodId, PathConstraint, PathId, RouteBuilder, Router,
};

#[test]
fn test_insert_conflict() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/test").build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new().route("/test").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            chain: DataChain {
                authority: AuthorityId(None),
                path: PathId(Some(1)),
                method: MethodId(None),
            }
        })
    );

    let route = RouteBuilder::new().route("(/test)").build()?;
    let insert = router.insert(&route, 3);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            chain: DataChain {
                authority: AuthorityId(None),
                path: PathId(Some(1)),
                method: MethodId(None),
            }
        })
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

    Ok(())
}

#[test]
fn test_insert_conflict_expanded() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("(/test)").build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new().route("/test").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            chain: DataChain {
                authority: AuthorityId(None),
                path: PathId(Some(1)),
                method: MethodId(None),
            }
        })
    );

    let route = RouteBuilder::new().route("(/test)").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            chain: DataChain {
                authority: AuthorityId(None),
                path: PathId(Some(1)),
                method: MethodId(None),
            }
        })
    );

    let route = RouteBuilder::new().route("(/best)").build()?;
    let insert = router.insert(&route, 3);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            chain: DataChain {
                authority: AuthorityId(None),
                path: PathId(Some(1)),
                method: MethodId(None),
            }
        })
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

    Ok(())
}

#[test]
fn test_insert_conflict_multiple_expanded() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("(/hello)").build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new().route("(/world)").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            chain: DataChain {
                authority: AuthorityId(None),
                path: PathId(Some(1)),
                method: MethodId(None),
            }
        })
    );

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    / [*:1]
    ╰─ hello [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    Ok(())
}

// NOTE: End wildcards have some duplicate code, so worth testing seperate to rest.
#[test]
fn test_insert_conflict_end_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("(/{*catch_all})").build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new().route("/{*catch_all}").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            chain: DataChain {
                authority: AuthorityId(None),
                path: PathId(Some(1)),
                method: MethodId(None),
            }
        })
    );

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    / [*:1]
    ╰─ {*catch_all} [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    Ok(())
}

#[test]
fn test_insert_overlapping() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/a(/b)").build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new().route("/x/y").build()?;
    router.insert(&route, 2)?;

    let route = RouteBuilder::new().route("(/a(/b))(/x/y)").build()?;
    let insert = router.insert(&route, 3);
    assert_eq!(
        insert,
        Err(InsertError::Path(PathInsertError::Overlapping {
            ids: vec![1, 2]
        }))
    );

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /
    ├─ x/y [*:2]
    ╰─ a [*:1]
       ╰─ /b [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    *-2-*
    ");

    Ok(())
}

#[test]
fn test_insert_constraint_conflict() {
    struct MyConstraint;
    impl PathConstraint for MyConstraint {
        const NAME: &'static str = "u32";
        fn check(segment: &str) -> bool {
            segment.parse::<u32>().is_ok()
        }
    }

    let mut router: Router<usize> = Router::new();
    let constraint = router.path.constraint::<MyConstraint>();
    assert_eq!(
        constraint,
        Err(PathConstraintError::DuplicateName {
            name: "u32",
            existing_type: "u32",
            new_type: "insert::test_insert_constraint_conflict::MyConstraint"
        })
    );
}
