#![no_main]

use libfuzzer_sys::fuzz_target;
use wayfind::{Path, Router};

fuzz_target!(|data: &[u8]| {
    let mut router = Router::new();
    if let Ok(route) = std::str::from_utf8(data) {
        let _ = router.insert(route, true);
        if let Ok(path) = Path::new(route) {
            let _ = router.search(&path);
        }
    }
});
