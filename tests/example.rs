use std::error::Error;
use wayfind::{constraint::Constraint, router::Router};

struct Hex32;
impl Constraint for Hex32 {
    const NAME: &'static str = "hex32";

    fn check(segment: &str) -> bool {
        segment.len() == 32
            && segment
                .chars()
                .all(|c| c.is_ascii_hexdigit())
    }
}

struct Semver;
impl Constraint for Semver {
    const NAME: &'static str = "semver";

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
    const NAME: &'static str = "hex40";

    fn check(segment: &str) -> bool {
        segment.len() == 40
            && segment
                .chars()
                .all(|c| c.is_ascii_hexdigit())
    }
}

#[test]
fn example() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    router.constraint::<Hex32>()?;
    router.constraint::<Semver>()?;
    router.constraint::<Hex40>()?;

    // Static route
    router.insert("/", 1)?;

    // Dynamic Segment
    router.insert("/users/{username}", 2)?;

    // Dynamic Inline
    router.insert("/avatars/{username}.png", 3)?;

    // Multiple Dynamic Inline
    router.insert("/avatars/{username}.{extension}", 4)?;

    // Wildcard Segment
    router.insert("/{*namespace}/{repository}", 5)?;

    // Multiple Wildcard Segments
    router.insert("/{*namespace}/{repository}/{*file}", 6)?;

    // Constraint
    router.insert("/repos/{id:hex32}", 8)?;

    // Multiple Constraints
    router.insert("/repos/{id:hex32}/archive/v{version:semver}", 9)?;

    // Multiple Constraints Inline
    router.insert("/repos/{id:hex32}/compare/{base:hex40}..{head:hex40}", 10)?;

    // Catch All
    router.insert("{*catch_all}", 11)?;

    insta::assert_snapshot!(router, @r###"
    $
    ├─ / [1]
    │  ├─ avatars/
    │  │         ╰─ {username}
    │  │                     ╰─ .
    │  │                        ├─ png [3]
    │  │                        ╰─ {extension} [4]
    │  ├─ repos/
    │  │       ╰─ {id:hex32} [8]
    │  │                   ╰─ /
    │  │                      ├─ archive/v
    │  │                      │          ╰─ {version:semver} [9]
    │  │                      ╰─ compare/
    │  │                                ╰─ {base:hex40}
    │  │                                              ╰─ ..
    │  │                                                  ╰─ {head:hex40} [10]
    │  ├─ users/
    │  │       ╰─ {username} [2]
    │  ╰─ {*namespace}
    │                ╰─ /
    │                   ╰─ {repository} [5]
    │                                 ╰─ /
    │                                    ╰─ {*file} [6]
    ╰─ {*catch_all} [11]
    "###);

    Ok(())
}
