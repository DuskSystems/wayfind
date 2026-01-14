![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)

![rust: 1.85+](https://img.shields.io/badge/rust-1.85+-orange.svg)
![`unsafe`: forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)
![`wasm`: compatible](https://img.shields.io/badge/wasm-compatible-success.svg)
![`no-std`: compatible](https://img.shields.io/badge/no--std-compatible-success.svg)

# `wayfind`

A speedy, flexible router for Rust.

## Why another router?

`wayfind` attempts to bridge the gap between existing Rust router options:

- fast routers, lacking in flexibility
- flexible routers, lacking in speed

Real-world projects often need fancy routing capabilities, such as projects ported from frameworks like [Ruby on Rails](https://guides.rubyonrails.org/routing.html), or those adhering to specifications like the [Open Container Initiative (OCI) Distribution Specification](https://github.com/opencontainers/distribution-spec/blob/main/spec.md).

The goal of `wayfind` is to remain competitive with the fastest libraries, while offering advanced routing features when needed. Unused features shouldn't impact performance - you only pay for what you use.

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
    router.insert("/images/<name>.<ext>", 6)?;

    {
        let search = router.search("/images/avatar.final.png").unwrap();
        assert_eq!(search.data, &5);
        assert_eq!(search.template, "/images/<name>.png");
        assert_eq!(search.parameters[0], ("name", "avatar.final"));

        let search = router.search("/images/photo.jpg").unwrap();
        assert_eq!(search.data, &6);
        assert_eq!(search.template, "/images/<name>.<ext>");
        assert_eq!(search.parameters[0], ("name", "photo"));
        assert_eq!(search.parameters[1], ("ext", "jpg"));

        let search = router.search("/images/.png");
        assert_eq!(search, None);
    }

    // Wildcard
    router.insert("/files/<*path>", 7)?;
    router.insert("/files/<*path>/delete", 8)?;

    {
        let search = router.search("/files/documents").unwrap();
        assert_eq!(search.data, &7);
        assert_eq!(search.template, "/files/<*path>");
        assert_eq!(search.parameters[0], ("path", "documents"));

        let search = router.search("/files/documents/my-project/delete").unwrap();
        assert_eq!(search.data, &8);
        assert_eq!(search.template, "/files/<*path>/delete");
        assert_eq!(search.parameters[0], ("path", "documents/my-project"));

        let search = router.search("/files");
        assert_eq!(search, None);
    }

    // Wildcard Inline
    router.insert("/backups/<*path>.tar.gz", 9)?;
    router.insert("/backups/<*path>.<ext>", 10)?;

    {
        let search = router.search("/backups/production/database.tar.gz").unwrap();
        assert_eq!(search.data, &9);
        assert_eq!(search.template, "/backups/<*path>.tar.gz");
        assert_eq!(search.parameters[0], ("path", "production/database"));

        let search = router.search("/backups/dev/application.log.bak").unwrap();
        assert_eq!(search.data, &10);
        assert_eq!(search.template, "/backups/<*path>.<ext>");
        assert_eq!(search.parameters[0], ("path", "dev/application.log"));
        assert_eq!(search.parameters[1], ("ext", "bak"));

        let search = router.search("/backups/.bak");
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
│     ╰─ .
│        ├─ tar.gz
│        ╰─ <ext>
├─ files/
│  ├─ <*path>
│  │  ╰─ /delete
│  ╰─ <*path>
├─ health
├─ images/
│  ╰─ <name>
│     ╰─ .
│        ├─ png
│        ╰─ <ext>
╰─ users/
   ╰─ <id>
      ╰─ /message
```

## Implementation details

`wayfind` uses a compressed radix trie for its data storage.
This is the common backbone of almost all routers implemented in Rust.

What sets `wayfind` apart is its search strategy.
Most routers either use "first match wins" or "best match wins" (via backtracking), `wayfind` uses a hybrid approach:

- per segment: first match wins
- within segment: best match wins

You only pay the cost of backtracking if you make use of inline parameters, and only for that given segment.

This can result in some matches which may be unexpected, but in practice it works well for real-world usage.

## Performance

`wayfind` is fast, and appears to be competitive against other top performers in all benchmarks we currently run.

See [BENCHMARKING.md](BENCHMARKING.md) for the results.

## License

`wayfind` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
