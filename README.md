![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)

![rust: 1.63+](https://img.shields.io/badge/rust-1.63+-orange.svg)
![`unsafe`: forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)
![`wasm`: compatible](https://img.shields.io/badge/wasm-compatible-success.svg)

[![codspeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/DuskSystems/wayfind)
[![codecov](https://codecov.io/gh/DuskSystems/wayfind/graph/badge.svg?token=QMSW55438K)](https://codecov.io/gh/DuskSystems/wayfind)

# `wayfind`

A speedy, flexible router for Rust.

NOTE: `wayfind` is still a work in progress.

## Why another router?

`wayfind` attempts to bridge the gap between existing Rust router options:

- fast routers, lacking in flexibility
- flexible routers, lacking in speed

Real-world projects often need fancy routing capabilities, such as projects ported from frameworks like [Ruby on Rails](https://guides.rubyonrails.org/routing.html), or those adhering to specifications like the [Open Container Initiative (OCI) Distribution Specification](https://github.com/opencontainers/distribution-spec/blob/main/spec.md).

The goal of `wayfind` is to remain competitive with the fastest libraries, while offering advanced routing features when needed. Unused features shouldn't impact performance - you only pay for what you use.

The downside of this approach is that we can't have as rich conflict detection as other routers.

## Features

### Dynamic Routing

Dynamic parameters can match any byte, **excluding** the path delimiter `/`.

We support both:
- whole segment parameters: `/{name}/`
- inline parameters: `/{year}-{month}-{day}/`

Dynamic parameters are greedy in nature, similar to a regex `.*`, and will attempt to match as many bytes as possible.

#### Example

```rust
use std::error::Error;
use wayfind::Router;

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users/{id}", 1)?;
    router.insert("/users/{id}/files/{filename}.{extension}", 2)?;

    let search = router.search("/users/123").unwrap();
    assert_eq!(*search.data, 1);
    assert_eq!(search.template, "/users/{id}");
    assert_eq!(search.parameters[0], ("id", "123"));

    let search = router.search("/users/123/files/my.document.pdf").unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.template, "/users/{id}/files/{filename}.{extension}");
    assert_eq!(search.parameters[0], ("id", "123"));
    assert_eq!(search.parameters[1], ("filename", "my.document"));
    assert_eq!(search.parameters[2], ("extension", "pdf"));

    Ok(())
}
```

### Wildcard Routing

Wildcard parameters can match any byte, **including** the path delimiter `/`.

We support both:
- inline wildcards: `/{*path}.html`
- mid-route wildcards: `/api/{*path}/help`
- end-route catch-all: `/{*catch_all}`

Like dynamic parameters, wildcard parameters are also greedy in nature.

#### Example

```rust
use std::error::Error;
use wayfind::Router;

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/files/{*slug}/delete", 1)?;
    router.insert("/{*catch_all}", 2)?;

    let search = router.search("/files/documents/reports/annual.pdf/delete").unwrap();
    assert_eq!(*search.data, 1);
    assert_eq!(search.template, "/files/{*slug}/delete");
    assert_eq!(search.parameters[0], ("slug", "documents/reports/annual.pdf"));

    let search = router.search("/any/other/path").unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.template, "/{*catch_all}");
    assert_eq!(search.parameters[0], ("catch_all", "any/other/path"));

    Ok(())
}
```

### Optional Groups

Optional groups allow for parts of a route to be absent.

They are commonly used for:
- optional IDs: `/users(/{id})`
- optional trailing slashes: `/users(/)`
- optional file extensions: `/images/{name}(.{extension})`

They work via 'expanding' the route into equivilant, simplified routes.

`/release/v{major}(.{minor}(.{patch}))`
- `/release/v{major}.{minor}.{patch}`
- `/release/v{major}.{minor}`
- `/release/v{major}`

There is a small overhead to using optional groups, due to `Arc` usage internally for data storage.

#### Example

```rust
use std::error::Error;
use wayfind::Router;

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users(/{id})", 1)?;
    router.insert("/files/{*slug}/{file}(.{extension})", 2)?;

    let search = router.search("/users").unwrap();
    assert_eq!(*search.data, 1);
    assert_eq!(search.template, "/users(/{id})");
    assert_eq!(search.expanded, Some("/users"));

    let search = router.search("/users/123").unwrap();
    assert_eq!(*search.data, 1);
    assert_eq!(search.template, "/users(/{id})");
    assert_eq!(search.expanded, Some("/users/{id}"));
    assert_eq!(search.parameters[0], ("id", "123"));

    let search = router.search("/files/documents/folder/report.pdf").unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.template, "/files/{*slug}/{file}(.{extension})");
    assert_eq!(search.expanded, Some("/files/{*slug}/{file}.{extension}"));
    assert_eq!(search.parameters[0], ("slug", "documents/folder"));
    assert_eq!(search.parameters[1], ("file", "report"));
    assert_eq!(search.parameters[2], ("extension", "pdf"));

    let search = router.search("/files/documents/folder/readme").unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.template, "/files/{*slug}/{file}(.{extension})");
    assert_eq!(search.expanded, Some("/files/{*slug}/{file}"));
    assert_eq!(search.parameters[0], ("slug", "documents/folder"));
    assert_eq!(search.parameters[1], ("file", "readme"));

    Ok(())
}
```

### Constraints

Constraints allow for custom logic to be injected into the routing process.

We support constraints for all types of parameters:
- Dynamic constraint: `/{name:constraint}`
- Wildcard constraint: `/{*name:constraint}`

The typical use-case for constraints would be to run a regex, or a simple `FromStr` implementation, against a path segment.

A common mistake would be to use these for validation of parameters. This should be avoided.

If a constraint fails to match, and no other suitable match exists, it results in a `Not Found` response, rather than any sort of `Bad Request`.

They act as an escape-hatch for when you need to disambiguate routes.

The current constraint implementation has a number of limitations:
- constraints cannot take parameters
- checks cannot make use of any prior state
- checks cannot store data after a successful check

#### Default Constraints

`wayfind` ships with a number of default constraints.

Curently, these can't be disabled.

| Name    | Method               |
|:--------|:---------------------|
| `u8`    | `u8::from_str`       |
| `u16`   | `u16::from_str`      |
| `u32`   | `u32::from_str`      |
| `u64`   | `u64::from_str`      |
| `u128`  | `u128::from_str`     |
| `usize` | `usize::from_str`    |
| `i8`    | `i8::from_str`       |
| `i16`   | `i16::from_str`      |
| `i32`   | `i32::from_str`      |
| `i64`   | `i64::from_str`      |
| `i128`  | `i128::from_str`     |
| `isize` | `isize::from_str`    |
| `f32`   | `f32::from_str`      |
| `f64`   | `f64::from_str`      |
| `bool`  | `bool::from_str`     |
| `ipv4`  | `Ipv4Addr::from_str` |
| `ipv6`  | `Ipv6Addr::from_str` |

#### Example

```rust
use std::error::Error;
use wayfind::{Router, Constraint};

struct NamespaceConstraint;
impl Constraint for NamespaceConstraint {
    const NAME: &'static str = "namespace";

    fn check(segment: &str) -> bool {
        segment
            .split('/')
            .all(|part| {
                !part.is_empty() && part.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-')
            })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.constraint::<NamespaceConstraint>()?;
    router.insert("/v2", 1)?;
    router.insert("/v2/{*name:namespace}/blobs/{type}:{digest}", 2)?;

    let search = router.search("/v2").unwrap();
    assert_eq!(*search.data, 1);
    assert_eq!(search.template, "/v2");

    let search = router.search("/v2/my-org/my-repo/blobs/sha256:1234567890").unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.template, "/v2/{*name:namespace}/blobs/{type}:{digest}");
    assert_eq!(search.parameters[0], ("name", "my-org/my-repo"));
    assert_eq!(search.parameters[1], ("type", "sha256"));
    assert_eq!(search.parameters[2], ("digest", "1234567890"));

    let search = router.search("/v2/invalid repo/blobs/uploads");
    assert!(search.is_none());

    Ok(())
}
```

### User-Friendly Error Messages

Where possible, we try to provide user-friendly error messages.

#### Example

```rust
use std::error::Error;
use wayfind::{Router, Constraint};

struct ConstraintA;
impl Constraint for ConstraintA {
    const NAME: &'static str = "my_constraint";

    fn check(segment: &str) -> bool {
        segment == "a"
    }
}

struct ConstraintB;
impl Constraint for ConstraintB {
    const NAME: &'static str = "my_constraint";

    fn check(segment: &str) -> bool {
        segment == "b"
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut router: Router<usize> = Router::new();
    router.constraint::<ConstraintA>()?;

    let error = router.constraint::<ConstraintB>().unwrap_err();
    insta::assert_snapshot!(error, @r"
    duplicate constraint name

    The constraint name 'my_constraint' is already in use:
        - existing constraint type: 'rust_out::ConstraintA'
        - new constraint type: 'rust_out::ConstraintB'

    help: each constraint must have a unique name

    try:
        - Check if you have accidentally added the same constraint twice
        - Ensure different constraints have different names
    ");

    Ok(())
}
```

### Router Display

Routers can print their routes.

`[*]` denotes nodes within the tree that can be matched against.

Currenty, this doesn't handle split multi-byte characters well.

#### Example

```rust
use std::error::Error;
use wayfind::Router;

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/pet", 1)?;
    router.insert("/pet/findByStatus", 2)?;
    router.insert("/pet/findByTags", 3)?;
    router.insert("/pet/{petId}", 4)?;
    router.insert("/pet/{petId}/uploadImage", 5)?;
    router.insert("/store/inventory", 6)?;
    router.insert("/store/order", 7)?;
    router.insert("/store/order/{orderId}", 8)?;
    router.insert("/user", 9)?;
    router.insert("/user/createWithList", 10)?;
    router.insert("/user/login", 11)?;
    router.insert("/user/logout", 12)?;
    router.insert("/user/{username}", 13)?;
    router.insert("/{*catch_all}", 14)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ pet [*]
    │  ╰─ /
    │     ├─ findBy
    │     │  ├─ Status [*]
    │     │  ╰─ Tags [*]
    │     ╰─ {petId} [*]
    │        ╰─ /uploadImage [*]
    ├─ store/
    │  ├─ inventory [*]
    │  ╰─ order [*]
    │     ╰─ /
    │        ╰─ {orderId} [*]
    ├─ user [*]
    │  ╰─ /
    │     ├─ createWithList [*]
    │     ├─ log
    │     │  ├─ in [*]
    │     │  ╰─ out [*]
    │     ╰─ {username} [*]
    ╰─ {*catch_all} [*]
    ");

    Ok(())
}
```

## Performance

`wayfind` is fast, and appears to be competitive against other top performers in all benchmarks we currently run.

However, as is often the case, your mileage may vary (YMMV).
Benchmarks, especially micro-benchmarks, should be taken with a grain of salt.

See [BENCHMARKING.md](BENCHMARKING.md) for the results.

## License

`wayfind` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
- [ASP.NET Core](https://github.com/dotnet/AspNetCore): Constraints-based approach to routing
