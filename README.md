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

### [`matchit 0.8.3` benches](https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs)

```
Compare Routers/wayfind
  time: [187.68 ns 188.07 ns 188.46 ns]

Compare Routers/matchit
  time: [190.81 ns 191.18 ns 191.62 ns]

Compare Routers/path-tree
  time: [421.71 ns 423.61 ns 425.52 ns]

Compare Routers/gonzales
  time: [155.13 ns 155.48 ns 155.91 ns]

Compare Routers/actix
  time: [19.228 µs 19.284 µs 19.349 µs]

Compare Routers/regex
  time: [1.1476 µs 1.1509 µs 1.1544 µs]

Compare Routers/route-recognizer
  time: [4.2903 µs 4.3001 µs 4.3108 µs]

Compare Routers/routefinder
  time: [6.0529 µs 6.0683 µs 6.0884 µs]
```
