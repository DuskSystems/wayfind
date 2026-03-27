#![expect(missing_docs, clippy::unwrap_used, reason = "Fuzz")]
#![no_main]

use std::sync::LazyLock;

use libfuzzer_sys::fuzz_target;
use wayfind::{Router, RouterBuilder};

#[path = "../../benches/fixtures/gitlab_routes.rs"]
mod gitlab_routes;

static ROUTER: LazyLock<Router<usize>> = LazyLock::new(|| {
    let mut builder = RouterBuilder::new();
    for (index, route) in gitlab_routes::routes().iter().enumerate() {
        builder.insert(route, index).unwrap();
    }

    builder.build()
});

fuzz_target!(|input: &str| {
    let _search = ROUTER.search(input);
});
