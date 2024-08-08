use regex::Regex;
use wayfind::{constraints::parameter::ParameterConstraint, route::RouteBuilder, router::Router};

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
    router.insert(
        RouteBuilder::new("/repos/<id>")
            .parameter_constraint("id", ParameterConstraint::Regex(Regex::new(r"[a-f0-9]{32}")?))
            .build()?,
        8,
    )?;

    // Regex Inline
    router.insert(
        RouteBuilder::new("/repos/<id>/archive/v<version>")
            .parameter_constraint("id", ParameterConstraint::Regex(Regex::new(r"[a-f0-9]{32}")?))
            .parameter_constraint(
                "version",
                ParameterConstraint::Regex(Regex::new(r"[0-9]+\.[0-9]+\.[0-9]+")?),
            )
            .build()?,
        9,
    )?;

    // Multiple Regex Inline
    router.insert(
        RouteBuilder::new("/repos/<id>/compare/<base>..<head>")
            .parameter_constraint("id", ParameterConstraint::Regex(Regex::new(r"[a-f0-9]{32}")?))
            .parameter_constraint("base", ParameterConstraint::Regex(Regex::new(r"[a-f0-9]{40}")?))
            .parameter_constraint("head", ParameterConstraint::Regex(Regex::new(r"[a-f0-9]{40}")?))
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
    │  │       ╰─ <id> [8] [ParameterConstraint::Regex([a-f0-9]{32})]
    │  │             ╰─ /
    │  │                ├─ archive/v
    │  │                │          ╰─ <version> [9] [ParameterConstraint::Regex([0-9]+\.[0-9]+\.[0-9]+)]
    │  │                ╰─ compare/
    │  │                          ╰─ <base> [ParameterConstraint::Regex([a-f0-9]{40})]
    │  │                                  ╰─ ..
    │  │                                      ╰─ <head> [10] [ParameterConstraint::Regex([a-f0-9]{40})]
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
