#![no_main]

use libfuzzer_sys::fuzz_target;
use wayfind::Router;

fuzz_target!(|data: &[u8]| {
    let mut router = Router::new();
    if let Ok(input) = core::str::from_utf8(data) {
        let _unused = router.insert(input, true);
        let _unused = router.search(input);
    }
});
