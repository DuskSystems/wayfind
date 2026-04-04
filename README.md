![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)

![rust: 1.85+](https://img.shields.io/badge/rust-1.85+-orange.svg)
![`unsafe`: forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)
![`wasm`: compatible](https://img.shields.io/badge/wasm-compatible-success.svg)
![`no-std`: compatible](https://img.shields.io/badge/no--std-compatible-success.svg)

[![codecov](https://codecov.io/gh/DuskSystems/wayfind/graph/badge.svg)](https://codecov.io/gh/DuskSystems/wayfind)
[![codspeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/DuskSystems/wayfind)

# `wayfind`

A speedy, flexible router for Rust.

## Why another router?

Real-world projects often need advanced routing: inline parameters, mid-route wildcards, or compatibility with frameworks like [Ruby on Rails](https://guides.rubyonrails.org/routing.html) and specifications like the [OCI Distribution Specification](https://github.com/opencontainers/distribution-spec/blob/main/spec.md).

`wayfind` aims to be competitive with the fastest routers while supporting these features. Unused features don't impact performance.

## Showcase

```toml
[dependencies]
wayfind = "0.9"
```

```rust
use wayfind::RouterBuilder;

let mut builder = RouterBuilder::new();
builder.insert("/", 1)?;
builder.insert("/health", 2)?;
builder.insert("/users/<id>", 3)?;
builder.insert("/users/<id>/message", 4)?;
builder.insert("/images/<name>.png", 5)?;
builder.insert("/files/<*path>", 6)?;
builder.insert("/files/<*path>/delete", 7)?;
builder.insert("/backups/<*path>.tar.gz", 8)?;

let router = builder.build();

let search = router.search("/").ok_or("no match")?;
assert_eq!(search.data(), &1);
assert_eq!(search.template(), "/");

let search = router.search("/health").ok_or("no match")?;
assert_eq!(search.data(), &2);
assert_eq!(search.template(), "/health");

assert!(router.search("/heal").is_none());

let search = router.search("/users/123").ok_or("no match")?;
assert_eq!(search.data(), &3);
assert_eq!(search.template(), "/users/<id>");
assert_eq!(search.parameters(), &[("id", "123")]);

let search = router.search("/users/123/message").ok_or("no match")?;
assert_eq!(search.data(), &4);
assert_eq!(search.template(), "/users/<id>/message");
assert_eq!(search.parameters(), &[("id", "123")]);

assert!(router.search("/users/").is_none());

let search = router.search("/images/avatar.final.png").ok_or("no match")?;
assert_eq!(search.data(), &5);
assert_eq!(search.template(), "/images/<name>.png");
assert_eq!(search.parameters(), &[("name", "avatar.final")]);

assert!(router.search("/images/.png").is_none());

let search = router.search("/files/documents").ok_or("no match")?;
assert_eq!(search.data(), &6);
assert_eq!(search.template(), "/files/<*path>");
assert_eq!(search.parameters(), &[("path", "documents")]);

let search = router.search("/files/documents/my-project/delete").ok_or("no match")?;
assert_eq!(search.data(), &7);
assert_eq!(search.template(), "/files/<*path>/delete");
assert_eq!(search.parameters(), &[("path", "documents/my-project")]);

assert!(router.search("/files").is_none());

let search = router.search("/backups/production/database.tar.gz").ok_or("no match")?;
assert_eq!(search.data(), &8);
assert_eq!(search.template(), "/backups/<*path>.tar.gz");
assert_eq!(search.parameters(), &[("path", "production/database")]);

assert!(router.search("/backups/.tar.gz").is_none());

println!("{router}");
# Ok::<_, Box<dyn core::error::Error>>(())
```

```text
/
├─ backups/
│  ╰─ <*path>
│     ╰─ .tar.gz
├─ files/
│  ├─ <*path>
│  │  ╰─ /delete
│  ╰─ <*path>
├─ health
├─ images/
│  ╰─ <name>
│     ╰─ .png
╰─ users/
   ╰─ <id>
      ╰─ /message
```

## Implementation Details

`wayfind` stores routes in a compressed radix trie.

When searching, each node tries its children in priority order:
1. static
2. dynamic
3. wildcard

All parameters are greedy, consuming as much of the path as possible.

### Limitations

There is no backtracking across priority levels.
This can result in some matches which may be unexpected.

In the following router:

```text
/api/
├─ <version>
│  ╰─ /
│     ╰─ <*rest>
╰─ <*path>
   ╰─ /help
```

The path `/api/docs/help` would match the first route, not the second.
Even though the second is arguably more specific.

## Performance

`wayfind` is competitive with the fastest Rust routers across all benchmarks we run.

For all benchmarks, we convert any extracted parameters to strings.

All routers provide a way to return parameters as strings, but some delay the actual UTF-8 decoding until post-search.

| Library          | Percent Decoding | String Parameters |
|:-----------------|:----------------:|:-----------------:|
| wayfind          | no               | yes               |
| actix-router     | partial          | yes               |
| matchit          | no               | delayed           |
| ntex-router      | partial          | yes               |
| path-tree        | no               | delayed           |
| route-recognizer | no               | yes               |
| xitca-router     | no               | yes               |

As such, we provide 2 sets of results per benchmark:
- one with the default behaviour of the router.
- one with the parameters extracted to `Vec<(&str, &str)>`.

See the results at: https://codspeed.io/DuskSystems/wayfind/benchmarks

## License

`wayfind` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
