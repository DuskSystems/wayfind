#![expect(missing_docs, clippy::missing_assert_message, reason = "Fuzz")]
#![no_main]

use core::fmt::Write as _;

use libfuzzer_sys::fuzz_target;
use wayfind::RouterBuilder;

fuzz_target!(|inputs: Vec<&str>| {
    let mut builder = RouterBuilder::new();
    let mut inserted = vec![];

    for (index, input) in inputs.iter().enumerate() {
        if builder.insert(input, index).is_ok() {
            inserted.push((index, *input));
        }
    }

    for (index, input) in &inserted {
        assert_eq!(builder.get(input), Some(index));
    }

    let router = builder.build();

    for (_, input) in &inserted {
        assert!(router.search(input).is_some());
    }

    for input in &inputs {
        let _search = router.search(input);
    }

    let mut display = String::new();
    let _display = write!(display, "{router}");
});
