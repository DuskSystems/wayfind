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

## Benchmarks

TODO
