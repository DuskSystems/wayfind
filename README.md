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

All benchmarks ran on an MacOS M1 Pro laptop and a Linux Ryzen 5 3600 desktop.

Also check out our [codspeed results](https://codspeed.io/DuskSystems/wayfind).

### [`matchit 0.8.3` benches](https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs)

```
Compare Routers/wayfind
  M1 Pro: [187.68 ns 188.07 ns 188.46 ns]
  Ryzen 5 3600: [270.74 ns 272.34 ns 273.75 ns]

Compare Routers/matchit
  M1 Pro: [190.81 ns 191.18 ns 191.62 ns]
  Ryzen 5 3600: [206.70 ns 207.30 ns 207.91 ns]

Compare Routers/path-tree
  M1 Pro: [421.71 ns 423.61 ns 425.52 ns]
  Ryzen 5 3600: [519.64 ns 521.46 ns 523.43 ns]

Compare Routers/gonzales
  M1 Pro: [155.13 ns 155.48 ns 155.91 ns]
  Ryzen 5 3600: [248.21 ns 249.09 ns 250.18 ns]

Compare Routers/actix
  M1 Pro: [19.228 µs 19.284 µs 19.349 µs]
  Ryzen 5 3600: [23.029 µs 23.101 µs 23.185 µs]

Compare Routers/regex
  M1 Pro: [1.1476 µs 1.1509 µs 1.1544 µs]
  Ryzen 5 3600: [1.7391 µs 1.7435 µs 1.7479 µs]

Compare Routers/route-recognizer
  M1 Pro: [4.2903 µs 4.3001 µs 4.3108 µs]
  Ryzen 5 3600: [4.4614 µs 4.4778 µs 4.4960 µs]

Compare Routers/routefinder
  M1 Pro: [6.0529 µs 6.0683 µs 6.0884 µs]
  Ryzen 5 3600: [7.7193 µs 7.7311 µs 7.7449 µs]
```
