use smallvec::smallvec;
use std::error::Error;
use wayfind::{Match, Path, Router};

#[test]
fn test_static_simple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users", 1)?;

    insta::assert_snapshot!(router, @"/users [*]");

    let path = Path::new("/users")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/user")?;
    let search = router.search(&path)?;
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

    let path = Path::new("/user")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/user",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/users")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users",
            data: &2,
            expanded: None,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/use")?;
    let search = router.search(&path)?;
    assert_eq!(search, None);

    let path = Path::new("/userss")?;
    let search = router.search(&path)?;
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

    let path = Path::new("/user_1")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/user_1",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/user/1")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/user/1",
            data: &2,
            expanded: None,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/user")?;
    let search = router.search(&path)?;
    assert_eq!(search, None);

    let path = Path::new("/users")?;
    let search = router.search(&path)?;
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

    let path = Path::new("/👨‍👩‍👧")?; // Family: Man, Woman, Girl
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/👨‍👩‍👧",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/👨‍👩‍👦")?; // Family: Man, Woman, Boy
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/👨‍👩‍👦",
            expanded: None,
            data: &2,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/👨")?; // Man
    let search = router.search(&path)?;
    assert_eq!(search, None);

    let path = Path::new("/👨‍👨")?; // Man Woman
    let search = router.search(&path)?;
    assert_eq!(search, None);

    let path = Path::new("/👨👩👧")?; // Man, Woman, Girl
    let search = router.search(&path)?;
    assert_eq!(search, None);

    let path = Path::new("/👨‍👨‍👧‍👦")?; // Family: Man, Woman, Girl, Boy
    let search = router.search(&path)?;
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

    let path = Path::new("/users")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/Users")?;
    let search = router.search(&path)?;
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
    router.insert("/users /items", 1)?;

    insta::assert_snapshot!(router, @"/users /items [*]");

    let path = Path::new("/users /items")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users /items",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/users/items")?;
    let search = router.search(&path)?;
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

    let path = Path::new("/users/items")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users/items",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/users//items")?;
    let search = router.search(&path)?;
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
    router.insert("/users///items", 1)?;

    insta::assert_snapshot!(router, @"/users///items [*]");

    let path = Path::new("/users///items")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users///items",
            expanded: None,
            data: &1,
            parameters: smallvec![],
        })
    );

    let path = Path::new("/users/items")?;
    let search = router.search(&path)?;
    assert_eq!(search, None);

    let path = Path::new("/users//items")?;
    let search = router.search(&path)?;
    assert_eq!(search, None);

    let path = Path::new("/users////items")?;
    let search = router.search(&path)?;
    assert_eq!(search, None);

    Ok(())
}
