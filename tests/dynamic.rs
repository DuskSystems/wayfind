use smallvec::smallvec;
use std::error::Error;
use wayfind::{Match, Path, Router};

#[test]
fn test_dynamic_simple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{id}", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ {id} [*]
    ");

    let path = Path::new("/123")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{id}",
            expanded: None,
            data: &1,
            parameters: smallvec![("id", "123")],
        })
    );

    let path = Path::new("/")?;
    let search = router.search(&path)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_dynamic_multiple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{year}", 1)?;
    router.insert("/{year}/{month}", 2)?;
    router.insert("/{year}/{month}/{day}", 3)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ {year} [*]
       ╰─ /
          ╰─ {month} [*]
             ╰─ /
                ╰─ {day} [*]
    ");

    let path = Path::new("/2024")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{year}",
            expanded: None,
            data: &1,
            parameters: smallvec![("year", "2024")],
        })
    );

    let path = Path::new("/2024/12")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{year}/{month}",
            expanded: None,
            data: &2,
            parameters: smallvec![("year", "2024"), ("month", "12")],
        })
    );

    let path = Path::new("/2024/12/01")?;
    let search = router.search(&path)?;
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
    router.insert("/{year}", 1)?;
    router.insert("/{year}-{month}", 2)?;
    router.insert("/{year}-{month}-{day}", 3)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ {year} [*]
       ╰─ -
          ╰─ {month} [*]
             ╰─ -
                ╰─ {day} [*]
    ");

    let path = Path::new("/2024")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{year}",
            expanded: None,
            data: &1,
            parameters: smallvec![("year", "2024")],
        })
    );

    let path = Path::new("/2024-12")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{year}-{month}",
            expanded: None,
            data: &2,
            parameters: smallvec![("year", "2024"), ("month", "12")],
        })
    );

    let path = Path::new("/2024-12-01")?;
    let search = router.search(&path)?;
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
    router.insert("/{file}.{extension}", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ {file}
       ╰─ .
          ╰─ {extension} [*]
    ");

    let path = Path::new("/report")?;
    let search = router.search(&path)?;
    assert_eq!(search, None);

    let path = Path::new("/report.pdf")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{file}.{extension}",
            expanded: None,
            data: &1,
            parameters: smallvec![("file", "report"), ("extension", "pdf")],
        })
    );

    let path = Path::new("/report.final.pdf")?;
    let search = router.search(&path)?;
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

// FIXME: Should really be an error at insert time.
#[test]
fn test_dynamic_touching() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{first}{second}", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ {first}
       ╰─ {second} [*]
    ");

    let path = Path::new("/hello")?;
    let search = router.search(&path)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_dynamic_priority() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/robots.txt", 1)?;
    router.insert("/robots.{extension}", 2)?;
    router.insert("/{name}.txt", 3)?;
    router.insert("/{name}.{extension}", 4)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ robots.
    │  ├─ txt [*]
    │  ╰─ {extension} [*]
    ╰─ {name}
       ╰─ .
          ├─ txt [*]
          ╰─ {extension} [*]
    ");

    let path = Path::new("/robots.txt")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/robots.txt",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/robots.pdf")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/robots.{extension}",
            expanded: None,
            data: &2,
            parameters: smallvec![("extension", "pdf")],
        })
    );

    let path = Path::new("/config.txt")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{name}.txt",
            expanded: None,
            data: &3,
            parameters: smallvec![("name", "config")],
        })
    );

    let path = Path::new("/config.pdf")?;
    let search = router.search(&path)?;
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
