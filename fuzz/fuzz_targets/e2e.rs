#![no_main]

use libfuzzer_sys::fuzz_target;
use wayfind::{RequestBuilder, RoutableBuilder, Router};

fuzz_target!(|data: &[u8]| {
    let mut router = Router::new();
    if let Ok(route) = std::str::from_utf8(data) {
        if let Ok(route) = RoutableBuilder::new().route(route).build() {
            let _ = router.insert(&route, true);
        }

        if let Ok(path) = RequestBuilder::new().path(route).build() {
            let _ = router.search(&path);
        }
    }
});
