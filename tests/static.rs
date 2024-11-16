use smallvec::smallvec;
use std::error::Error;
use wayfind::{Match, RequestBuilder, RouteBuilder, Router};

#[test]
fn test_static_simple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router.path, @"/users [*]");

    let request = RequestBuilder::new().path("/users").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/user").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_static_overlapping() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/user").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/users").build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router.path, @r"
    /user [*]
    ╰─ s [*]
    ");

    let request = RequestBuilder::new().path("/user").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/user",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/users").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users",
            data: &2,
            expanded: None,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/use").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    let request = RequestBuilder::new().path("/userss").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_static_overlapping_slash() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/user_1").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/user/1").build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router.path, @r"
    /user
    ├─ /1 [*]
    ╰─ _1 [*]
    ");

    let request = RequestBuilder::new().path("/user_1").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/user_1",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/user/1").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/user/1",
            data: &2,
            expanded: None,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/user").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    let request = RequestBuilder::new().path("/users").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_static_split_multibyte() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/👨‍👩‍👧").build()?; // Family: Man, Woman, Girl
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/👨‍👩‍👦").build()?; // Family: Man, Woman, Boy
    router.insert(&route, 2)?;
    let route = RouteBuilder::new().route("/👩‍👩‍👧").build()?; // Family: Woman, Woman, Girl
    router.insert(&route, 3)?;
    let route = RouteBuilder::new().route("/👩‍👩‍👦").build()?; // Family: Woman, Woman, Boy
    router.insert(&route, 4)?;
    let route = RouteBuilder::new().route("/👨‍👨‍👧").build()?; // Family: Man, Man, Girl
    router.insert(&route, 5)?;
    let route = RouteBuilder::new().route("/👨‍👨‍👦").build()?; // Family: Man, Man, Boy
    router.insert(&route, 6)?;

    insta::assert_snapshot!(router.path, @r"
    /�
    ├─ �‍👩‍�
    │  ├─ � [*]
    │  ╰─ � [*]
    ╰─ �‍�
       ├─ �‍�
       │  ├─ � [*]
       │  ╰─ � [*]
       ╰─ �‍�
          ├─ � [*]
          ╰─ � [*]
    ");

    let request = RequestBuilder::new().path("/👨‍👩‍👧").build()?; // Family: Man, Woman, Girl
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/👨‍👩‍👧",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/👨‍👩‍👦").build()?; // Family: Man, Woman, Boy
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/👨‍👩‍👦",
            expanded: None,
            data: &2,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/👨").build()?; // Man
    let search = router.search(&request)?;
    assert_eq!(search, None);

    let request = RequestBuilder::new().path("/👨‍👨").build()?; // Man Woman
    let search = router.search(&request)?;
    assert_eq!(search, None);

    let request = RequestBuilder::new().path("/👨👩👧").build()?; // Man, Woman, Girl
    let search = router.search(&request)?;
    assert_eq!(search, None);

    let request = RequestBuilder::new().path("/👨‍👨‍👧‍👦").build()?; // Family: Man, Woman, Girl, Boy
    let search = router.search(&request)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_static_case_sensitive() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/Users").build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router.path, @r"
    /
    ├─ Users [*]
    ╰─ users [*]
    ");

    let request = RequestBuilder::new().path("/users").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/Users").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/Users",
            expanded: None,
            data: &2,
            parameters: smallvec![],
        })
    );

    Ok(())
}

#[test]
fn test_static_whitespace() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users /items").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router.path, @"/users /items [*]");

    let request = RequestBuilder::new().path("/users /items").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users /items",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/users/items").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_static_duplicate_slashes() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users/items").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/users//items").build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router.path, @r"
    /users/
    ├─ /items [*]
    ╰─ items [*]
    ");

    let request = RequestBuilder::new().path("/users/items").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users/items",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/users//items").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users//items",
            expanded: None,
            data: &2,
            parameters: smallvec![],
        })
    );

    Ok(())
}

#[test]
fn test_static_empty_segments() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users///items").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router.path, @"/users///items [*]");

    let request = RequestBuilder::new().path("/users///items").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users///items",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/users/items").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    let request = RequestBuilder::new().path("/users//items").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    let request = RequestBuilder::new().path("/users////items").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    Ok(())
}
