[![Crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![Documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)
[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/DuskSystems/wayfind)
[![Codecov Badge](https://codecov.io/github/DuskSystems/wayfind/graph/badge.svg?token=QMSW55438K)](https://codecov.io/github/DuskSystems/wayfind)

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
    //    ├─ pet [1]
    //    │    ╰─ /
    //    │       ├─ findBy
    //    │       │       ├─ Status [2]
    //    │       │       ╰─ Tags [3]
    //    │       ╰─ {petId} [4]
    //    │                ╰─ /uploadImage [5]
    //    ├─ store/
    //    │       ├─ inventory [6]
    //    │       ╰─ order [7]
    //    │              ╰─ /
    //    │                 ╰─ {orderId} [8]
    //    ╰─ user [9]
    //          ╰─ /
    //             ├─ createWithList [10]
    //             ├─ log
    //             │    ├─ in [11]
    //             │    ╰─ out [12]
    //             ╰─ {username} [13]

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
    // ╰─ /v2 [1]
    //      ╰─ /
    //         ╰─ {*name:namespace}
    //                            ╰─ /
    //                               ├─ blobs/
    //                               │       ├─ uploads [4]
    //                               │       │        ╰─ /
    //                               │       │           ╰─ {reference} [5]
    //                               │       ╰─ {digest} [2]
    //                               ├─ manifests/
    //                               │           ╰─ {reference} [3]
    //                               ├─ referrers/
    //                               │           ╰─ {digest} [7]
    //                               ╰─ tags/list [6]

    Ok(())
}
```

## Benchmarks

All benchmarks ran on a MacOS M1 Pro laptop.

Check out our [codspeed results](https://codspeed.io/DuskSystems/wayfind/benchmarks) for a more accurate set of timings.

### [`matchit` benches](https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs)

In a router of 130 routes, benchmark matching 4 paths.

```
matchit benchmarks/wayfind
  time: [260.22 ns 261.07 ns 262.01 ns]

matchit benchmarks/actix-router
  time: [20.786 µs 20.837 µs 20.893 µs]

matchit benchmarks/gonzales
  time: [129.14 ns 129.48 ns 129.91 ns]

matchit benchmarks/matchit
  time: [182.83 ns 183.13 ns 183.49 ns]

matchit benchmarks/ntex-router
  time: [1.5853 µs 1.5912 µs 1.5989 µs]

matchit benchmarks/path-table
  time: [529.85 ns 530.70 ns 531.72 ns]

matchit benchmarks/path-tree
  time: [329.50 ns 330.75 ns 332.26 ns]

matchit benchmarks/regex
  time: [1.1410 µs 1.1435 µs 1.1463 µs]

matchit benchmarks/route-recognizer
  time: [4.3062 µs 4.3145 µs 4.3262 µs]

matchit benchmarks/routefinder
  time: [6.1499 µs 6.1597 µs 6.1705 µs]

matchit benchmarks/xitca-router
  time: [354.24 ns 359.52 ns 368.46 ns]
```

### [`path-tree` benches](https://github.com/viz-rs/path-tree/blob/v0.8.1/benches/bench.rs)

In a router of 320 routes, benchmark matching 80 paths.

```
path-tree benchmarks/wayfind
  time: [4.4622 µs 4.4692 µs 4.4767 µs]

path-tree benchmarks/actix-router
  time: [172.47 µs 172.87 µs 173.32 µs]

path-tree benchmarks/gonzales
  time: [5.8191 µs 5.8533 µs 5.8853 µs]

path-tree benchmarks/matchit
  time: [4.8825 µs 4.8900 µs 4.8984 µs]

path-tree benchmarks/ntex-router
  time: [26.971 µs 27.024 µs 27.094 µs]

path-tree benchmarks/path-table
  time: [10.264 µs 10.278 µs 10.295 µs]

path-tree benchmarks/path-tree
  time: [5.2534 µs 5.2783 µs 5.3049 µs]

path-tree benchmarks/regex
  time: [41.216 µs 41.356 µs 41.530 µs]

path-tree benchmarks/route-recognizer
  time: [86.208 µs 86.399 µs 86.667 µs]

path-tree benchmarks/routefinder
  time: [92.274 µs 92.413 µs 92.586 µs]

path-tree benchmarks/xitca-router
  time: [7.3530 µs 7.3689 µs 7.3873 µs]
```

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
- [ASP.NET Core](https://github.com/dotnet/AspNetCore): Constraints-based approach to routing
