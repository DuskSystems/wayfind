[![crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)
[![rust 1.66](https://img.shields.io/badge/rust-1.66-orange.svg)](https://img.shields.io/badge/rust-1.66-orange.svg)
[![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![codspeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/DuskSystems/wayfind)
[![codecov](https://codecov.io/gh/DuskSystems/wayfind/graph/badge.svg?token=QMSW55438K)](https://codecov.io/gh/DuskSystems/wayfind)

# `wayfind`

A speedy, flexible router for Rust.

> [!WARNING]
> Currently in a pre-alpha state.

## Why another router?

`wayfind` tries to bridge the gap between existing Rust router options:

- fast routers, lacking in flexibility
- flexible routers, lacking in speed

Real-world projects often need fancy routing capabilities, such as projects ported from frameworks like [Ruby on Rails](https://guides.rubyonrails.org/routing.html), or those adhering to specifications like the [Open Container Initiative (OCI) Distribution Specification](https://github.com/opencontainers/distribution-spec/blob/main/spec.md).

The goal of `wayfind` is to remain competitive with the fastest libraries, while offering advanced routing features when needed. Unused features shouldn't impact performance - you only pay for what you use.

## Examples

### [Swagger Petstore](https://petstore.swagger.io)

Simple routing, with only static and dynamic sections.

```rust
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

    Ok(())
}
```

```
$
╰─ /
   ├─ pet [*]
   │    ╰─ /
   │       ├─ findBy
   │       │       ├─ Status [*]
   │       │       ╰─ Tags [*]
   │       ╰─ {petId} [*]
   │                ╰─ /uploadImage [*]
   ├─ store/
   │       ├─ inventory [*]
   │       ╰─ order [*]
   │              ╰─ /
   │                 ╰─ {orderId} [*]
   ╰─ user [*]
         ╰─ /
            ├─ createWithList [*]
            ├─ log
            │    ├─ in [*]
            │    ╰─ out [*]
            ╰─ {username} [*]
```

### [OCI Distribution Specification](https://github.com/opencontainers/distribution-spec)

Complex routing, with wildcard sections and custom constraints.

```rust
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
    router.insert("/v2/{*name:namespace}/blobs/{digest}", 2)?;
    router.insert("/v2/{*name:namespace}/manifests/{reference}", 3)?;
    router.insert("/v2/{*name:namespace}/blobs/uploads", 4)?;
    router.insert("/v2/{*name:namespace}/blobs/uploads/{reference}", 5)?;
    router.insert("/v2/{*name:namespace}/tags/list", 6)?;
    router.insert("/v2/{*name:namespace}/referrers/{digest}", 7)?;

    Ok(())
}
```

```
$
╰─ /v2 [*]
     ╰─ /
        ╰─ {*name:namespace}
                           ╰─ /
                              ├─ blobs/
                              │       ├─ uploads [*]
                              │       │        ╰─ /
                              │       │           ╰─ {reference} [*]
                              │       ╰─ {digest} [*]
                              ├─ manifests/
                              │           ╰─ {reference} [*]
                              ├─ referrers/
                              │           ╰─ {digest} [*]
                              ╰─ tags/list [*]
```

## Benchmarks

All benchmarks ran on a MacOS M1 Pro laptop.

Check out our [codspeed results](https://codspeed.io/DuskSystems/wayfind/benchmarks) for a more accurate set of timings.

> [!NOTE]
> For all benchmarks, we percent-decode the path before matching.
> After matching, we convert any extracted parameters to strings.
> Some routers perform these operations automatically, while others require them to be done manually.
> We do this to try and match behaviour as best as possible.

### `matchit` inspired benches

In a router of 130 routes, benchmark matching 4 paths.

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 437.32 ns | 4           | 265 B      | 4             | 265 B        |
| matchit          | 538.64 ns | 4           | 416 B      | 4             | 448 B        |
| xitca-router     | 697.62 ns | 7           | 800 B      | 7             | 832 B        |
| path-tree        | 684.44 ns | 4           | 416 B      | 4             | 448 B        |
| ntex-router      | 2.1531 µs | 18          | 1.248 KB   | 18            | 1.28 KB      |
| route-recognizer | 5.4506 µs | 160         | 8.515 KB   | 160           | 8.547 KB     |
| routefinder      | 7.6915 µs | 67          | 5.024 KB   | 67            | 5.056 KB     |
| actix-router     | 24.998 µs | 214         | 13.93 KB   | 214           | 13.96 KB     |

### `path-tree` inspired benches

In a router of 320 routes, benchmark matching 80 paths.

| Library          | Time      | Alloc Count | Alloc Size | Dealloc Count | Dealloc Size |
|:-----------------|----------:|------------:|-----------:|--------------:|-------------:|
| wayfind          | 6.1418 µs | 59          | 2.567 KB   | 59            | 2.567 KB     |
| matchit          | 10.908 µs | 140         | 17.81 KB   | 140           | 17.83 KB     |
| path-tree        | 11.460 µs | 59          | 7.447 KB   | 59            | 7.47 KB      |
| xitca-router     | 13.067 µs | 209         | 25.51 KB   | 209           | 25.53 KB     |
| ntex-router      | 37.354 µs | 201         | 19.54 KB   | 201           | 19.56 KB     |
| route-recognizer | 110.38 µs | 2872        | 191.8 KB   | 2872          | 205 KB       |
| routefinder      | 117.94 µs | 525         | 48.4 KB    | 525           | 48.43 KB     |
| actix-router     | 213.03 µs | 2201        | 128.8 KB   | 2201          | 128.8 KB     |

## Minimum Supported Rust Version (MSRV)

The MSRV is 1.66.

## License

`wayfind` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
- [ASP.NET Core](https://github.com/dotnet/AspNetCore): Constraints-based approach to routing
