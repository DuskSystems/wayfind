use smallvec::smallvec;
use std::error::Error;
use wayfind::{Match, RequestBuilder, RouteBuilder, Router};

#[test]
fn test_dynamic_simple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/{id}").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router.path, @r"
    /
    ╰─ {id} [*]
    ");

    let request = RequestBuilder::new().path("/123").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{id}",
            expanded: None,
            data: &1,
            parameters: smallvec![("id", "123")],
        })
    );

    let request = RequestBuilder::new().path("/").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_dynamic_multiple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/{year}").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/{year}/{month}").build()?;
    router.insert(&route, 2)?;
    let route = RouteBuilder::new().route("/{year}/{month}/{day}").build()?;
    router.insert(&route, 3)?;

    insta::assert_snapshot!(router.path, @r"
    /
    ╰─ {year} [*]
       ╰─ /
          ╰─ {month} [*]
             ╰─ /
                ╰─ {day} [*]
    ");

    let request = RequestBuilder::new().path("/2024").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{year}",
            expanded: None,
            data: &1,
            parameters: smallvec![("year", "2024")],
        })
    );

    let request = RequestBuilder::new().path("/2024/12").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{year}/{month}",
            expanded: None,
            data: &2,
            parameters: smallvec![("year", "2024"), ("month", "12")],
        })
    );

    let request = RequestBuilder::new().path("/2024/12/01").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{year}/{month}/{day}",
            expanded: None,
            data: &3,
            parameters: smallvec![("year", "2024"), ("month", "12"), ("day", "01")],
        })
    );

    Ok(())
}

#[test]
fn test_dynamic_inline() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/{year}").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/{year}-{month}").build()?;
    router.insert(&route, 2)?;
    let route = RouteBuilder::new().route("/{year}-{month}-{day}").build()?;
    router.insert(&route, 3)?;

    insta::assert_snapshot!(router.path, @r"
    /
    ╰─ {year} [*]
       ╰─ -
          ╰─ {month} [*]
             ╰─ -
                ╰─ {day} [*]
    ");

    let request = RequestBuilder::new().path("/2024").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{year}",
            expanded: None,
            data: &1,
            parameters: smallvec![("year", "2024")],
        })
    );

    let request = RequestBuilder::new().path("/2024-12").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{year}-{month}",
            expanded: None,
            data: &2,
            parameters: smallvec![("year", "2024"), ("month", "12")],
        })
    );

    let request = RequestBuilder::new().path("/2024-12-01").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{year}-{month}-{day}",
            expanded: None,
            data: &3,
            parameters: smallvec![("year", "2024"), ("month", "12"), ("day", "01")],
        })
    );

    Ok(())
}

#[test]
fn test_dynamic_greedy() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/{file}.{extension}").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router.path, @r"
    /
    ╰─ {file}
       ╰─ .
          ╰─ {extension} [*]
    ");

    let request = RequestBuilder::new().path("/report").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    let request = RequestBuilder::new().path("/report.pdf").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{file}.{extension}",
            expanded: None,
            data: &1,
            parameters: smallvec![("file", "report"), ("extension", "pdf")],
        })
    );

    let request = RequestBuilder::new().path("/report.final.pdf").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{file}.{extension}",
            expanded: None,
            data: &1,
            parameters: smallvec![("file", "report.final"), ("extension", "pdf")],
        })
    );

    Ok(())
}

#[test]
fn test_dynamic_priority() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/robots.txt").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/robots.{extension}").build()?;
    router.insert(&route, 2)?;
    let route = RouteBuilder::new().route("/{name}.txt").build()?;
    router.insert(&route, 3)?;
    let route = RouteBuilder::new().route("/{name}.{extension}").build()?;
    router.insert(&route, 4)?;

    insta::assert_snapshot!(router.path, @r"
    /
    ├─ robots.
    │  ├─ txt [*]
    │  ╰─ {extension} [*]
    ╰─ {name}
       ╰─ .
          ├─ txt [*]
          ╰─ {extension} [*]
    ");

    let request = RequestBuilder::new().path("/robots.txt").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/robots.txt",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/robots.pdf").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/robots.{extension}",
            expanded: None,
            data: &2,
            parameters: smallvec![("extension", "pdf")],
        })
    );

    let request = RequestBuilder::new().path("/config.txt").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{name}.txt",
            expanded: None,
            data: &3,
            parameters: smallvec![("name", "config")],
        })
    );

    let request = RequestBuilder::new().path("/config.pdf").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{name}.{extension}",
            expanded: None,
            data: &4,
            parameters: smallvec![("name", "config"), ("extension", "pdf")],
        })
    );

    Ok(())
}
