#![no_main]

use libfuzzer_sys::fuzz_target;
use wayfind::Router;

fuzz_target!(|inputs: Vec<&str>| {
    let mut router = Router::new();

    for (index, input) in inputs.iter().enumerate() {
        drop(router.insert(input, index));
    }

    for input in &inputs {
        drop(router.search(input));
    }
});
