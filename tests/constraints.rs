#![allow(clippy::too_many_lines)]

use std::error::Error;
use wayfind::{Constraint, Router};

#[path = "./common.rs"]
mod common;

struct LengthBetween3And10;
impl Constraint for LengthBetween3And10 {
    const NAME: &'static str = "length_3_to_10";

    fn check(segment: &str) -> bool {
        (3..=10).contains(&segment.len())
    }
}

struct Year1000To10000;
impl Constraint for Year1000To10000 {
    const NAME: &'static str = "year_1000_to_10000";

    fn check(segment: &str) -> bool {
        segment
            .parse::<u32>()
            .map(|num| (1000..=10000).contains(&num))
            .unwrap_or(false)
    }
}

struct PngOrJpg;
impl Constraint for PngOrJpg {
    const NAME: &'static str = "png_or_jpg";

    fn check(segment: &str) -> bool {
        segment == "png" || segment == "jpg"
    }
}

struct EvenYear;
impl Constraint for EvenYear {
    const NAME: &'static str = "even_year";

    fn check(segment: &str) -> bool {
        segment
            .parse::<i32>()
            .map(|year| year % 2 == 0)
            .unwrap_or(false)
    }
}

struct ValidSlug;
impl Constraint for ValidSlug {
    const NAME: &'static str = "valid_slug";

    fn check(segment: &str) -> bool {
        !segment.is_empty()
            && (3..=10).contains(&segment.len())
            && segment
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    }
}

#[test]
fn test_multiple_constraints() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    router.constraint::<LengthBetween3And10>()?;
    router.constraint::<Year1000To10000>()?;
    router.constraint::<PngOrJpg>()?;
    router.constraint::<EvenYear>()?;
    router.constraint::<ValidSlug>()?;

    router.insert("/user/{name:length_3_to_10}/{id:year_1000_to_10000}", 1)?;
    router.insert("/profile/{username:length_3_to_10}.{ext:png_or_jpg}", 2)?;
    router.insert("/posts/{year:even_year}/{slug:valid_slug}", 3)?;

    insta::assert_snapshot!(router, @r###"
    ▽
    ╰─ /
       ├─ p
       │  ├─ osts/
       │  │      ╰─ {year:even_year}
       │  │                        ╰─ /
       │  │                           ╰─ {slug:valid_slug} ○
       │  ╰─ rofile/
       │           ╰─ {username:length_3_to_10}
       │                                      ╰─ .
       │                                         ╰─ {ext:png_or_jpg} ○
       ╰─ user/
              ╰─ {name:length_3_to_10}
                                     ╰─ /
                                        ╰─ {id:year_1000_to_10000} ○
    "###);

    assert_router_matches!(router, {
        "/user/john/1234" => {
            route: "/user/{name:length_3_to_10}/{id:year_1000_to_10000}",
            data: 1,
            params: {
                "name" => "john",
                "id" => "1234"
            }
        }
        "/user/johndoe/10000" => {
            route: "/user/{name:length_3_to_10}/{id:year_1000_to_10000}",
            data: 1,
            params: {
                "name" => "johndoe",
                "id" => "10000"
            }
        }
        "/user/j/1234" => None
        "/user/johndoetolong/1234" => None
        "/user/john/123" => None
        "/user/john/10001" => None

        "/profile/alice.png" => {
            route: "/profile/{username:length_3_to_10}.{ext:png_or_jpg}",
            data: 2,
            params: {
                "username" => "alice",
                "ext" => "png"
            }
        }
        "/profile/bob.jpg" => {
            route: "/profile/{username:length_3_to_10}.{ext:png_or_jpg}",
            data: 2,
            params: {
                "username" => "bob",
                "ext" => "jpg"
            }
        }
        "/profile/a.png" => None
        "/profile/toolongusername.png" => None
        "/profile/alice.gif" => None

        "/posts/2022/hello" => {
            route: "/posts/{year:even_year}/{slug:valid_slug}",
            data: 3,
            params: {
                "year" => "2022",
                "slug" => "hello"
            }
        }
        "/posts/2024/test-123" => {
            route: "/posts/{year:even_year}/{slug:valid_slug}",
            data: 3,
            params: {
                "year" => "2024",
                "slug" => "test-123"
            }
        }
        "/posts/2023/hello" => None
        "/posts/2022/toolongslug" => None
        "/posts/2022/INVALID" => None
    });

    Ok(())
}

#[test]
fn test_unknown_constraints() {
    let mut router = Router::new();

    let error = router.insert("/{name:unknown}", 0).unwrap_err();
    insta::assert_snapshot!(error, @r###"
    unknown constraint

       Constraint: unknown

    The router doesn't recognize this constraint
    "###);
}

struct ConstraintA;
impl Constraint for ConstraintA {
    const NAME: &'static str = "my_constraint";

    fn check(_segment: &str) -> bool {
        true
    }
}

struct ConstraintB;
impl Constraint for ConstraintB {
    const NAME: &'static str = "my_constraint";

    fn check(_segment: &str) -> bool {
        true
    }
}

#[test]
fn constraint_duplicate_name_error() -> Result<(), Box<dyn Error>> {
    let mut router: Router<usize> = Router::new();
    router.constraint::<ConstraintA>()?;

    let error = router.constraint::<ConstraintB>().unwrap_err();
    insta::assert_snapshot!(error, @r###"
    duplicate constraint name

    The constraint name 'my_constraint' is already in use:
        - existing constraint type: 'constraints::ConstraintA'
        - new constraint type: 'constraints::ConstraintB'

    help: each constraint must have a unique name

    try:
        - Check if you have accidentally added the same constraint twice
        - Ensure different constraints have different names
    "###);

    Ok(())
}
