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
| matchit          | 5.374 µs   | 7.719 µs    | 1           | 128 B      |
| wayfind          | 6.999 µs   | 13.61 µs    | 0           | n/a        |
| path-tree        | 9.374 µs   | 15.61 µs    | 0           | n/a        |
| xitca-router     | 14.62 µs   | 20.13 µs    | 103         | 13.18 KB   |
| ntex-router      | 44.81 µs   | 74.84 µs    | 306         | 22.97 KB   |
| route-recognizer | 76.50 µs   | 125.1 µs    | 3596        | 195.6 KB   |
| actix-router     | 562.4 µs   | 1.040 ms    | 6934        | 447.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 8.333 µs   | 17.86 µs    | 103         | 6.464 KB   |
| matchit          | 10.79 µs   | 14.15 µs    | 104         | 13.31 KB   |
| path-tree        | 13.79 µs   | 22.41 µs    | 103         | 13.18 KB   |
| xitca-router     | 16.95 µs   | 22.63 µs    | 206         | 26.36 KB   |
| ntex-router      | 47.50 µs   | 78.46 µs    | 409         | 36.16 KB   |
| route-recognizer | 77.91 µs   | 128.8 µs    | 3699        | 208.8 KB   |
| actix-router     | 569.5 µs   | 1.039 ms    | 7037        | 460.6 KB   |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 3.624 µs   | 6.364 µs    | 0           | n/a        |
| path-tree        | 5.165 µs   | 8.059 µs    | 0           | n/a        |
| matchit          | 6.109 µs   | 7.498 µs    | 81          | 10.36 KB   |
| xitca-router     | 9.477 µs   | 11.70 µs    | 150         | 18.06 KB   |
| ntex-router      | 27.94 µs   | 46.75 µs    | 142         | 12.09 KB   |
| route-recognizer | 74.41 µs   | 128.1 µs    | 2813        | 184.2 KB   |
| actix-router     | 188.3 µs   | 335.5 µs    | 2142        | 121.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 4.375 µs   | 7.991 µs    | 58          | 2.528 KB   |
| path-tree        | 7.333 µs   | 11.53 µs    | 58          | 7.424 KB   |
| matchit          | 7.777 µs   | 11.44 µs    | 139         | 17.79 KB   |
| xitca-router     | 10.47 µs   | 12.74 µs    | 208         | 25.49 KB   |
| ntex-router      | 29.52 µs   | 49.12 µs    | 200         | 19.52 KB   |
| route-recognizer | 75.12 µs   | 130.0 µs    | 2871        | 191.6 KB   |
| actix-router     | 190.8 µs   | 330.5 µs    | 2200        | 128.8 KB   |

## `wayfind` benches

TODO.

The features `wayfind` provides come with inherent risk.
It is possible to insert a 'bad template' that poisons the performance of the entire router.
As such, it would be nice to have more `wayfind` focused benchmarks, to measure such scenarios.
