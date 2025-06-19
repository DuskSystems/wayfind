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
| matchit          | 5.457 µs   | 8.148 µs    | 1           | 128 B      |
| wayfind          | 7.416 µs   | 15.85 µs    | 0           | n/a        |
| path-tree        | 8.958 µs   | 16.23 µs    | 0           | n/a        |
| xitca-router     | 14.78 µs   | 18.79 µs    | 103         | 13.18 KB   |
| ntex-router      | 47.41 µs   | 78.92 µs    | 306         | 22.97 KB   |
| route-recognizer | 75.32 µs   | 139.2 µs    | 3596        | 195.6 KB   |
| actix-router     | 569.6 µs   | 958.7 µs    | 6934        | 447.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| matchit          | 7.583 µs   | 13.90 µs    | 1           | 128 B      |
| wayfind          | 7.833 µs   | 16.62 µs    | 0           | n/a        |
| path-tree        | 10.37 µs   | 20.06 µs    | 0           | n/a        |
| xitca-router     | 15.41 µs   | 19.67 µs    | 103         | 13.18 KB   |
| ntex-router      | 48.20 µs   | 80.11 µs    | 306         | 22.97 KB   |
| route-recognizer | 76.30 µs   | 142.2 µs    | 3596        | 195.6 KB   |
| actix-router     | 574.3 µs   | 983.9 µs    | 6934        | 447.4 KB   |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 3.916 µs   | 7.790 µs    | 0           | n/a        |
| path-tree        | 4.957 µs   | 8.475 µs    | 0           | n/a        |
| matchit          | 6.163 µs   | 6.843 µs    | 81          | 10.36 KB   |
| xitca-router     | 9.536 µs   | 11.20 µs    | 150         | 18.06 KB   |
| ntex-router      | 28.66 µs   | 45.30 µs    | 142         | 12.09 KB   |
| route-recognizer | 67.89 µs   | 131.4 µs    | 2813        | 184.2 KB   |
| actix-router     | 194.6 µs   | 326.8 µs    | 2142        | 121.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 4.208 µs   | 8.897 µs    | 0           | n/a        |
| path-tree        | 5.791 µs   | 10.71 µs    | 0           | n/a        |
| matchit          | 7.122 µs   | 9.431 µs    | 81          | 10.36 KB   |
| xitca-router     | 9.745 µs   | 12.86 µs    | 150         | 18.06 KB   |
| ntex-router      | 28.99 µs   | 46.34 µs    | 142         | 12.09 KB   |
| route-recognizer | 68.91 µs   | 134.3 µs    | 2813        | 184.2 KB   |
| actix-router     | 188.9 µs   | 341.0 µs    | 2142        | 121.4 KB   |

## `wayfind` benches

TODO.

The features `wayfind` provides come with inherent risk.
It is possible to insert a 'bad template' that poisons the performance of the entire router.
For example, having a top-level constrained wildcard route like `/{*path:constraint}`.
As such, it would be nice to have more `wayfind` focused benchmarks, to measure such scenarios.
