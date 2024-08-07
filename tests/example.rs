use regex::bytes::Regex;
use wayfind::{node::NodeConstraint, router::Router};

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

    // Regex Segment
    router.insert_with_constraints(
        "/repos/<id>",
        8,
        vec![("id", NodeConstraint::Regex(Regex::new(r"[a-f0-9]{32}")?))],
    )?;

    // Regex Inline
    router.insert_with_constraints(
        "/repos/<id>/archive/v<version>",
        9,
        vec![
            ("id", NodeConstraint::Regex(Regex::new(r"[a-f0-9]{32}")?)),
            ("version", NodeConstraint::Regex(Regex::new(r"[0-9]+\.[0-9]+\.[0-9]+")?)),
        ],
    )?;

    // Multiple Regex Inline
    router.insert_with_constraints(
        "/repos/<id>/compare/<base>..<head>",
        10,
        vec![
            ("id", NodeConstraint::Regex(Regex::new(r"[a-f0-9]{32}")?)),
            ("base", NodeConstraint::Regex(Regex::new(r"[a-f0-9]{40}")?)),
            ("head", NodeConstraint::Regex(Regex::new(r"[a-f0-9]{40}")?)),
        ],
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
    │  │       ╰─ <id> [8] Constraint::Regex: [a-f0-9]{32}
    │  │             ╰─ /
    │  │                ├─ archive/v
    │  │                │          ╰─ <version> [9] Constraint::Regex: [0-9]+\.[0-9]+\.[0-9]+
    │  │                ╰─ compare/
    │  │                          ╰─ <base> Constraint::Regex: [a-f0-9]{40}
    │  │                                  ╰─ ..
    │  │                                      ╰─ <head> [10] Constraint::Regex: [a-f0-9]{40}
    │  ╰─ <namespace:*>
    │                 ╰─ /
    │                    ╰─ <repository> [5]
    │                                  ╰─ /
    │                                     ╰─ <file:*> [6]
    ╰─ <catch_all:*> [11]
    "###);

    Ok(())
}
