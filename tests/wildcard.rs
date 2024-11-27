use smallvec::smallvec;
use std::error::Error;
use wayfind::{Match, RequestBuilder, RouteBuilder, Router};

#[test]
fn test_wildcard_simple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/{*path}/delete").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router.path, @r"
    /
    ╰─ {*path}
       ╰─ /delete [*]
    ");

    let request = RequestBuilder::new().path("/docs/delete").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            route: "/{*path}/delete",
            expanded: None,
            parameters: smallvec![("path", "docs")],
        })
    );

    let request = RequestBuilder::new()
        .path("/nested/docs/folder/delete")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            route: "/{*path}/delete",
            expanded: None,
            parameters: smallvec![("path", "nested/docs/folder")],
        })
    );

    let request = RequestBuilder::new().path("/delete").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_wildcard_multiple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/{*prefix}/static/{*suffix}/file")
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router.path, @r"
    /
    ╰─ {*prefix}
       ╰─ /static/
          ╰─ {*suffix}
             ╰─ /file [*]
    ");

    let request = RequestBuilder::new().path("/a/static/b/file").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            route: "/{*prefix}/static/{*suffix}/file",
            expanded: None,
            parameters: smallvec![("prefix", "a"), ("suffix", "b")],
        })
    );

    let request = RequestBuilder::new()
        .path("/a/b/c/static/d/e/f/file")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            route: "/{*prefix}/static/{*suffix}/file",
            expanded: None,
            parameters: smallvec![("prefix", "a/b/c"), ("suffix", "d/e/f")],
        })
    );

    Ok(())
}

#[test]
fn test_wildcard_inline() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/{*path}.html").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router.path, @r"
    /
    ╰─ {*path}
       ╰─ .html [*]
    ");

    let request = RequestBuilder::new().path("/page.html").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            route: "/{*path}.html",
            expanded: None,
            parameters: smallvec![("path", "page")],
        })
    );

    let request = RequestBuilder::new().path("/nested/page.html").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            route: "/{*path}.html",
            expanded: None,
            parameters: smallvec![("path", "nested/page")],
        })
    );

    let request = RequestBuilder::new().path("/.html").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_wildcard_greedy() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/{*first}-{*second}").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router.path, @r"
    /
    ╰─ {*first}
       ╰─ -
          ╰─ {*second} [*]
    ");

    let request = RequestBuilder::new().path("/a-b-c").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            route: "/{*first}-{*second}",
            expanded: None,
            parameters: smallvec![("first", "a-b"), ("second", "c")],
        })
    );

    let request = RequestBuilder::new()
        .path("/path/to/some-file/with-multiple-hyphens")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            route: "/{*first}-{*second}",
            expanded: None,
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

    let route = RouteBuilder::new().route("/{*path}/end").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router.path, @r"
    /
    ╰─ {*path}
       ╰─ /end [*]
    ");

    let request = RequestBuilder::new().path("/start/middle/end").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            route: "/{*path}/end",
            expanded: None,
            parameters: smallvec![("path", "start/middle")],
        })
    );

    let request = RequestBuilder::new().path("/start//middle///end").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            route: "/{*path}/end",
            expanded: None,
            parameters: smallvec![("path", "start//middle//")],
        })
    );

    Ok(())
}

#[test]
fn test_wildcard_priority() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/static/path").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/static/{*rest}").build()?;
    router.insert(&route, 2)?;
    let route = RouteBuilder::new().route("/{*path}/static").build()?;
    router.insert(&route, 3)?;
    let route = RouteBuilder::new().route("/prefix.{*suffix}").build()?;
    router.insert(&route, 4)?;
    let route = RouteBuilder::new().route("/{*prefix}.suffix").build()?;
    router.insert(&route, 5)?;

    insta::assert_snapshot!(router.path, @r"
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

    let request = RequestBuilder::new().path("/static/path").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            route: "/static/path",
            expanded: None,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new()
        .path("/static/some/nested/path")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            route: "/static/{*rest}",
            expanded: None,
            parameters: smallvec![("rest", "some/nested/path")],
        })
    );

    let request = RequestBuilder::new()
        .path("/some/nested/path/static")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &3,
            route: "/{*path}/static",
            expanded: None,
            parameters: smallvec![("path", "some/nested/path")],
        })
    );

    let request = RequestBuilder::new()
        .path("/prefix.some/nested/path")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &4,
            route: "/prefix.{*suffix}",
            expanded: None,
            parameters: smallvec![("suffix", "some/nested/path")],
        })
    );

    let request = RequestBuilder::new()
        .path("/some/nested/path.suffix")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &5,
            route: "/{*prefix}.suffix",
            expanded: None,
            parameters: smallvec![("prefix", "some/nested/path")],
        })
    );

    Ok(())
}
