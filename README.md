[![Crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![Documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)
[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/DuskSystems/wayfind)

# `wayfind`

A speedy, flexible router for Rust.

- Zero dependencies.
- No unsafe code.

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

### [`matchit` benches](https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs)

In a router of 130 routes, benchmark matching 4 paths.

```
matchit benchmarks/gonzales
  time: [130.19 ns 130.46 ns 130.75 ns]

matchit benchmarks/matchit
  time: [182.51 ns 182.90 ns 183.31 ns]

matchit benchmarks/wayfind
  time: [264.19 ns 265.47 ns 266.77 ns]

matchit benchmarks/path-tree
  time: [334.91 ns 336.14 ns 337.42 ns]

matchit benchmarks/xitca-router
  time: [352.85 ns 353.63 ns 354.46 ns]

matchit benchmarks/path-table
  time: [538.15 ns 539.48 ns 540.91 ns]

matchit benchmarks/regex
  time: [1.1483 µs 1.1507 µs 1.1533 µs]

matchit benchmarks/ntex-router
  time: [1.5752 µs 1.5797 µs 1.5846 µs]

matchit benchmarks/route-recognizer
  time: [4.4940 µs 4.5062 µs 4.5164 µs]

matchit benchmarks/routefinder
  time: [6.1983 µs 6.2179 µs 6.2371 µs]

matchit benchmarks/actix-router
  time: [20.758 µs 20.864 µs 20.978 µs]
```

### [`path-tree` benches](https://github.com/viz-rs/path-tree/blob/v0.8.1/benches/bench.rs)

In a router of 320 routes, benchmark matching 80 paths.

```
path-tree benchmarks/wayfind
  time: [4.2534 µs 4.2664 µs 4.2791 µs]

path-tree benchmarks/matchit
  time: [4.8098 µs 4.8250 µs 4.8414 µs]

path-tree benchmarks/gonzales
  time: [5.1135 µs 5.1565 µs 5.1963 µs]

path-tree benchmarks/path-tree
  time: [5.6984 µs 5.7135 µs 5.7309 µs]

path-tree benchmarks/xitca-router
  time: [7.3627 µs 7.3818 µs 7.4020 µs]

path-tree benchmarks/path-table
  time: [10.455 µs 10.484 µs 10.514 µs]

path-tree benchmarks/ntex-router
  time: [27.316 µs 27.404 µs 27.499 µs]

path-tree benchmarks/regex
  time: [41.631 µs 41.831 µs 42.069 µs]

path-tree benchmarks/route-recognizer
  time: [85.349 µs 85.576 µs 85.832 µs]

path-tree benchmarks/routefinder
  time: [92.337 µs 92.717 µs 93.221 µs]

path-tree benchmarks/actix-router
  time: [175.34 µs 176.21 µs 177.18 µs]
```

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
- [ASP.NET Core](https://github.com/dotnet/AspNetCore): Constraints-based approach to routing
