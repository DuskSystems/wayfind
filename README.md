[![Crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![Documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)
[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/DuskSystems/wayfind)

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

    // $
    // ╰─ /
    //    ├─ pet [*]
    //    │    ╰─ /
    //    │       ├─ findBy
    //    │       │       ├─ Status [*]
    //    │       │       ╰─ Tags [*]
    //    │       ╰─ {petId} [*]
    //    │                ╰─ /uploadImage [*]
    //    ├─ store/
    //    │       ├─ inventory [*]
    //    │       ╰─ order [*]
    //    │              ╰─ /
    //    │                 ╰─ {orderId} [*]
    //    ╰─ user [*]
    //          ╰─ /
    //             ├─ createWithList [*]
    //             ├─ log
    //             │    ├─ in [*]
    //             │    ╰─ out [*]
    //             ╰─ {username} [*]

    Ok(())
}
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

    // $
    // ╰─ /v2 [*]
    //      ╰─ /
    //         ╰─ {*name:namespace}
    //                            ╰─ /
    //                               ├─ blobs/
    //                               │       ├─ uploads [*]
    //                               │       │        ╰─ /
    //                               │       │           ╰─ {reference} [*]
    //                               │       ╰─ {digest} [*]
    //                               ├─ manifests/
    //                               │           ╰─ {reference} [*]
    //                               ├─ referrers/
    //                               │           ╰─ {digest} [*]
    //                               ╰─ tags/list [*]

    Ok(())
}
```

## Benchmarks

All benchmarks ran on a MacOS M1 Pro laptop.

Check out our [codspeed results](https://codspeed.io/DuskSystems/wayfind/benchmarks) for a more accurate set of timings.

### `matchit` inspired benches

In a router of 130 routes, benchmark matching 4 paths.

| Library | Time |
|-----------|------|
| wayfind | 210.33 ns |
| matchit | 310.24 ns |
| path-tree | 406.26 ns |
| xitca-router | 415.02 ns |
| ntex-router | 1.6291 µs |
| route-recognizer | 4.3608 µs |
| routefinder | 6.2077 µs |
| actix-router | 20.722 µs |

### `path-tree` inspired benches

In a router of 320 routes, benchmark matching 80 paths.

| Library | Time |
|-----------|------|
| wayfind | 3.5117 µs |
| matchit | 6.8657 µs |
| path-tree | 7.5262 µs |
| xitca-router | 8.5490 µs |
| ntex-router | 28.003 µs |
| route-recognizer | 87.400 µs |
| routefinder | 95.115 µs |
| actix-router | 176.11 µs |

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
- [ASP.NET Core](https://github.com/dotnet/AspNetCore): Constraints-based approach to routing
