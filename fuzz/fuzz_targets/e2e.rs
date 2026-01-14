#![no_main]

use libfuzzer_sys::fuzz_target;
use wayfind::Router;

fuzz_target!(|input: &str| {
    let mut router = Router::new();
    let _unused = router.insert(input, true);
    let _unused = router.search(input);
});
