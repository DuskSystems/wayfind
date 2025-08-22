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
| matchit          | 5.457 µs   | 7.766 µs    | 1           | 128 B      |
| wayfind          | 7.374 µs   | 14.17 µs    | 0           | n/a        |
| path-tree        | 9.082 µs   | 16.27 µs    | 0           | n/a        |
| xitca-router     | 14.87 µs   | 19.78 µs    | 103         | 13.18 KB   |
| ntex-router      | 44.87 µs   | 73.81 µs    | 306         | 22.97 KB   |
| route-recognizer | 76.20 µs   | 124.8 µs    | 3596        | 195.6 KB   |
| actix-router     | 563.1 µs   | 1.023 ms    | 6934        | 447.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| matchit          | 7.708 µs   | 13.55 µs    | 1           | 128 B      |
| wayfind          | 7.874 µs   | 15.12 µs    | 0           | n/a        |
| path-tree        | 10.49 µs   | 19.96 µs    | 0           | n/a        |
| xitca-router     | 15.45 µs   | 20.48 µs    | 103         | 13.18 KB   |
| ntex-router      | 45.33 µs   | 75.85 µs    | 306         | 22.97 KB   |
| route-recognizer | 76.66 µs   | 126.4 µs    | 3596        | 195.6 KB   |
| actix-router     | 572.9 µs   | 1.020 ms    | 6934        | 447.4 KB   |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 3.624 µs   | 6.893 µs    | 0           | n/a        |
| path-tree        | 4.874 µs   | 8.355 µs    | 0           | n/a        |
| matchit          | 6.122 µs   | 7.416 µs    | 81          | 10.36 KB   |
| xitca-router     | 9.536 µs   | 11.68 µs    | 150         | 18.06 KB   |
| ntex-router      | 28.32 µs   | 45.45 µs    | 142         | 12.09 KB   |
| route-recognizer | 76.95 µs   | 113.9 µs    | 2813        | 184.2 KB   |
| actix-router     | 188.4 µs   | 330.6 µs    | 2142        | 121.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 3.958 µs   | 7.629 µs    | 0           | n/a        |
| path-tree        | 5.665 µs   | 10.15 µs    | 0           | n/a        |
| matchit          | 7.122 µs   | 10.25 µs    | 81          | 10.36 KB   |
| xitca-router     | 9.828 µs   | 12.12 µs    | 150         | 18.06 KB   |
| ntex-router      | 28.53 µs   | 46.45 µs    | 142         | 12.09 KB   |
| route-recognizer | 76.74 µs   | 129.5 µs    | 2813        | 184.2 KB   |
| actix-router     | 192.1 µs   | 343.1 µs    | 2142        | 121.4 KB   |

## `wayfind` benches

TODO.

The features `wayfind` provides come with inherent risk.
It is possible to insert a 'bad template' that poisons the performance of the entire router.
As such, it would be nice to have more `wayfind` focused benchmarks, to measure such scenarios.
