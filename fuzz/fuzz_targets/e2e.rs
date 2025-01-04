#![no_main]

use libfuzzer_sys::fuzz_target;
use wayfind::Router;

fuzz_target!(|data: &[u8]| {
    let mut router = Router::new();
    if let Ok(input) = std::str::from_utf8(data) {
        let _ = router.insert(input, true);
        let _ = router.search(input);
    }
});
