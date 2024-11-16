![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)

![rust: 1.81+](https://img.shields.io/badge/rust-1.81+-orange.svg)
![`unsafe`: forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)
![`no_std`: compatible](https://img.shields.io/badge/no__std-compatible-success.svg)
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
    assert_eq!(search.route, "/users/{id}");
    assert_eq!(search.parameters[0], ("id", "123"));

    let request = RequestBuilder::new()
      .path("/users/123/files/my.document.pdf")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.route, "/users/{id}/files/{filename}.{extension}");
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
    assert_eq!(search.route, "/files/{*slug}/delete");
    assert_eq!(search.parameters[0], ("slug", "documents/reports/annual.pdf"));

    let request = RequestBuilder::new()
      .path("/any/other/path")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.route, "/{*catch_all}");
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
    assert_eq!(search.route, "/users(/{id})");
    assert_eq!(search.expanded, Some("/users"));

    let request = RequestBuilder::new()
      .path("/users/123")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 1);
    assert_eq!(search.route, "/users(/{id})");
    assert_eq!(search.expanded, Some("/users/{id}"));
    assert_eq!(search.parameters[0], ("id", "123"));

    let request = RequestBuilder::new()
      .path("/files/documents/folder/report.pdf")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.route, "/files/{*slug}/{file}(.{extension})");
    assert_eq!(search.expanded, Some("/files/{*slug}/{file}.{extension}"));
    assert_eq!(search.parameters[0], ("slug", "documents/folder"));
    assert_eq!(search.parameters[1], ("file", "report"));
    assert_eq!(search.parameters[2], ("extension", "pdf"));

    let request = RequestBuilder::new()
      .path("/files/documents/folder/readme")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.route, "/files/{*slug}/{file}(.{extension})");
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
    assert_eq!(search.route, "/v2");

    let request = RequestBuilder::new()
      .path("/v2/my-org/my-repo/blobs/sha256:1234567890")
      .build()?;
    let search = router.search(&request)?.unwrap();
    assert_eq!(*search.data, 2);
    assert_eq!(search.route, "/v2/{*name:namespace}/blobs/{type}:{digest}");
    assert_eq!(search.parameters[0], ("name", "my-org/my-repo"));
    assert_eq!(search.parameters[1], ("type", "sha256"));
    assert_eq!(search.parameters[2], ("digest", "1234567890"));

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

const ERROR_DISPLAY: &str = "
duplicate constraint name

The constraint name 'my_constraint' is already in use:
    - existing constraint type: 'rust_out::ConstraintA'
    - new constraint type: 'rust_out::ConstraintB'

help: each constraint must have a unique name

try:
    - Check if you have accidentally added the same constraint twice
    - Ensure different constraints have different names
";

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
    assert_eq!(error.to_string(), ERROR_DISPLAY.trim());

    Ok(())
}
```

### Router Display

Routers can print their routes as an tree diagram.

- `[*]` denotes nodes within the tree that can be matched against.

Currenty, this doesn't handle split multi-byte characters well.

#### Example

```rust
use std::error::Error;
use wayfind::{Router, RouteBuilder};

const ROUTER_DISPLAY: &str = "
/
├─ user [*]
│  ╰─ /
│     ├─ createWithList [*]
│     ├─ log
│     │  ├─ out [*]
│     │  ╰─ in [*]
│     ╰─ {username} [*]
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
╰─ {*catch_all} [*]
";

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/pet")
        .build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new()
        .route("/pet/findByStatus")
        .build()?;
    router.insert(&route, 2)?;

    let route = RouteBuilder::new()
        .route("/pet/findByTags")
        .build()?;
    router.insert(&route, 3)?;

    let route = RouteBuilder::new()
        .route("/pet/{petId}")
        .build()?;
    router.insert(&route, 4)?;

    let route = RouteBuilder::new()
        .route("/pet/{petId}/uploadImage")
        .build()?;
    router.insert(&route, 5)?;

    let route = RouteBuilder::new()
        .route("/store/inventory")
        .build()?;
    router.insert(&route, 6)?;

    let route = RouteBuilder::new()
        .route("/store/order")
        .build()?;
    router.insert(&route, 7)?;

    let route = RouteBuilder::new()
        .route("/store/order/{orderId}")
        .build()?;
    router.insert(&route, 8)?;

    let route = RouteBuilder::new()
        .route("/user")
        .build()?;
    router.insert(&route, 9)?;

    let route = RouteBuilder::new()
        .route("/user/createWithList")
        .build()?;
    router.insert(&route, 10)?;

    let route = RouteBuilder::new()
        .route("/user/login")
        .build()?;
    router.insert(&route, 11)?;

    let route = RouteBuilder::new()
        .route("/user/logout")
        .build()?;
    router.insert(&route, 12)?;

    let route = RouteBuilder::new()
        .route("/user/{username}")
        .build()?;
    router.insert(&route, 13)?;

    let route = RouteBuilder::new()
        .route("/{*catch_all}")
        .build()?;
    router.insert(&route, 14)?;

    assert_eq!(router.path.to_string(), ROUTER_DISPLAY.trim());
    Ok(())
}
```

## Performance

`wayfind` is fast, and appears to be competitive against other top performers in all benchmarks we currently run.

However, as is often the case, your mileage may vary (YMMV).
Benchmarks, especially micro-benchmarks, should be taken with a grain of salt.

### Benchmarks

All benchmarks ran on a M1 Pro laptop.

Check out our [codspeed results](https://codspeed.io/DuskSystems/wayfind/benchmarks) for a more accurate set of timings.

#### Context

For all benchmarks, we percent-decode the path before matching.
After matching, we convert any extracted parameters to strings.

Some routers perform these operations automatically, while others require them to be done manually.

We do this to try and match behaviour as best as possible. This is as close to an "apples-to-apples" comparison as we can get.

#### `matchit` inspired benches

In a router of 130 routes, benchmark matching 4 paths.

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 335.58 ns | 4           | 265 B      | 4             | 265 B        |
| matchit          | 377.06 ns | 4           | 416 B      | 4             | 448 B        |
| xitca-router     | 410.98 ns | 7           | 800 B      | 7             | 832 B        |
| path-tree        | 439.78 ns | 4           | 416 B      | 4             | 448 B        |
| ntex-router      | 1.7437 µs | 18          | 1.248 KB   | 18            | 1.28 KB      |
| route-recognizer | 2.0488 µs | 160         | 8.505 KB   | 160           | 8.537 KB     |
| routefinder      | 4.7027 µs | 67          | 5.024 KB   | 67            | 5.056 KB     |
| actix-router     | 17.785 µs | 214         | 13.93 KB   | 214           | 13.96 KB     |

#### `path-tree` inspired benches

In a router of 320 routes, benchmark matching 80 paths.

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 4.7872 µs | 59          | 2.567 KB   | 59            | 2.567 KB     |
| matchit          | 6.4767 µs | 140         | 17.81 KB   | 140           | 17.83 KB     |
| path-tree        | 7.2644 µs | 59          | 7.447 KB   | 59            | 7.47 KB      |
| xitca-router     | 7.5313 µs | 209         | 25.51 KB   | 209           | 25.53 KB     |
| ntex-router      | 31.085 µs | 201         | 19.54 KB   | 201           | 19.56 KB     |
| route-recognizer | 54.813 µs | 2872        | 191.7 KB   | 2872          | 204.8 KB     |
| routefinder      | 75.617 µs | 525         | 48.4 KB    | 525           | 48.43 KB     |
| actix-router     | 151.15 µs | 2201        | 128.8 KB   | 2201          | 128.8 KB     |

## License

`wayfind` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
- [ASP.NET Core](https://github.com/dotnet/AspNetCore): Constraints-based approach to routing
