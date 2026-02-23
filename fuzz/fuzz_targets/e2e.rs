#![no_main]

use core::fmt::Write as _;

use libfuzzer_sys::fuzz_target;
use wayfind::Router;

fuzz_target!(|inputs: Vec<&str>| {
    let mut router = Router::new();
    let mut inserted = vec![];

    for (index, input) in inputs.iter().enumerate() {
        if router.insert(input, index).is_ok() {
            inserted.push((index, *input));
        }
    }

    for (index, input) in &inserted {
        assert_eq!(router.get(input), Some(index));
        assert!(router.search(input).is_some());
    }

    for input in &inputs {
        let _search = router.search(input);
    }

    let mut display = String::new();
    let _display = write!(display, "{router}");

    for (_, input) in &inserted {
        let _delete = router.delete(input);
    }

    for (_, input) in &inserted {
        assert!(router.get(input).is_none());
        assert!(router.search(input).is_none());
    }
});
