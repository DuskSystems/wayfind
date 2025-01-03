use similar_asserts::assert_eq;
use smallvec::smallvec;
use std::error::Error;
use wayfind::{
    AuthorityMatch, Match, MethodMatch, PathMatch, RequestBuilder, RouteBuilder, Router,
};

#[test]
fn test_dynamic_simple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/{id}").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /
    ╰─ {id} [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let request = RequestBuilder::new().path("/123").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/{id}".into(),
                expanded: None,
                parameters: smallvec![("id", "123")],
            },
            method: MethodMatch { method: None }
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

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /
    ╰─ {year} [*:1]
       ╰─ /
          ╰─ {month} [*:2]
             ╰─ /
                ╰─ {day} [*:3]
    === Method
    Empty
    === Chains
    *-1-*
    *-2-*
    *-3-*
    ");

    let request = RequestBuilder::new().path("/2024").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/{year}".into(),
                expanded: None,
                parameters: smallvec![("year", "2024")],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/2024/12").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/{year}/{month}".into(),
                expanded: None,
                parameters: smallvec![("year", "2024"), ("month", "12")],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/2024/12/01").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &3,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/{year}/{month}/{day}".into(),
                expanded: None,
                parameters: smallvec![("year", "2024"), ("month", "12"), ("day", "01")],
            },
            method: MethodMatch { method: None }
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

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /
    ╰─ {year} [*:1]
       ╰─ -
          ╰─ {month} [*:2]
             ╰─ -
                ╰─ {day} [*:3]
    === Method
    Empty
    === Chains
    *-1-*
    *-2-*
    *-3-*
    ");

    let request = RequestBuilder::new().path("/2024").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/{year}".into(),
                expanded: None,
                parameters: smallvec![("year", "2024")],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/2024-12").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/{year}-{month}".into(),
                expanded: None,
                parameters: smallvec![("year", "2024"), ("month", "12")],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/2024-12-01").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &3,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/{year}-{month}-{day}".into(),
                expanded: None,
                parameters: smallvec![("year", "2024"), ("month", "12"), ("day", "01")],
            },
            method: MethodMatch { method: None }
        })
    );

    Ok(())
}

#[test]
fn test_dynamic_greedy() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/{file}.{extension}").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /
    ╰─ {file}
       ╰─ .
          ╰─ {extension} [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let request = RequestBuilder::new().path("/report").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    let request = RequestBuilder::new().path("/report.pdf").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/{file}.{extension}".into(),
                expanded: None,
                parameters: smallvec![("file", "report"), ("extension", "pdf")],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/report.final.pdf").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/{file}.{extension}".into(),
                expanded: None,
                parameters: smallvec![("file", "report.final"), ("extension", "pdf")],
            },
            method: MethodMatch { method: None }
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

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /
    ├─ robots.
    │  ├─ txt [*:1]
    │  ╰─ {extension} [*:2]
    ╰─ {name}
       ╰─ .
          ├─ txt [*:3]
          ╰─ {extension} [*:4]
    === Method
    Empty
    === Chains
    *-1-*
    *-2-*
    *-3-*
    *-4-*
    ");

    let request = RequestBuilder::new().path("/robots.txt").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/robots.txt".into(),
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/robots.pdf").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/robots.{extension}".into(),
                expanded: None,
                parameters: smallvec![("extension", "pdf")],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/config.txt").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &3,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/{name}.txt".into(),
                expanded: None,
                parameters: smallvec![("name", "config")],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/config.pdf").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &4,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/{name}.{extension}".into(),
                expanded: None,
                parameters: smallvec![("name", "config"), ("extension", "pdf")],
            },
            method: MethodMatch { method: None }
        })
    );

    Ok(())
}
