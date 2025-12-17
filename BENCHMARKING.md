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
| matchit          | 5.415 µs   | 7.854 µs    | 1           | 128 B      |
| wayfind          | 7.041 µs   | 14.20 µs    | 0           | n/a        |
| path-tree        | 8.979 µs   | 17.76 µs    | 0           | n/a        |
| xitca-router     | 14.33 µs   | 20.91 µs    | 103         | 13.18 KB   |
| ntex-router      | 44.45 µs   | 75.13 µs    | 306         | 22.97 KB   |
| route-recognizer | 73.81 µs   | 118.9 µs    | 3596        | 195.6 KB   |
| actix-router     | 552.2 µs   | 1.121 ms    | 6934        | 447.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 8.208 µs   | 18.19 µs    | 103         | 6.464 KB   |
| matchit          | 10.95 µs   | 14.86 µs    | 104         | 13.31 KB   |
| path-tree        | 13.62 µs   | 23.88 µs    | 103         | 13.18 KB   |
| xitca-router     | 16.52 µs   | 22.39 µs    | 206         | 26.36 KB   |
| ntex-router      | 47.95 µs   | 81.50 µs    | 409         | 36.16 KB   |
| route-recognizer | 75.37 µs   | 125.0 µs    | 3699        | 208.8 KB   |
| actix-router     | 545.7 µs   | 1.130 ms    | 7037        | 460.6 KB   |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 3.791 µs   | 6.990 µs    | 0           | n/a        |
| path-tree        | 4.957 µs   | 8.680 µs    | 0           | n/a        |
| matchit          | 5.790 µs   | 7.789 µs    | 81          | 10.36 KB   |
| xitca-router     | 9.208 µs   | 13.02 µs    | 150         | 18.06 KB   |
| ntex-router      | 28.12 µs   | 46.02 µs    | 142         | 12.09 KB   |
| route-recognizer | 71.83 µs   | 112.2 µs    | 2813        | 184.2 KB   |
| actix-router     | 179.8 µs   | 354.9 µs    | 2142        | 121.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 4.583 µs   | 8.837 µs    | 58          | 2.528 KB   |
| path-tree        | 7.416 µs   | 16.68 µs    | 58          | 7.424 KB   |
| matchit          | 7.458 µs   | 13.99 µs    | 139         | 17.79 KB   |
| xitca-router     | 10.25 µs   | 12.69 µs    | 208         | 25.49 KB   |
| ntex-router      | 29.62 µs   | 48.20 µs    | 200         | 19.52 KB   |
| route-recognizer | 72.62 µs   | 114.2 µs    | 2871        | 191.6 KB   |
| actix-router     | 182.1 µs   | 354.2 µs    | 2200        | 128.8 KB   |
