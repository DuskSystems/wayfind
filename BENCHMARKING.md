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
- one with the parameters extracted to `Vec<(&str, &str)>`.

We use [`divan`](https://github.com/nvzqz/divan) for our benchmarks, taking the 'median' results from its output to create the following tables.

### `matchit` inspired benches

In a router of 130 templates, benchmark matching 130 paths.

#### Default

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| matchit          | 5.416 µs   | 7.960 µs    | 1           | 128 B      |
| wayfind          | 7.166 µs   | 14.00 µs    | 0           | n/a        |
| path-tree        | 9.082 µs   | 16.17 µs    | 0           | n/a        |
| xitca-router     | 14.74 µs   | 20.00 µs    | 103         | 13.18 KB   |
| ntex-router      | 44.88 µs   | 74.83 µs    | 306         | 22.97 KB   |
| route-recognizer | 74.83 µs   | 123.9 µs    | 3596        | 195.6 KB   |
| actix-router     | 555.7 µs   | 1.018 ms    | 6934        | 447.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 8.787 µs   | 18.19 µs    | 103         | 6.464 KB   |
| matchit          | 10.70 µs   | 14.17 µs    | 104         | 13.31 KB   |
| path-tree        | 13.41 µs   | 22.29 µs    | 103         | 13.18 KB   |
| xitca-router     | 16.78 µs   | 22.75 µs    | 206         | 26.36 KB   |
| ntex-router      | 47.65 µs   | 77.82 µs    | 409         | 36.16 KB   |
| route-recognizer | 76.08 µs   | 127.5 µs    | 3699        | 208.8 KB   |
| actix-router     | 563.1 µs   | 1.020 ms    | 7037        | 460.6 KB   |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 3.790 µs   | 6.786 µs    | 0           | n/a        |
| path-tree        | 4.999 µs   | 8.056 µs    | 0           | n/a        |
| matchit          | 6.119 µs   | 9.260 µs    | 81          | 10.36 KB   |
| xitca-router     | 9.572 µs   | 11.74 µs    | 150         | 18.06 KB   |
| ntex-router      | 28.15 µs   | 46.65 µs    | 142         | 12.09 KB   |
| route-recognizer | 76.30 µs   | 128.8 µs    | 2813        | 184.2 KB   |
| actix-router     | 188.2 µs   | 334.5 µs    | 2142        | 121.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 4.537 µs   | 8.411 µs    | 58          | 2.528 KB   |
| path-tree        | 7.370 µs   | 11.37 µs    | 58          | 7.424 KB   |
| matchit          | 7.782 µs   | 11.43 µs    | 139         | 17.79 KB   |
| xitca-router     | 10.56 µs   | 12.97 µs    | 208         | 25.49 KB   |
| ntex-router      | 29.73 µs   | 49.47 µs    | 200         | 19.52 KB   |
| route-recognizer | 77.17 µs   | 129.4 µs    | 2871        | 191.6 KB   |
| actix-router     | 190.2 µs   | 329.4 µs    | 2200        | 128.8 KB   |

## `wayfind` benches

TODO.

The features `wayfind` provides come with inherent risk.
It is possible to insert a 'bad template' that poisons the performance of the entire router.
As such, it would be nice to have more `wayfind` focused benchmarks, to measure such scenarios.
