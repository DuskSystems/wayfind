use smallvec::smallvec;
use std::error::Error;
use wayfind::{Match, Path, RoutableBuilder, Router};

#[test]
fn test_wildcard_simple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/{*path}/delete").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ {*path}
       ╰─ /delete [*]
    ");

    let path = Path::new("/docs/delete")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{*path}/delete",
            expanded: None,
            data: &1,
            parameters: smallvec![("path", "docs")],
        })
    );

    let path = Path::new("/nested/docs/folder/delete")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{*path}/delete",
            expanded: None,
            data: &1,
            parameters: smallvec![("path", "nested/docs/folder")],
        })
    );

    let path = Path::new("/delete")?;
    let search = router.search(&path)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_wildcard_multiple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new()
        .route("/{*prefix}/static/{*suffix}/file")
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ {*prefix}
       ╰─ /static/
          ╰─ {*suffix}
             ╰─ /file [*]
    ");

    let path = Path::new("/a/static/b/file")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{*prefix}/static/{*suffix}/file",
            expanded: None,
            data: &1,
            parameters: smallvec![("prefix", "a"), ("suffix", "b")],
        })
    );

    let path = Path::new("/a/b/c/static/d/e/f/file")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{*prefix}/static/{*suffix}/file",
            expanded: None,
            data: &1,
            parameters: smallvec![("prefix", "a/b/c"), ("suffix", "d/e/f")],
        })
    );

    Ok(())
}

#[test]
fn test_wildcard_inline() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/{*path}.html").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ {*path}
       ╰─ .html [*]
    ");

    let path = Path::new("/page.html")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{*path}.html",
            expanded: None,
            data: &1,
            parameters: smallvec![("path", "page")],
        })
    );

    let path = Path::new("/nested/page.html")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{*path}.html",
            expanded: None,
            data: &1,
            parameters: smallvec![("path", "nested/page")],
        })
    );

    let path = Path::new("/.html")?;
    let search = router.search(&path)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_wildcard_greedy() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new()
        .route("/{*first}-{*second}")
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ {*first}
       ╰─ -
          ╰─ {*second} [*]
    ");

    let path = Path::new("/a-b-c")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{*first}-{*second}",
            expanded: None,
            data: &1,
            parameters: smallvec![("first", "a-b"), ("second", "c")],
        })
    );

    let path = Path::new("/path/to/some-file/with-multiple-hyphens")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{*first}-{*second}",
            expanded: None,
            data: &1,
            parameters: smallvec![
                ("first", "path/to/some-file/with-multiple"),
                ("second", "hyphens")
            ],
        })
    );

    Ok(())
}

#[test]
fn test_wildcard_empty_segments() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/{*path}/end").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ {*path}
       ╰─ /end [*]
    ");

    let path = Path::new("/start/middle/end")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{*path}/end",
            expanded: None,
            data: &1,
            parameters: smallvec![("path", "start/middle")],
        })
    );

    let path = Path::new("/start//middle///end")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{*path}/end",
            expanded: None,
            data: &1,
            parameters: smallvec![("path", "start//middle//")],
        })
    );

    Ok(())
}

#[test]
fn test_wildcard_priority() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/static/path").build()?;
    router.insert(&route, 1)?;
    let route = RoutableBuilder::new().route("/static/{*rest}").build()?;
    router.insert(&route, 2)?;
    let route = RoutableBuilder::new().route("/{*path}/static").build()?;
    router.insert(&route, 3)?;
    let route = RoutableBuilder::new().route("/prefix.{*suffix}").build()?;
    router.insert(&route, 4)?;
    let route = RoutableBuilder::new().route("/{*prefix}.suffix").build()?;
    router.insert(&route, 5)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ prefix.
    │  ╰─ {*suffix} [*]
    ├─ static/
    │  ├─ path [*]
    │  ╰─ {*rest} [*]
    ├─ {*prefix}
    │  ╰─ .suffix [*]
    ╰─ {*path}
       ╰─ /static [*]
    ");

    let path = Path::new("/static/path")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/static/path",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/static/some/nested/path")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/static/{*rest}",
            expanded: None,
            data: &2,
            parameters: smallvec![("rest", "some/nested/path")],
        })
    );

    let path = Path::new("/some/nested/path/static")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{*path}/static",
            expanded: None,
            data: &3,
            parameters: smallvec![("path", "some/nested/path")],
        })
    );

    let path = Path::new("/prefix.some/nested/path")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/prefix.{*suffix}",
            expanded: None,
            data: &4,
            parameters: smallvec![("suffix", "some/nested/path")],
        })
    );

    let path = Path::new("/some/nested/path.suffix")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{*prefix}.suffix",
            expanded: None,
            data: &5,
            parameters: smallvec![("prefix", "some/nested/path")],
        })
    );

    Ok(())
}
