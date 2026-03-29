#![expect(missing_docs, reason = "Tests")]

use core::error::Error;

use similar_asserts::assert_eq;
use smallvec::smallvec;
use wayfind::{Match, RouterBuilder};

#[test]
fn static_simple() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/users", 1)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @"/users");

    let search = router.search("/users");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users",
            parameters: smallvec![],
        })
    );

    let search = router.search("/user");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn static_overlapping() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/user", 1)?;
    builder.insert("/users", 2)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /user
    ╰─ s
    ");

    let search = router.search("/user");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/user",
            parameters: smallvec![],
        })
    );

    let search = router.search("/users");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/users",
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
fn static_overlapping_slash() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/user_1", 1)?;
    builder.insert("/user/1", 2)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @"
    /user
    ├─ /1
    ╰─ _1
    ");

    let search = router.search("/user_1");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/user_1",
            parameters: smallvec![],
        })
    );

    let search = router.search("/user/1");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/user/1",
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
fn static_split_multibyte() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();

    builder.insert("/👨‍👩‍👧", 1)?; // Family: Man, Woman, Girl
    builder.insert("/👨‍👩‍👦", 2)?; // Family: Man, Woman, Boy
    builder.insert("/👩‍👩‍👧", 3)?; // Family: Woman, Woman, Girl
    builder.insert("/👩‍👩‍👦", 4)?; // Family: Woman, Woman, Boy
    builder.insert("/👨‍👨‍👧", 5)?; // Family: Man, Man, Girl
    builder.insert("/👨‍👨‍👦", 6)?; // Family: Man, Man, Boy

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /�
    ├─ �‍�
    │  ├─ �‍�
    │  │  ├─ �
    │  │  ╰─ �
    │  ╰─ �‍�
    │     ├─ �
    │     ╰─ �
    ╰─ �‍👩‍�
       ├─ �
       ╰─ �
    ");

    let search = router.search("/👨‍👩‍👧"); // Family: Man, Woman, Girl
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/👨‍👩‍👧",
            parameters: smallvec![],
        })
    );

    let search = router.search("/👨‍👩‍👦"); // Family: Man, Woman, Boy
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/👨‍👩‍👦",
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
fn static_case_sensitive() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/users", 1)?;
    builder.insert("/Users", 2)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @"
    /
    ├─ Users
    ╰─ users
    ");

    let search = router.search("/users");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users",
            parameters: smallvec![],
        })
    );

    let search = router.search("/Users");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/Users",
            parameters: smallvec![],
        })
    );

    Ok(())
}

#[test]
fn static_whitespace() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/users /items", 1)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @"/users /items");

    let search = router.search("/users /items");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users /items",
            parameters: smallvec![],
        })
    );

    let search = router.search("/users/items");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn static_duplicate_slashes() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/users/items", 1)?;
    builder.insert("/users//items", 2)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @"
    /users/
    ├─ /items
    ╰─ items
    ");

    let search = router.search("/users/items");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users/items",
            parameters: smallvec![],
        })
    );

    let search = router.search("/users//items");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/users//items",
            parameters: smallvec![],
        })
    );

    Ok(())
}

#[test]
fn static_empty_segments() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/users///items", 1)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @"/users///items");

    let search = router.search("/users///items");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users///items",
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
