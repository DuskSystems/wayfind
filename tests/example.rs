use wayfind::{node::Constraint, route::RouteBuilder, router::Router};

struct Hex32;
impl Constraint for Hex32 {
    fn name() -> &'static str {
        "hex32"
    }

    fn check(segment: &str) -> bool {
        segment.len() == 32
            && segment
                .chars()
                .all(|c| c.is_ascii_hexdigit())
    }
}

struct Semver;
impl Constraint for Semver {
    fn name() -> &'static str {
        "semver"
    }

    fn check(segment: &str) -> bool {
        let parts: Vec<&str> = segment.split('.').collect();
        parts.len() == 3
            && parts
                .iter()
                .all(|part| part.parse::<u32>().is_ok())
    }
}

struct Hex40;
impl Constraint for Hex40 {
    fn name() -> &'static str {
        "hex40"
    }

    fn check(segment: &str) -> bool {
        segment.len() == 40
            && segment
                .chars()
                .all(|c| c.is_ascii_hexdigit())
    }
}

#[test]
fn example() -> Result<(), Box<dyn std::error::Error>> {
    let mut router = Router::new();

    // Static route
    router.insert("/", 1)?;

    // Dynamic Segment
    router.insert("/users/<username>", 2)?;

    // Dynamic Inline
    router.insert("/avatars/<username>.png", 3)?;

    // Multiple Dynamic Inline
    router.insert("/avatars/<username>.<extension>", 4)?;

    // Wildcard Segment
    router.insert("/<namespace:*>/<repository>", 5)?;

    // Multiple Wildcard Segments
    router.insert("/<namespace:*>/<repository>/<file:*>", 6)?;

    // Constraint
    router.insert(
        RouteBuilder::new("/repos/<id>")
            .constraint::<Hex32>("id")
            .build()?,
        8,
    )?;

    // Multiple Constraints
    router.insert(
        RouteBuilder::new("/repos/<id>/archive/v<version>")
            .constraint::<Hex32>("id")
            .constraint::<Semver>("version")
            .build()?,
        9,
    )?;

    // Multiple Constraints Inline
    router.insert(
        RouteBuilder::new("/repos/<id>/compare/<base>..<head>")
            .constraint::<Hex32>("id")
            .constraint::<Hex40>("base")
            .constraint::<Hex40>("head")
            .build()?,
        10,
    )?;

    // Catch All
    router.insert("<catch_all:*>", 11)?;

    insta::assert_snapshot!(router, @r###"
    $
    ├─ / [1]
    │  ├─ avatars/
    │  │         ╰─ <username>
    │  │                     ╰─ .
    │  │                        ├─ png [3]
    │  │                        ╰─ <extension> [4]
    │  ├─ repos/
    │  │       ╰─ <id> [8] (hex32)
    │  │             ╰─ /
    │  │                ├─ archive/v
    │  │                │          ╰─ <version> [9] (semver)
    │  │                ╰─ compare/
    │  │                          ╰─ <base> (hex40)
    │  │                                  ╰─ ..
    │  │                                      ╰─ <head> [10] (hex40)
    │  ├─ users/
    │  │       ╰─ <username> [2]
    │  ╰─ <namespace:*>
    │                 ╰─ /
    │                    ╰─ <repository> [5]
    │                                  ╰─ /
    │                                     ╰─ <file:*> [6]
    ╰─ <catch_all:*> [11]
    "###);

    Ok(())
}
