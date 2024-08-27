#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut router = wayfind::Router::new();
    if let Ok(route) = std::str::from_utf8(data) {
        let _ = router.insert(route, true);
    }
});
