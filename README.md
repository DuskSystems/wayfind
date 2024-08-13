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

matchit allocations
├─ wayfind           alloc:
│                      678
│                      112.3 KB
│                    dealloc:
│                      678
│                      137.2 KB
│                    grow:
│                      80
│                      24.89 KB
│
├─ actix-router      alloc:
│                      31187
│                      38.03 MB
│                    dealloc:
│                      30985
│                      41.25 MB
│                    grow:
│                      6502
│                      3.228 MB
│                    shrink:
│                      183
│                      526 B
│
├─ gonzales          alloc:
│                      1014
│                      5.426 MB
│                    dealloc:
│                      1014
│                      5.434 MB
│                    grow:
│                      203
│                      8.272 KB
│
├─ matchit           alloc:
│                      1106
│                      67.99 KB
│                    dealloc:
│                      1106
│                      89.93 KB
│                    grow:
│                      89
│                      21.93 KB
│
├─ ntex-router       alloc:
│                      36933
│                      71.99 MB
│                    dealloc:
│                      36731
│                      73.92 MB
│                    grow:
│                      6729
│                      1.93 MB
│                    shrink:
│                      6
│                      23 B
│
├─ path-table        alloc:
│                      406
│                      39.73 KB
│                    dealloc:
│                      406
│                      39.73 KB
│
├─ path-tree         alloc:
│                      968
│                      70.79 KB
│                    dealloc:
│                      968
│                      93.03 KB
│                    grow:
│                      74
│                      22.24 KB
│
├─ regex             alloc:
│                      17782
│                      1.824 MB
│                    dealloc:
│                      17782
│                      3.619 MB
│                    grow:
│                      2747
│                      1.8 MB
│                    shrink:
│                      215
│                      5.938 KB
│
├─ route-recognizer  alloc:
│                      1437
│                      60.11 KB
│                    dealloc:
│                      1437
│                      222.3 KB
│                    grow:
│                      75
│                      162.2 KB
│
├─ routefinder       alloc:
│                      322
│                      37.74 KB
│                    dealloc:
│                      322
│                      58.74 KB
│                    grow:
│                      131
│                      20.99 KB
│
╰─ xitca-router      alloc:
                       1311
                       55.61 KB
                     dealloc:
                       1311
                       73.49 KB
                     grow:
                       89
                       17.88 KB
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

path-tree allocations
├─ wayfind           alloc:
│                      1698
│                      298.3 KB
│                    dealloc:
│                      1698
│                      353.1 KB
│                    grow:
│                      195
│                      54.81 KB
│
├─ actix-router      alloc:
│                      78390
│                      94.92 MB
│                    dealloc:
│                      77862
│                      104.7 MB
│                    grow:
│                      16411
│                      9.872 MB
│                    shrink:
│                      510
│                      1.856 KB
│
├─ gonzales          alloc:
│                      2638
│                      14.26 MB
│                    dealloc:
│                      2638
│                      14.28 MB
│                    grow:
│                      578
│                      19.53 KB
│
├─ matchit           alloc:
│                      2951
│                      204.3 KB
│                    dealloc:
│                      2951
│                      261.4 KB
│                    grow:
│                      218
│                      57.03 KB
│
├─ ntex-router       alloc:
│                      97922
│                      188.5 MB
│                    dealloc:
│                      97394
│                      193.5 MB
│                    grow:
│                      17688
│                      5.016 MB
│
├─ path-table        alloc:
│                      1192
│                      105.7 KB
│                    dealloc:
│                      1192
│                      105.7 KB
│
├─ path-tree         alloc:
│                      2471
│                      184.1 KB
│                    dealloc:
│                      2471
│                      233.5 KB
│                    grow:
│                      177
│                      49.44 KB
│
├─ regex             alloc:
│                      48316
│                      4.547 MB
│                    dealloc:
│                      48316
│                      8.485 MB
│                    grow:
│                      6982
│                      3.942 MB
│                    shrink:
│                      568
│                      4.694 KB
│
├─ route-recognizer  alloc:
│                      6152
│                      321.2 KB
│                    dealloc:
│                      6152
│                      981.4 KB
│                    grow:
│                      268
│                      660.1 KB
│
├─ routefinder       alloc:
│                      1131
│                      115.8 KB
│                    dealloc:
│                      1131
│                      179.3 KB
│                    grow:
│                      375
│                      63.48 KB
│
╰─ xitca-router      alloc:
                       3548
                       179.1 KB
                     dealloc:
                       3548
                       226.8 KB
                     grow:
                       218
                       47.65 KB
```

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
- [ASP.NET Core](https://github.com/dotnet/AspNetCore): Constraints-based approach to routing
