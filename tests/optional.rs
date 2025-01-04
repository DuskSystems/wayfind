use std::error::Error;

use similar_asserts::assert_eq;
use smallvec::smallvec;
use wayfind::{Match, Router};

#[test]
fn test_optional_starting() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("(/{lang})/users", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ users [*]
    ╰─ {lang}
       ╰─ /users [*]
    ");

    let search = router.search("/en/users")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/{lang})/users",
            expanded: Some("/{lang}/users"),
            parameters: smallvec![("lang", "en")],
        })
    );

    let search = router.search("/users")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/{lang})/users",
            expanded: Some("/users"),
            parameters: smallvec![],
        })
    );

    Ok(())
}

#[test]
fn test_optional_ending() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users(/)", 1)?;

    insta::assert_snapshot!(router, @r"
    /users [*]
    ╰─ / [*]
    ");

    let search = router.search("/users")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users(/)",
            expanded: Some("/users"),
            parameters: smallvec![],
        })
    );

    let search = router.search("/users/")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users(/)",
            expanded: Some("/users/"),
            parameters: smallvec![],
        })
    );

    Ok(())
}

#[test]
fn test_optional_nested() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("(/a(/b(/c)))", 1)?;

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ a [*]
       ╰─ /b [*]
          ╰─ /c [*]
    ");

    let search = router.search("/a/b/c")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/a(/b(/c)))",
            expanded: Some("/a/b/c"),
            parameters: smallvec![],
        })
    );

    let search = router.search("/a/b")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/a(/b(/c)))",
            expanded: Some("/a/b"),
            parameters: smallvec![],
        })
    );

    let search = router.search("/a")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/a(/b(/c)))",
            expanded: Some("/a"),
            parameters: smallvec![],
        })
    );

    let search = router.search("/")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/a(/b(/c)))",
            expanded: Some("/"),
            parameters: smallvec![],
        })
    );

    Ok(())
}

#[test]
fn test_optional_only() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("(/test)", 1)?;

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ test [*]
    ");

    let search = router.search("/test")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/test)",
            expanded: Some("/test"),
            parameters: smallvec![],
        })
    );

    let search = router.search("/")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/test)",
            expanded: Some("/"),
            parameters: smallvec![],
        })
    );

    Ok(())
}

#[test]
fn test_optional_touching() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("(/a)(/b)(/c)", 1)?;

    insta::assert_snapshot!(router, @r"
    / [*]
    ├─ a [*]
    │  ╰─ /
    │     ├─ b [*]
    │     │  ╰─ /c [*]
    │     ╰─ c [*]
    ├─ b [*]
    │  ╰─ /c [*]
    ╰─ c [*]
    ");

    let search = router.search("/a/b/c")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/a)(/b)(/c)",
            expanded: Some("/a/b/c"),
            parameters: smallvec![],
        })
    );

    let search = router.search("/a/b")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/a)(/b)(/c)",
            expanded: Some("/a/b"),
            parameters: smallvec![],
        })
    );

    let search = router.search("/a/c")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/a)(/b)(/c)",
            expanded: Some("/a/c"),
            parameters: smallvec![],
        })
    );

    let search = router.search("/a")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/a)(/b)(/c)",
            expanded: Some("/a"),
            parameters: smallvec![],
        })
    );

    let search = router.search("/b/c")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/a)(/b)(/c)",
            expanded: Some("/b/c"),
            parameters: smallvec![],
        })
    );

    let search = router.search("/b")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/a)(/b)(/c)",
            expanded: Some("/b"),
            parameters: smallvec![],
        })
    );

    let search = router.search("/c")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/a)(/b)(/c)",
            expanded: Some("/c"),
            parameters: smallvec![],
        })
    );

    let search = router.search("/")?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "(/a)(/b)(/c)",
            expanded: Some("/"),
            parameters: smallvec![],
        })
    );

    Ok(())
}
