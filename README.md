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

## Parameters types

### Implemented

- Static: `/index.html`
- Dynamic: `/users/{id}`
- Dynamic (inline): `/files/{file}.{extension}/delete`
- Wildcard (end): `/{catch_all:*}`

### Planned

- Wildcard: `/v1/{namespace:*}/tags/list`
- Regex: `/ids/{id:[0-9]+}`

### Under consideration

Could be convinced to add these, but likely not in scope for `v1`.

- Optional: `/photos/{id:?}`
- Optional Groups: `/images/{name}(.{type})`

## Benchmarks

All benchmarks ran on a MacOS M1 Pro laptop.

Check out our [codspeed results](https://codspeed.io/DuskSystems/wayfind) for a more accurate set of timings.

### [`matchit 0.8.3` benches](https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs)

```
wayfind
  time: [197.91 ns 198.24 ns 198.64 ns]

matchit
  time: [188.43 ns 188.70 ns 189.03 ns]

path-tree
  time: [403.57 ns 404.67 ns 405.92 ns]

gonzales
  time: [155.42 ns 155.65 ns 155.92 ns]

actix
  time: [19.227 µs 19.286 µs 19.344 µs]

regex
  time: [1.1352 µs 1.1373 µs 1.1395 µs]

route-recognizer
  time: [4.2923 µs 4.3013 µs 4.3121 µs]

routefinder
  time: [6.0263 µs 6.0336 µs 6.0428 µs]
```

### [`path-tree 0.8.1` benches](https://github.com/viz-rs/path-tree/blob/v0.8.1/benches/bench.rs)

```
wayfind
  time: [18.670 µs 18.693 µs 18.724 µs]

actix_router
  time: [3.9160 ms 3.9373 ms 3.9709 ms]

ntex_router
  time: [185.86 µs 188.18 µs 192.94 µs]

path_table
  time: [51.964 µs 52.107 µs 52.263 µs]

path_tree
  time: [42.204 µs 42.424 µs 42.741 µs]

matchit
  time: [24.756 µs 24.817 µs 24.880 µs]

route_recognizer
  time: [443.35 µs 444.00 µs 444.79 µs]
```
g
