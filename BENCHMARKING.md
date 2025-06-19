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
| matchit          | 5.458 µs   | 8.3540 µs   | 1           | 128 B      |
| wayfind          | 7.583 µs   | 15.480 µs   | 0           | n/a        |
| path-tree        | 8.937 µs   | 15.890 µs   | 0           | n/a        |
| xitca-router     | 14.79 µs   | 19.040 µs   | 103         | 13.18 KB   |
| ntex-router      | 47.29 µs   | 78.720 µs   | 306         | 22.97 KB   |
| route-recognizer | 76.19 µs   | 140.30 µs   | 3596        | 195.6 KB   |
| actix-router     | 557.3 µs   | 983.90 µs   | 6934        | 447.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| matchit          | 7.499 µs   | 13.940 µs   | 1           | 128 B      |
| wayfind          | 7.874 µs   | 16.560 µs   | 0           | n/a        |
| path-tree        | 10.68 µs   | 20.120 µs   | 0           | n/a        |
| xitca-router     | 15.29 µs   | 19.590 µs   | 103         | 13.18 KB   |
| ntex-router      | 48.24 µs   | 80.640 µs   | 306         | 22.97 KB   |
| route-recognizer | 77.27 µs   | 141.90 µs   | 3596        | 195.6 KB   |
| actix-router     | 563.8 µs   | 1002.0 µs   | 6934        | 447.4 KB   |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 3.749 µs   | 7.890 µs    | 0           | n/a        |
| path-tree        | 5.082 µs   | 8.457 µs    | 0           | n/a        |
| matchit          | 6.118 µs   | 7.006 µs    | 81          | 10.36 KB   |
| xitca-router     | 9.488 µs   | 11.23 µs    | 150         | 18.06 KB   |
| ntex-router      | 28.48 µs   | 47.31 µs    | 142         | 12.09 KB   |
| route-recognizer | 67.78 µs   | 146.4 µs    | 2813        | 184.2 KB   |
| actix-router     | 209.8 µs   | 330.6 µs    | 2142        | 121.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 4.041 µs   | 8.815 µs    | 0           | n/a        |
| path-tree        | 5.707 µs   | 10.66 µs    | 0           | n/a        |
| matchit          | 7.159 µs   | 9.331 µs    | 81          | 10.36 KB   |
| xitca-router     | 9.738 µs   | 12.08 µs    | 150         | 18.06 KB   |
| ntex-router      | 28.82 µs   | 48.34 µs    | 142         | 12.09 KB   |
| route-recognizer | 68.41 µs   | 148.5 µs    | 2813        | 184.2 KB   |
| actix-router     | 186.2 µs   | 346.6 µs    | 2142        | 121.4 KB   |

## `wayfind` benches

TODO.

The features `wayfind` provides come with inherent risk.
It is possible to insert a 'bad template' that poisons the performance of the entire router.
For example, having a top-level constrained wildcard route like `/{*path:constraint}`.
As such, it would be nice to have more `wayfind` focused benchmarks, to measure such scenarios.
