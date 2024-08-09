use wayfind::{route::RouteBuilder, router::Router};

fn is_hex_32(segment: &str) -> bool {
    segment.len() == 32
        && segment
            .chars()
            .all(|c| c.is_ascii_hexdigit())
}

fn is_semver(segment: &str) -> bool {
    let parts: Vec<&str> = segment.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| part.parse::<u32>().is_ok())
}

fn is_hex_40(segment: &str) -> bool {
    segment.len() == 40
        && segment
            .chars()
            .all(|c| c.is_ascii_hexdigit())
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
            .constraint("id", is_hex_32)
            .build()?,
        8,
    )?;

    // Multiple Constraints
    router.insert(
        RouteBuilder::new("/repos/<id>/archive/v<version>")
            .constraint("id", is_hex_32)
            .constraint("version", is_semver)
            .build()?,
        9,
    )?;

    // Multiple Constraints Inline
    router.insert(
        RouteBuilder::new("/repos/<id>/compare/<base>..<head>")
            .constraint("id", is_hex_32)
            .constraint("base", is_hex_40)
            .constraint("head", is_hex_40)
            .build()?,
        10,
    )?;

    // Catch All
    router.insert("<catch_all:*>", 11)?;

    insta::assert_snapshot!(router, @r###"
    $
    ├─ / [1]
    │  ├─ users/
    │  │       ╰─ <username> [2]
    │  ├─ avatars/
    │  │         ╰─ <username>
    │  │                     ╰─ .
    │  │                        ├─ png [3]
    │  │                        ╰─ <extension> [4]
    │  ├─ repos/
    │  │       ╰─ <id> [8] [Constraint]
    │  │             ╰─ /
    │  │                ├─ archive/v
    │  │                │          ╰─ <version> [9] [Constraint]
    │  │                ╰─ compare/
    │  │                          ╰─ <base> [Constraint]
    │  │                                  ╰─ ..
    │  │                                      ╰─ <head> [10] [Constraint]
    │  ╰─ <namespace:*>
    │                 ╰─ /
    │                    ╰─ <repository> [5]
    │                                  ╰─ /
    │                                     ╰─ <file:*> [6]
    ╰─ <catch_all:*> [11]
    "###);

    Ok(())
}
