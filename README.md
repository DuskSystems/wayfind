![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![crates.io](https://img.shields.io/crates/v/wayfind)](https://crates.io/crates/wayfind)
[![documentation](https://docs.rs/wayfind/badge.svg)](https://docs.rs/wayfind)

![rust: 1.85+](https://img.shields.io/badge/rust-1.85+-orange.svg)
![`unsafe`: forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)
![`wasm`: compatible](https://img.shields.io/badge/wasm-compatible-success.svg)
![`no-std`: compatible](https://img.shields.io/badge/no--std-compatible-success.svg)

[![codecov](https://codecov.io/gh/DuskSystems/wayfind/graph/badge.svg?token=QMSW55438K)](https://codecov.io/gh/DuskSystems/wayfind)

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
wayfind = "0.8"
```

```rust
use std::error::Error;

use wayfind::Router;

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/pet", 1)?;
    router.insert("/pet/findByStatus", 2)?;
    router.insert("/pet/findByTags", 3)?;
    router.insert("/pet/<pet>", 4)?;
    router.insert("/pet/<petId>/uploadImage", 5)?;
    router.insert("/store/inventory", 6)?;
    router.insert("/store/order", 7)?;
    router.insert("/store/order/<orderId>", 8)?;
    router.insert("/user", 9)?;
    router.insert("/user/createWithList", 10)?;
    router.insert("/user/login", 11)?;
    router.insert("/user/logout", 12)?;
    router.insert("/user/<username>", 13)?;
    router.insert("/<*catch_all>", 14)?;

    let search = router.search("/pet").unwrap();
    assert_eq!(*search.data, 1);

    let search = router.search("/pet/123/uploadImage").unwrap();
    assert_eq!(*search.data, 5);
    assert_eq!(search.parameters[0], ("petId", "123"));

    let search = router.search("/store/order").unwrap();
    assert_eq!(*search.data, 7);

    let search = router.search("/store/order/456").unwrap();
    assert_eq!(*search.data, 8);
    assert_eq!(search.parameters[0], ("orderId", "456"));

    let search = router.search("/user/alice").unwrap();
    assert_eq!(*search.data, 13);
    assert_eq!(search.parameters[0], ("username", "alice"));

    let search = router.search("/unknown/path").unwrap();
    assert_eq!(*search.data, 14);
    assert_eq!(search.parameters[0], ("catch_all", "unknown/path"));

    println!("{router}");
    Ok(())
}
```

```
/
├─ pet
│  ├─ /findBy
│  │  ├─ Status
│  │  ╰─ Tags
│  ├─ /<pet>
│  ╰─ /<petId>
│     ╰─ /uploadImage
├─ store/
│  ├─ inventory
│  ╰─ order
│     ╰─ /<orderId>
├─ user
│  ├─ /createWithList
│  ├─ /log
│  │  ├─ in
│  │  ╰─ out
│  ╰─ /<username>
╰─ <*catch_all>
```

## Performance

`wayfind` is fast, and appears to be competitive against other top performers in all benchmarks we currently run.

See [BENCHMARKING.md](BENCHMARKING.md) for the results.

## License

`wayfind` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

## Inspirations

- [poem](https://github.com/poem-web/poem): Initial experimentations started out as a Poem router fork
- [matchit](https://github.com/ibraheemdev/matchit): Performance leader among pre-existing routers
- [path-tree](https://github.com/viz-rs/path-tree): Extensive testing and router display feature
