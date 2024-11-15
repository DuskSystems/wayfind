#![no_main]

use libfuzzer_sys::fuzz_target;
use wayfind::{Path, RoutableBuilder, Router};

fuzz_target!(|data: &[u8]| {
    let mut router = Router::new();
    if let Ok(route) = std::str::from_utf8(data) {
        let routable = RoutableBuilder::new().route(route).build().unwrap();
        let _ = router.insert(&routable, true);
        if let Ok(path) = Path::new(route) {
            let _ = router.search(&path);
        }
    }
});
