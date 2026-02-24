#![no_main]

use std::sync::LazyLock;

use libfuzzer_sys::fuzz_target;
use wayfind::Router;

#[path = "../../benches/fixtures/gitlab_routes.rs"]
mod gitlab_routes;

static ROUTER: LazyLock<Router<usize>> = LazyLock::new(|| {
    let mut router = Router::new();
    for (index, route) in gitlab_routes::routes().iter().enumerate() {
        router.insert(route, index).unwrap();
    }

    router
});

fuzz_target!(|input: &str| {
    let _search = ROUTER.search(input);
});
