[![Crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![Documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)
[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/DuskSystems/wayfind)

# `wayfind`

A speedy, flexible router for Rust.

Currently in a pre-alpha state.

## Why another router?

`wayfind` tries to bridge the gap between existing Rust router options:

- fast routers, lacking in flexibility
- flexible routers, lacking in speed

Real-world projects often need fancy routing capabilities, such as projects ported from frameworks like [Ruby on Rails](https://guides.rubyonrails.org/routing.html), or those adhering to specifications like the [OpenContainers Distribution Spec](https://github.com/opencontainers/distribution-spec/blob/main/spec.md).

The goal of `wayfind` is to remain competitive with the fastest libraries, while offering advanced routing features when needed. Unused features shouldn't impact performance - you only pay for what you use.

## Planned parameters types

- Static: `/index.html`
- Dynamic: `/users/{id}`
- Dynamic (inline): `/files/{file}.{extension}/delete`
- Wildcard: `/v1/{namespace:*}/tags/list`
- Wildcard (end): `/{catch_all:*}`
- Regex: `/ids/{id:[0-9]+}`

## Potential parameters types

- Optional: `/photos/{id:?}`
- Optional Groups: `/images/{name}(.{type})`

## Benchmarks

All benchmarks ran on an MacOS M1 Pro laptop.

Check out our [codspeed results](https://codspeed.io/DuskSystems/wayfind) for a more accurate set of results.

### [`matchit 0.8.3` benches](https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs)

```
wayfind
  time: [226.56 ns 226.73 ns 226.98 ns]

matchit
  time: [228.98 ns 229.30 ns 229.72 ns]

path-tree
  time: [480.98 ns 481.66 ns 482.58 ns]

gonzales
  time: [185.65 ns 185.69 ns 185.74 ns]

actix
  time: [22.807 µs 22.843 µs 22.887 µs]

regex
  time: [1.3549 µs 1.3577 µs 1.3607 µs]

route-recognizer
  time: [5.1537 µs 5.1601 µs 5.1671 µs]

routefinder
  time: [7.2353 µs 7.2430 µs 7.2531 µs]
```

### [`path-tree 0.8.1` benches](https://github.com/viz-rs/path-tree/blob/v0.8.1/benches/bench.rs)

```
wayfind
  time: [22.821 µs 23.010 µs 23.392 µs]

actix_router
  time: [4.7060 ms 4.7150 ms 4.7248 ms]

ntex_router
  time: [223.00 µs 224.13 µs 225.97 µs]

path_table
  time: [62.474 µs 62.565 µs 62.671 µs]

path_tree
  time: [50.806 µs 50.859 µs 50.921 µs]

matchit
  time: [29.469 µs 29.502 µs 29.538 µs]

route_recognizer
  time: [533.00 µs 533.58 µs 534.36 µs]
```
