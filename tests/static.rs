use similar_asserts::assert_eq;
use smallvec::smallvec;
use std::error::Error;
use wayfind::{
    AuthorityMatch, Match, MethodMatch, PathMatch, RequestBuilder, RouteBuilder, Router,
};

#[test]
fn test_static_simple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let request = RequestBuilder::new().path("/users").build()?;
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
                route: "/users",
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
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

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /user [1]
    ╰─ s [2]
    === Method
    Empty
    === Chains
    *-1-*
    *-2-*
    ");

    let request = RequestBuilder::new().path("/user").build()?;
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
                route: "/user",
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/users").build()?;
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
                route: "/users",
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
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

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /user
    ├─ /1 [2]
    ╰─ _1 [1]
    === Method
    Empty
    === Chains
    *-1-*
    *-2-*
    ");

    let request = RequestBuilder::new().path("/user_1").build()?;
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
                route: "/user_1",
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/user/1").build()?;
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
                route: "/user/1",
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
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

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /�
    ├─ �‍👩‍�
    │  ├─ � [4]
    │  ╰─ � [3]
    ╰─ �‍�
       ├─ �‍�
       │  ├─ � [6]
       │  ╰─ � [5]
       ╰─ �‍�
          ├─ � [2]
          ╰─ � [1]
    === Method
    Empty
    === Chains
    *-1-*
    *-2-*
    *-3-*
    *-4-*
    *-5-*
    *-6-*
    ");

    let request = RequestBuilder::new().path("/👨‍👩‍👧").build()?; // Family: Man, Woman, Girl
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
                route: "/👨‍👩‍👧",
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/👨‍👩‍👦").build()?; // Family: Man, Woman, Boy
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
                route: "/👨‍👩‍👦",
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
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

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /
    ├─ Users [2]
    ╰─ users [1]
    === Method
    Empty
    === Chains
    *-1-*
    *-2-*
    ");

    let request = RequestBuilder::new().path("/users").build()?;
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
                route: "/users",
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/Users").build()?;
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
                route: "/Users",
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    Ok(())
}

#[test]
fn test_static_whitespace() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users /items").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users /items [1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let request = RequestBuilder::new().path("/users /items").build()?;
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
                route: "/users /items",
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
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

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users/
    ├─ /items [2]
    ╰─ items [1]
    === Method
    Empty
    === Chains
    *-1-*
    *-2-*
    ");

    let request = RequestBuilder::new().path("/users/items").build()?;
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
                route: "/users/items",
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/users//items").build()?;
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
                route: "/users//items",
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    Ok(())
}

#[test]
fn test_static_empty_segments() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users///items").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users///items [1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let request = RequestBuilder::new().path("/users///items").build()?;
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
                route: "/users///items",
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
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
