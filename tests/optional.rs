use smallvec::smallvec;
use std::error::Error;
use wayfind::{Match, Path, Router};

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

    let path = Path::new("/en/users")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/{lang})/users",
            expanded: Some("/{lang}/users"),
            data: &1,
            parameters: smallvec![("lang", "en")],
        })
    );

    let path = Path::new("/users")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/{lang})/users",
            expanded: Some("/users"),
            data: &1,
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

    let path = Path::new("/users")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users(/)",
            expanded: Some("/users"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/users/")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users(/)",
            expanded: Some("/users/"),
            data: &1,
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

    let path = Path::new("/a/b/c")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a(/b(/c)))",
            expanded: Some("/a/b/c"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/a/b")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a(/b(/c)))",
            expanded: Some("/a/b"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/a")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a(/b(/c)))",
            expanded: Some("/a"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a(/b(/c)))",
            expanded: Some("/"),
            data: &1,
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

    let path = Path::new("/test")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/test)",
            expanded: Some("/test"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/test)",
            expanded: Some("/"),
            data: &1,
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

    let path = Path::new("/a/b/c")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/a/b/c"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/a/b")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/a/b"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/a/c")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/a/c"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/a")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/a"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/b/c")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/b/c"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/b")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/b"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/c")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/c"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/"),
            data: &1,
            parameters: smallvec![],
        })
    );

    Ok(())
}
