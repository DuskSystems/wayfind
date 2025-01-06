use std::error::Error;

use similar_asserts::assert_eq;
use smallvec::smallvec;
use wayfind::{Match, Router};

#[test]
fn test_static_simple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users", 1)?;

    insta::assert_snapshot!(router, @"/users [*]");

    let search = router.search("/users");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users",
            expanded: None,
            parameters: smallvec![],
        })
    );

    let search = router.search("/user");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_static_overlapping() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/user", 1)?;
    router.insert("/users", 2)?;

    insta::assert_snapshot!(router, @r"
    /user [*]
    ╰─ s [*]
    ");

    let search = router.search("/user");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/user",
            expanded: None,
            parameters: smallvec![],
        })
    );

    let search = router.search("/users");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/users",
            expanded: None,
            parameters: smallvec![],
        })
    );

    let search = router.search("/use");
    assert_eq!(search, None);

    let search = router.search("/userss");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_static_overlapping_slash() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/user_1", 1)?;
    router.insert("/user/1", 2)?;

    insta::assert_snapshot!(router, @r"
    /user
    ├─ /1 [*]
    ╰─ _1 [*]
    ");

    let search = router.search("/user_1");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/user_1",
            expanded: None,
            parameters: smallvec![],
        })
    );

    let search = router.search("/user/1");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/user/1",
            expanded: None,
            parameters: smallvec![],
        })
    );

    let search = router.search("/user");
    assert_eq!(search, None);

    let search = router.search("/users");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_static_split_multibyte() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    router.insert("/👨‍👩‍👧", 1)?; // Family: Man, Woman, Girl
    router.insert("/👨‍👩‍👦", 2)?; // Family: Man, Woman, Boy
    router.insert("/👩‍👩‍👧", 3)?; // Family: Woman, Woman, Girl
    router.insert("/👩‍👩‍👦", 4)?; // Family: Woman, Woman, Boy
    router.insert("/👨‍👨‍👧", 5)?; // Family: Man, Man, Girl
    router.insert("/👨‍👨‍👦", 6)?; // Family: Man, Man, Boy

    insta::assert_snapshot!(router, @r"
    /�
    ├─ �‍�
    │  ├─ �‍�
    │  │  ├─ � [*]
    │  │  ╰─ � [*]
    │  ╰─ �‍�
    │     ├─ � [*]
    │     ╰─ � [*]
    ╰─ �‍👩‍�
       ├─ � [*]
       ╰─ � [*]
    ");

    let search = router.search("/👨‍👩‍👧"); // Family: Man, Woman, Girl
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/👨‍👩‍👧",
            expanded: None,
            parameters: smallvec![],
        })
    );

    let search = router.search("/👨‍👩‍👦"); // Family: Man, Woman, Boy
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/👨‍👩‍👦",
            expanded: None,
            parameters: smallvec![],
        })
    );

    let search = router.search("/👨"); // Man
    assert_eq!(search, None);

    let search = router.search("/👨‍👨"); // Man Woman
    assert_eq!(search, None);

    let search = router.search("/👨👩👧"); // Man, Woman, Girl
    assert_eq!(search, None);

    let search = router.search("/👨‍👨‍👧‍👦"); // Family: Man, Woman, Girl, Boy
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_static_case_sensitive() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users", 1)?;
    router.insert("/Users", 2)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ Users [*]
    ╰─ users [*]
    ");

    let search = router.search("/users");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users",
            expanded: None,
            parameters: smallvec![],
        })
    );

    let search = router.search("/Users");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/Users",
            expanded: None,
            parameters: smallvec![],
        })
    );

    Ok(())
}

#[test]
fn test_static_whitespace() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users /items", 1)?;

    insta::assert_snapshot!(router, @"/users /items [*]");

    let search = router.search("/users /items");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users /items",
            expanded: None,
            parameters: smallvec![],
        })
    );

    let search = router.search("/users/items");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_static_duplicate_slashes() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users/items", 1)?;
    router.insert("/users//items", 2)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ├─ /items [*]
    ╰─ items [*]
    ");

    let search = router.search("/users/items");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users/items",
            expanded: None,
            parameters: smallvec![],
        })
    );

    let search = router.search("/users//items");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/users//items",
            expanded: None,
            parameters: smallvec![],
        })
    );

    Ok(())
}

#[test]
fn test_static_empty_segments() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users///items", 1)?;

    insta::assert_snapshot!(router, @"/users///items [*]");

    let search = router.search("/users///items");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users///items",
            expanded: None,
            parameters: smallvec![],
        })
    );

    let search = router.search("/users/items");
    assert_eq!(search, None);

    let search = router.search("/users//items");
    assert_eq!(search, None);

    let search = router.search("/users////items");
    assert_eq!(search, None);

    Ok(())
}
