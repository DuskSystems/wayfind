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
- Dynamic Segment: `/users/{id}`
- Dynamic Inline: `/calender/{year}-{month}-{day}`
- Wildcard End: `/{catch_all:*}`

### Planned

- Wildcard Segment(s): `/v1/{namespace:*}/tags/list`
- Regex Segment: `/ids/{id:[0-9]+}`
- Regex Inline: `/user-{user_id:[0-9]{8}-[0-9]{4}-[0-9]{4}-[0-9]{4}}`

### Under consideration

- Wildcard Inline: `/files/{file:*}.{extension}`

Currently, inline dynamic variables are greedy in nature, but maybe we'd be better off using inline wildcards to allow choosing between greedy and lazy matching.

### Unlikely

Could be convinced to add these, but likely not in scope for `v1`.

- Optional: `/photos/{id:?}`
- Optional Groups: `/images/{name}(.{type})`

## Potential additional features

- Case insensitive matches
- URL encoding/decoding
- OpenAPI compatibility
- Routing via query parameters, host, methods, ...

## Benchmarks

All benchmarks ran on a MacOS M1 Pro laptop.

Check out our [codspeed results](https://codspeed.io/DuskSystems/wayfind) for a more accurate set of timings.

### [`matchit` benches](https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs)

```
matchit benchmarks/wayfind
  time: [238.66 ns 238.92 ns 239.25 ns]

matchit benchmarks/actix-router
  time: [24.776 µs 24.826 µs 24.882 µs]

matchit benchmarks/gonzales
  time: [186.88 ns 187.11 ns 187.43 ns]

matchit benchmarks/matchit
  time: [226.44 ns 226.63 ns 226.90 ns]

matchit benchmarks/ntex-router
  time: [1.8766 µs 1.8797 µs 1.8834 µs]

matchit benchmarks/path-table
  time: [654.45 ns 655.11 ns 656.04 ns]

matchit benchmarks/path-tree
  time: [398.42 ns 399.51 ns 400.71 ns]

matchit benchmarks/regex
  time: [1.3865 µs 1.3895 µs 1.3926 µs]

matchit benchmarks/route-recognizer
  time: [5.1563 µs 5.1643 µs 5.1742 µs]

matchit benchmarks/routefinder
  time: [7.5365 µs 7.6053 µs 7.7237 µs]
```

### [`path-tree` benches](https://github.com/viz-rs/path-tree/blob/v0.8.1/benches/bench.rs)

```
path-tree benchmarks/wayfind
  time: [3.5272 µs 3.5315 µs 3.5368 µs]

path-tree benchmarks/actix-router
  time: [208.91 µs 209.32 µs 209.80 µs]

path-tree benchmarks/gonzales
  time: [7.0332 µs 7.0806 µs 7.1247 µs]

path-tree benchmarks/matchit
  time: [5.8580 µs 5.8653 µs 5.8729 µs]

path-tree benchmarks/ntex-router
  time: [32.272 µs 32.306 µs 32.347 µs]

path-tree benchmarks/path-table
  time: [12.286 µs 12.298 µs 12.313 µs]

path-tree benchmarks/path-tree
  time: [6.3182 µs 6.3274 µs 6.3399 µs]

path-tree benchmarks/regex
  time: [49.999 µs 50.411 µs 51.097 µs]

path-tree benchmarks/route-recognizer
  time: [104.09 µs 104.17 µs 104.27 µs]

path-tree benchmarks/routefinder
  time: [104.76 µs 106.39 µs 109.94 µs]
```

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
