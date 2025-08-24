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
| matchit          | 5.499 µs   | 8.017 µs    | 1           | 128 B      |
| wayfind          | 6.957 µs   | 14.54 µs    | 0           | n/a        |
| path-tree        | 9.082 µs   | 15.82 µs    | 0           | n/a        |
| xitca-router     | 14.61 µs   | 19.74 µs    | 103         | 13.18 KB   |
| ntex-router      | 44.66 µs   | 73.91 µs    | 306         | 22.97 KB   |
| route-recognizer | 75.80 µs   | 124.0 µs    | 3596        | 195.6 KB   |
| actix-router     | 564.8 µs   | 1.021 ms    | 6934        | 447.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| matchit          | 7.499 µs   | 14.62 µs    | 1           | 128 B      |
| wayfind          | 7.499 µs   | 14.79 µs    | 0           | n/a        |
| path-tree        | 10.41 µs   | 20.35 µs    | 0           | n/a        |
| xitca-router     | 15.32 µs   | 20.35 µs    | 103         | 13.18 KB   |
| ntex-router      | 45.20 µs   | 75.58 µs    | 306         | 22.97 KB   |
| route-recognizer | 76.59 µs   | 128.6 µs    | 3596        | 195.6 KB   |
| actix-router     | 585.9 µs   | 1.021 ms    | 6934        | 447.4 KB   |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 3.583 µs   | 6.735 µs    | 0           | n/a        |
| path-tree        | 5.208 µs   | 8.198 µs    | 0           | n/a        |
| matchit          | 6.094 µs   | 7.425 µs    | 81          | 10.36 KB   |
| xitca-router     | 9.485 µs   | 11.89 µs    | 150         | 18.06 KB   |
| ntex-router      | 28.28 µs   | 46.25 µs    | 142         | 12.09 KB   |
| route-recognizer | 76.00 µs   | 129.0 µs    | 2813        | 184.2 KB   |
| actix-router     | 187.3 µs   | 337.3 µs    | 2142        | 121.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 3.916 µs   | 7.475 µs    | 0           | n/a        |
| path-tree        | 5.832 µs   | 9.970 µs    | 0           | n/a        |
| matchit          | 7.136 µs   | 10.20 µs    | 81          | 10.36 KB   |
| xitca-router     | 9.736 µs   | 12.00 µs    | 150         | 18.06 KB   |
| ntex-router      | 28.59 µs   | 46.85 µs    | 142         | 12.09 KB   |
| route-recognizer | 76.00 µs   | 128.6 µs    | 2813        | 184.2 KB   |
| actix-router     | 189.4 µs   | 337.3 µs    | 2142        | 121.4 KB   |

## `wayfind` benches

TODO.

The features `wayfind` provides come with inherent risk.
It is possible to insert a 'bad template' that poisons the performance of the entire router.
As such, it would be nice to have more `wayfind` focused benchmarks, to measure such scenarios.
