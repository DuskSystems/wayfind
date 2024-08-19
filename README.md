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

| Library          | Time      |
|:-----------------|----------:|
| wayfind          | 299.14 ns |
| matchit          | 465.70 ns |
| xitca-router     | 562.67 ns |
| path-tree        | 583.24 ns |
| ntex-router      | 1.7803 µs |
| route-recognizer | 4.5356 µs |
| routefinder      | 6.4985 µs |
| actix-router     | 20.811 µs |

### `path-tree` inspired benches

In a router of 320 routes, benchmark matching 80 paths.

| Library          | Time      |
|:-----------------|----------:|
| wayfind          | 3.8742 µs |
| matchit          | 8.8435 µs |
| path-tree        | 9.5325 µs |
| xitca-router     | 10.753 µs |
| ntex-router      | 30.610 µs |
| route-recognizer | 89.968 µs |
| routefinder      | 97.513 µs |
| actix-router     | 177.33 µs |

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
- [ASP.NET Core](https://github.com/dotnet/AspNetCore): Constraints-based approach to routing
