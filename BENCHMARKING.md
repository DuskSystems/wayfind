# Benchmarking

All benchmarks are ran on:
1. MBP: (aarch64-linux, 2021 Apple M1 Pro)
2. M93p: (x86_64-linux, 2013 i7-4785T)

## Context

For all benchmarks, we convert any extracted parameters to strings.

All routers provide a way to return parameters as strings, but some delay the actual UTF-8 decoding until post-search.

| Library          | Percent Decoding | String Parameters |
|:-----------------|:----------------:|:-----------------:|
| wayfind          | no               | yes               |
| actix-router     | partial          | yes               |
| matchit          | no               | delayed           |
| ntex-router      | partial          | yes               |
| path-tree        | no               | delayed           |
| route-recognizer | no               | yes               |
| routefinder      | no               | yes               |
| xitca-router     | no               | yes               |

As such, we provide 2 sets of results per benchmark:
- one with the default behaviour of the router.
- one with the parameters extracted to `SmallVec<[(&str, &str); 4]>`.

We use [`divan`](https://github.com/nvzqz/divan) for our benchmarks, taking the 'median' results from its output to create the following tables.

### `matchit` inspired benches

In a router of 130 templates, benchmark matching 130 paths.

#### Default

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| matchit          | 5.499 µs   | 7.775 µs    | 1           | 128 B      |
| wayfind          | 7.707 µs   | 15.30 µs    | 0           | n/a        |
| path-tree        | 8.957 µs   | 15.51 µs    | 0           | n/a        |
| xitca-router     | 14.72 µs   | 19.71 µs    | 103         | 13.18 KB   |
| ntex-router      | 44.45 µs   | 74.76 µs    | 306         | 22.97 KB   |
| route-recognizer | 76.58 µs   | 125.0 µs    | 3596        | 195.6 KB   |
| actix-router     | 563.2 µs   | 1.009 ms    | 6934        | 447.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| matchit          | 7.624 µs   | 13.28 µs    | 1           | 128 B      |
| wayfind          | 8.124 µs   | 16.45 µs    | 0           | n/a        |
| path-tree        | 10.29 µs   | 19.51 µs    | 0           | n/a        |
| xitca-router     | 15.29 µs   | 23.39 µs    | 103         | 13.18 KB   |
| ntex-router      | 44.91 µs   | 76.27 µs    | 306         | 22.97 KB   |
| route-recognizer | 77.08 µs   | 127.0 µs    | 3596        | 195.6 KB   |
| actix-router     | 571.1 µs   | 1.031 ms    | 6934        | 447.4 KB   |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 3.791 µs   | 7.499 µs    | 0           | n/a        |
| path-tree        | 4.957 µs   | 8.991 µs    | 0           | n/a        |
| matchit          | 6.037 µs   | 7.849 µs    | 81          | 10.36 KB   |
| xitca-router     | 9.533 µs   | 11.48 µs    | 150         | 18.06 KB   |
| ntex-router      | 30.90 µs   | 53.71 µs    | 142         | 12.09 KB   |
| route-recognizer | 76.81 µs   | 129.4 µs    | 2813        | 184.2 KB   |
| actix-router     | 188.3 µs   | 340.7 µs    | 2142        | 121.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 4.166 µs   | 8.015 µs    | 0           | n/a        |
| path-tree        | 5.708 µs   | 10.12 µs    | 0           | n/a        |
| matchit          | 7.204 µs   | 10.16 µs    | 81          | 10.36 KB   |
| xitca-router     | 9.824 µs   | 11.98 µs    | 150         | 18.06 KB   |
| ntex-router      | 28.61 µs   | 47.48 µs    | 142         | 12.09 KB   |
| route-recognizer | 77.02 µs   | 130.4 µs    | 2813        | 184.2 KB   |
| actix-router     | 190.6 µs   | 338.2 µs    | 2142        | 121.4 KB   |

## `wayfind` benches

TODO.

The features `wayfind` provides come with inherent risk.
It is possible to insert a 'bad template' that poisons the performance of the entire router.
As such, it would be nice to have more `wayfind` focused benchmarks, to measure such scenarios.
