![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)

![rust: 1.88+](https://img.shields.io/badge/rust-1.88+-orange.svg)
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
use core::error::Error;

use wayfind::Router;

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    // Static
    router.insert("/", 1)?;
    router.insert("/health", 2)?;

    {
        let search = router.search("/").unwrap();
        assert_eq!(search.data, &1);
        assert_eq!(search.template, "/");

        let search = router.search("/health").unwrap();
        assert_eq!(search.data, &2);
        assert_eq!(search.template, "/health");

        let search = router.search("/heal");
        assert_eq!(search, None);
    }

    // Dynamic
    router.insert("/users/<id>", 3)?;
    router.insert("/users/<id>/message", 4)?;

    {
        let search = router.search("/users/123").unwrap();
        assert_eq!(search.data, &3);
        assert_eq!(search.template, "/users/<id>");
        assert_eq!(search.parameters[0], ("id", "123"));

        let search = router.search("/users/123/message").unwrap();
        assert_eq!(search.data, &4);
        assert_eq!(search.template, "/users/<id>/message");
        assert_eq!(search.parameters[0], ("id", "123"));

        let search = router.search("/users/");
        assert_eq!(search, None);
    }

    // Dynamic Inline
    router.insert("/images/<name>.png", 5)?;

    {
        let search = router.search("/images/avatar.final.png").unwrap();
        assert_eq!(search.data, &5);
        assert_eq!(search.template, "/images/<name>.png");
        assert_eq!(search.parameters[0], ("name", "avatar.final"));

        let search = router.search("/images/.png");
        assert_eq!(search, None);
    }

    // Wildcard
    router.insert("/files/<*path>", 6)?;
    router.insert("/files/<*path>/delete", 7)?;

    {
        let search = router.search("/files/documents").unwrap();
        assert_eq!(search.data, &6);
        assert_eq!(search.template, "/files/<*path>");
        assert_eq!(search.parameters[0], ("path", "documents"));

        let search = router.search("/files/documents/my-project/delete").unwrap();
        assert_eq!(search.data, &7);
        assert_eq!(search.template, "/files/<*path>/delete");
        assert_eq!(search.parameters[0], ("path", "documents/my-project"));

        let search = router.search("/files");
        assert_eq!(search, None);
    }

    // Wildcard Inline
    router.insert("/backups/<*path>.tar.gz", 8)?;

    {
        let search = router.search("/backups/production/database.tar.gz").unwrap();
        assert_eq!(search.data, &8);
        assert_eq!(search.template, "/backups/<*path>.tar.gz");
        assert_eq!(search.parameters[0], ("path", "production/database"));

        let search = router.search("/backups/.tar.gz");
        assert_eq!(search, None);
    }

    println!("{router}");
    Ok(())
}
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

`wayfind` stores routes in a compressed radix trie, like most performant routers.

The difference is in the search strategy. Most routers use either "first match wins" or "best match wins" (via backtracking).

We use a hybrid approach:

- per segment: first match wins
- within a segment: best match wins

This can result in some matches which may be unexpected, but in practice it works well for real-world usage.

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
