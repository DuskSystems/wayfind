![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)

![rust: 1.83+](https://img.shields.io/badge/rust-1.83+-orange.svg)
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
use wayfind::{Router, RouteBuilder, RequestBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users/{id}")
        .build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new()
        .route("/users/{id}/files/{filename}.{extension}")
        .build()?;
    router.insert(&route, 2)?;

    let request = RequestBuilder::new()
      .path("/users/123")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 1);
    assert_eq!(search.path.route, "/users/{id}".into());
    assert_eq!(search.path.parameters[0], ("id", "123"));

    let request = RequestBuilder::new()
      .path("/users/123/files/my.document.pdf")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.path.route, "/users/{id}/files/{filename}.{extension}".into());
    assert_eq!(search.path.parameters[0], ("id", "123"));
    assert_eq!(search.path.parameters[1], ("filename", "my.document"));
    assert_eq!(search.path.parameters[2], ("extension", "pdf"));

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
use wayfind::{Router, RouteBuilder, RequestBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/files/{*slug}/delete")
        .build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new()
        .route("/{*catch_all}")
        .build()?;
    router.insert(&route, 2)?;

    let request = RequestBuilder::new()
      .path("/files/documents/reports/annual.pdf/delete")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 1);
    assert_eq!(search.path.route, "/files/{*slug}/delete".into());
    assert_eq!(search.path.parameters[0], ("slug", "documents/reports/annual.pdf"));

    let request = RequestBuilder::new()
      .path("/any/other/path")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.path.route, "/{*catch_all}".into());
    assert_eq!(search.path.parameters[0], ("catch_all", "any/other/path"));

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
use wayfind::{Router, RouteBuilder, RequestBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users(/{id})")
        .build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new()
        .route("/files/{*slug}/{file}(.{extension})")
        .build()?;
    router.insert(&route, 2)?;

    let request = RequestBuilder::new()
      .path("/users")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 1);
    assert_eq!(search.path.route, "/users(/{id})".into());
    assert_eq!(search.path.expanded, Some("/users".into()));

    let request = RequestBuilder::new()
      .path("/users/123")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 1);
    assert_eq!(search.path.route, "/users(/{id})".into());
    assert_eq!(search.path.expanded, Some("/users/{id}".into()));
    assert_eq!(search.path.parameters[0], ("id", "123"));

    let request = RequestBuilder::new()
      .path("/files/documents/folder/report.pdf")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.path.route, "/files/{*slug}/{file}(.{extension})".into());
    assert_eq!(search.path.expanded, Some("/files/{*slug}/{file}.{extension}".into()));
    assert_eq!(search.path.parameters[0], ("slug", "documents/folder"));
    assert_eq!(search.path.parameters[1], ("file", "report"));
    assert_eq!(search.path.parameters[2], ("extension", "pdf"));

    let request = RequestBuilder::new()
      .path("/files/documents/folder/readme")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.path.route, "/files/{*slug}/{file}(.{extension})".into());
    assert_eq!(search.path.expanded, Some("/files/{*slug}/{file}".into()));
    assert_eq!(search.path.parameters[0], ("slug", "documents/folder"));
    assert_eq!(search.path.parameters[1], ("file", "readme"));

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
use wayfind::{PathConstraint, Router, RouteBuilder, RequestBuilder};

struct NamespaceConstraint;
impl PathConstraint for NamespaceConstraint {
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
    router.path.constraint::<NamespaceConstraint>()?;

    let route = RouteBuilder::new()
        .route("/v2")
        .build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new()
        .route("/v2/{*name:namespace}/blobs/{type}:{digest}")
        .build()?;
    router.insert(&route, 2)?;

    let request = RequestBuilder::new()
      .path("/v2")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 1);
    assert_eq!(search.path.route, "/v2".into());

    let request = RequestBuilder::new()
      .path("/v2/my-org/my-repo/blobs/sha256:1234567890")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.path.route, "/v2/{*name:namespace}/blobs/{type}:{digest}".into());
    assert_eq!(search.path.parameters[0], ("name", "my-org/my-repo"));
    assert_eq!(search.path.parameters[1], ("type", "sha256"));
    assert_eq!(search.path.parameters[2], ("digest", "1234567890"));

    let request = RequestBuilder::new()
      .path("/v2/invalid repo/blobs/uploads")
      .build()?;
    assert!(router.search(&request)?.is_none());

    Ok(())
}
```

### User-Friendly Error Messages

Where possible, we try to provide user-friendly error messages.

#### Example

```rust
use std::error::Error;
use wayfind::{PathConstraint, Router};

struct ConstraintA;
impl PathConstraint for ConstraintA {
    const NAME: &'static str = "my_constraint";

    fn check(segment: &str) -> bool {
        segment == "a"
    }
}

struct ConstraintB;
impl PathConstraint for ConstraintB {
    const NAME: &'static str = "my_constraint";

    fn check(segment: &str) -> bool {
        segment == "b"
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut router: Router<usize> = Router::new();
    router.path.constraint::<ConstraintA>()?;

    let error = router.path.constraint::<ConstraintB>().unwrap_err();
    insta::assert_snapshot!(error, @r"
    duplicate path constraint name

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

Currenty, this doesn't handle split multi-byte characters well.

#### Example

```rust
use std::error::Error;
use wayfind::{Router, RouteBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/pet")
        .methods(vec!["POST"])
        .build()?;
    router.insert(&route, "handle_add_pet")?;

    let route = RouteBuilder::new()
        .route("/pet")
        .methods(vec!["PUT"])
        .build()?;
    router.insert(&route, "handle_update_pet")?;

    let route = RouteBuilder::new()
        .route("/pet/findByStatus")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, "handle_find_pets_by_status")?;

    let route = RouteBuilder::new()
        .route("/pet/findByTags")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, "handle_find_pets_by_tags")?;

    let route = RouteBuilder::new()
        .route("/pet/{petId}")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, "handle_get_pet")?;

    let route = RouteBuilder::new()
        .route("/pet/{petId}")
        .methods(vec!["POST"])
        .build()?;
    router.insert(&route, "handle_update_pet_form")?;

    let route = RouteBuilder::new()
        .route("/pet/{petId}")
        .methods(vec!["DELETE"])
        .build()?;
    router.insert(&route, "handle_delete_pet")?;

    let route = RouteBuilder::new()
        .route("/pet/{petId}/uploadImage")
        .methods(vec!["POST"])
        .build()?;
    router.insert(&route, "handle_upload_pet_image")?;

    let route = RouteBuilder::new()
        .route("/store/inventory")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, "handle_get_inventory")?;

    let route = RouteBuilder::new()
        .route("/store/order")
        .methods(vec!["POST"])
        .build()?;
    router.insert(&route, "handle_place_order")?;

    let route = RouteBuilder::new()
        .route("/store/order/{orderId}")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, "handle_get_order")?;

    let route = RouteBuilder::new()
        .route("/store/order/{orderId}")
        .methods(vec!["DELETE"])
        .build()?;
    router.insert(&route, "handle_delete_order")?;

    let route = RouteBuilder::new()
        .route("/user")
        .methods(vec!["POST"])
        .build()?;
    router.insert(&route, "handle_create_user")?;

    let route = RouteBuilder::new()
        .route("/user/createWithList")
        .methods(vec!["POST"])
        .build()?;
    router.insert(&route, "handle_create_users_list")?;

    let route = RouteBuilder::new()
        .route("/user/login")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, "handle_login")?;

    let route = RouteBuilder::new()
        .route("/user/logout")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, "handle_logout")?;

    let route = RouteBuilder::new()
        .route("/user/{username}")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, "handle_get_user")?;

    let route = RouteBuilder::new()
        .route("/user/{username}")
        .methods(vec!["PUT"])
        .build()?;
    router.insert(&route, "handle_update_user")?;

    let route = RouteBuilder::new()
        .route("/user/{username}")
        .methods(vec!["DELETE"])
        .build()?;
    router.insert(&route, "handle_delete_user")?;

    let route = RouteBuilder::new()
        .route("/{*catch_all}")
        .build()?;
    router.insert(&route, "handle_not_found")?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /
    ├─ user [*:9]
    │  ╰─ /
    │     ├─ createWithList [*:10]
    │     ├─ log
    │     │  ├─ out [*:12]
    │     │  ╰─ in [*:11]
    │     ╰─ {username} [*:13]
    ├─ pet [*:1]
    │  ╰─ /
    │     ├─ findBy
    │     │  ├─ Status [*:2]
    │     │  ╰─ Tags [*:3]
    │     ╰─ {petId} [*:4]
    │        ╰─ /uploadImage [*:5]
    ├─ store/
    │  ├─ inventory [*:6]
    │  ╰─ order [*:7]
    │     ╰─ /
    │        ╰─ {orderId} [*:8]
    ╰─ {*catch_all} [*:14]
    === Method
    [1]
    ├─ POST [1]
    ╰─ PUT [2]

    [2]
    ╰─ GET [3]

    [3]
    ╰─ GET [4]

    [4]
    ├─ DELETE [7]
    ├─ GET [5]
    ╰─ POST [6]

    [5]
    ╰─ POST [8]

    [6]
    ╰─ GET [9]

    [7]
    ╰─ POST [10]

    [8]
    ├─ DELETE [12]
    ╰─ GET [11]

    [9]
    ╰─ POST [13]

    [10]
    ╰─ POST [14]

    [11]
    ╰─ GET [15]

    [12]
    ╰─ GET [16]

    [13]
    ├─ DELETE [19]
    ├─ GET [17]
    ╰─ PUT [18]
    === Chains
    *-1-1
    *-1-2
    *-2-3
    *-3-4
    *-4-5
    *-4-6
    *-4-7
    *-5-8
    *-6-9
    *-7-10
    *-8-11
    *-8-12
    *-9-13
    *-10-14
    *-11-15
    *-12-16
    *-13-17
    *-13-18
    *-13-19
    *-14-*
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
