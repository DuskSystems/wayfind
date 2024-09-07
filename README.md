[![crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)
[![rust: 1.66+](https://img.shields.io/badge/rust-1.66+-orange.svg)](https://img.shields.io/badge/rust-1.66-orange.svg)
![unsafe: forbidden](https://img.shields.io/badge/unsafe-forbidden-brightgreen.svg)
![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

[![codspeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/DuskSystems/wayfind)
[![codecov](https://codecov.io/gh/DuskSystems/wayfind/graph/badge.svg?token=QMSW55438K)](https://codecov.io/gh/DuskSystems/wayfind)

# `wayfind`

A speedy, flexible router for Rust.

## Why another router?

`wayfind` attempts to bridge the gap between existing Rust router options:

- fast routers, lacking in flexibility
- flexible routers, lacking in speed

Real-world projects often need fancy routing capabilities, such as projects ported from frameworks like [Ruby on Rails](https://guides.rubyonrails.org/routing.html), or those adhering to specifications like the [Open Container Initiative (OCI) Distribution Specification](https://github.com/opencontainers/distribution-spec/blob/main/spec.md).

The goal of `wayfind` is to remain competitive with the fastest libraries, while offering advanced routing features when needed. Unused features shouldn't impact performance - you only pay for what you use.

## Features

### Dynamic Routing

Dynamic parameters allow matching for any byte, excluding the path delimiter `/`.

We support both:
- Whole segment parameters: `/{name}/`
- Inline parameters: `/{year}-{month}-{day}/`

Inline dynamic parameters are greedy in nature, similar to a regex `.*`, and will attempt to match as many bytes as possible.

#### Example

```rust
use std::error::Error;
use wayfind::{Path, Router};

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users/{id}", 1)?;
    router.insert("/users/{id}/files/{filename}.{extension}", 2)?;

    let path = Path::new("/users/123")?;
    let search = router.search(&path)?.unwrap();
    assert_eq!(search.data.value, 1);
    assert_eq!(search.data.path, "/users/{id}".into());
    assert_eq!(search.parameters[0].key, "id");
    assert_eq!(search.parameters[0].value, "123");

    let path = Path::new("/users/123/files/my.document.pdf")?;
    let search = router.search(&path)?.unwrap();
    assert_eq!(search.data.value, 2);
    assert_eq!(search.data.path, "/users/{id}/files/{filename}.{extension}".into());
    assert_eq!(search.parameters[0].key, "id");
    assert_eq!(search.parameters[0].value, "123");
    assert_eq!(search.parameters[1].key, "filename");
    assert_eq!(search.parameters[1].value, "my.document");
    assert_eq!(search.parameters[2].key, "extension");
    assert_eq!(search.parameters[2].value, "pdf");

    Ok(())
}
```

### Wildcard Routing

Wildcard parameters enable matching of one or more segments within a path.

We support both:
- mid-route wildcards: `/api/{*path}/help`
- end-route catch-all: `/{*catch_all}`

#### Example

```rust
use std::error::Error;
use wayfind::{Path, Router};

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/files/{*slug}/delete", 1)?;
    router.insert("/{*catch_all}", 2)?;

    let path = Path::new("/files/documents/reports/annual.pdf/delete")?;
    let search = router.search(&path)?.unwrap();
    assert_eq!(search.data.value, 1);
    assert_eq!(search.data.path, "/files/{*slug}/delete".into());
    assert_eq!(search.parameters[0].key, "slug");
    assert_eq!(search.parameters[0].value, "documents/reports/annual.pdf");

    let path = Path::new("/any/other/path")?;
    let search = router.search(&path)?.unwrap();
    assert_eq!(search.data.value, 2);
    assert_eq!(search.data.path, "/{*catch_all}".into());
    assert_eq!(search.parameters[0].key, "catch_all");
    assert_eq!(search.parameters[0].value, "any/other/path");

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
use wayfind::{Constraint, Path, Router};

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

    let path = Path::new("/v2")?;
    let search = router.search(&path)?.unwrap();
    assert_eq!(search.data.value, 1);
    assert_eq!(search.data.path, "/v2".into());

    let path = Path::new("/v2/my-org/my-repo/blobs/sha256:1234567890")?;
    let search = router.search(&path)?.unwrap();
    assert_eq!(search.data.value, 2);
    assert_eq!(search.data.path, "/v2/{*name:namespace}/blobs/{type}:{digest}".into());
    assert_eq!(search.parameters[0].key, "name");
    assert_eq!(search.parameters[0].value, "my-org/my-repo");
    assert_eq!(search.parameters[1].key, "type");
    assert_eq!(search.parameters[1].value, "sha256");
    assert_eq!(search.parameters[2].key, "digest");
    assert_eq!(search.parameters[2].value, "1234567890");

    let path = Path::new("/v2/invalid repo/blobs/uploads")?;
    assert!(router.search(&path)?.is_none());

    Ok(())
}
```

### User-Friendly Error Messages

Where possible, we try to provide user-friendly error messages.

#### Example

```rust
use std::error::Error;
use wayfind::{Constraint, Router, errors::ConstraintError};

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
    assert_eq!(error.to_string(), ERROR_DISPLAY.trim());

    Ok(())
}
```

### Router Display

Routers can print their routes as an tree diagram.

- `▽` represents the root node.
- `○` represents nodes within the tree that can be matched against.

Currenty, this doesn't handle split multi-byte characters well.

#### Example

```rust
use std::error::Error;
use wayfind::Router;

const ROUTER_DISPLAY: &str = "
▽
├─ /
│  ├─ pet ○
│  │    ╰─ /
│  │       ├─ findBy
│  │       │       ├─ Status ○
│  │       │       ╰─ Tags ○
│  │       ╰─ {petId} ○
│  │                ╰─ /uploadImage ○
│  ├─ store/
│  │       ├─ inventory ○
│  │       ╰─ order ○
│  │              ╰─ /
│  │                 ╰─ {orderId} ○
│  ╰─ user ○
│        ╰─ /
│           ├─ createWithList ○
│           ├─ log
│           │    ├─ in ○
│           │    ╰─ out ○
│           ╰─ {username} ○
╰─ {*catch_all} ○
";

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

    router.insert("{*catch_all}", 14)?;

    assert_eq!(router.to_string(), ROUTER_DISPLAY.trim());
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
| matchit          | 462.33 ns | 4           | 416 B      | 4             | 448 B        |
| wayfind          | 483.07 ns | 7           | 649 B      | 7             | 649 B        |
| xitca-router     | 562.71 ns | 7           | 800 B      | 7             | 832 B        |
| path-tree        | 572.69 ns | 4           | 416 B      | 4             | 448 B        |
| ntex-router      | 1.7347 µs | 18          | 1.248 KB   | 18            | 1.28 KB      |
| route-recognizer | 4.6183 µs | 160         | 8.515 KB   | 160           | 8.547 KB     |
| routefinder      | 6.5185 µs | 67          | 5.024 KB   | 67            | 5.056 KB     |
| actix-router     | 21.268 µs | 214         | 13.93 KB   | 214           | 13.96 KB     |

#### `path-tree` inspired benches

In a router of 320 routes, benchmark matching 80 paths.

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 7.0411 µs | 117         | 9.991 KB   | 117           | 9.991 KB     |
| matchit          | 8.8426 µs | 140         | 17.81 KB   | 140           | 17.83 KB     |
| path-tree        | 9.2876 µs | 59          | 7.447 KB   | 59            | 7.47 KB      |
| xitca-router     | 10.888 µs | 209         | 25.51 KB   | 209           | 25.53 KB     |
| ntex-router      | 30.283 µs | 201         | 19.54 KB   | 201           | 19.56 KB     |
| routefinder      | 99.873 µs | 525         | 48.4 KB    | 525           | 48.43 KB     |
| route-recognizer | 107.16 µs | 2872        | 191.8 KB   | 2872          | 205 KB       |
| actix-router     | 192.44 µs | 2201        | 128.8 KB   | 2201          | 128.8 KB     |

## License

`wayfind` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
- [ASP.NET Core](https://github.com/dotnet/AspNetCore): Constraints-based approach to routing
