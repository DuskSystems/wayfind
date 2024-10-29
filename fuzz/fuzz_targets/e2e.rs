#![no_main]

use libfuzzer_sys::fuzz_target;
use wayfind::Router;

fuzz_target!(|data: &[u8]| {
    let mut router = Router::new();
    if let Ok(route) = std::str::from_utf8(data) {
        let _ = router.insert(route, true);
        let _ = router.search(route);
    }
});
