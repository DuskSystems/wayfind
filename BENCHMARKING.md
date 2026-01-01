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
| matchit          | 5.374 µs   | 7.918 µs    | 1           | 128 B      |
| wayfind          | 6.832 µs   | 13.60 µs    | 0           | n/a        |
| path-tree        | 8.749 µs   | 15.80 µs    | 0           | n/a        |
| xitca-router     | 14.03 µs   | 18.98 µs    | 103         | 13.18 KB   |
| ntex-router      | 44.33 µs   | 75.15 µs    | 306         | 22.97 KB   |
| route-recognizer | 72.65 µs   | 117.7 µs    | 3596        | 195.6 KB   |
| actix-router     | 537.6 µs   | 1.061 ms    | 6934        | 447.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 7.916 µs   | 17.44 µs    | 103         | 6.464 KB   |
| matchit          | 10.87 µs   | 16.51 µs    | 104         | 13.31 KB   |
| path-tree        | 13.49 µs   | 23.31 µs    | 103         | 13.18 KB   |
| xitca-router     | 16.23 µs   | 21.41 µs    | 206         | 26.36 KB   |
| ntex-router      | 47.20 µs   | 80.11 µs    | 409         | 36.16 KB   |
| route-recognizer | 74.14 µs   | 121.4 µs    | 3699        | 208.8 KB   |
| actix-router     | 534.3 µs   | 1.068 ms    | 7037        | 460.6 KB   |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 3.416 µs   | 6.709 µs    | 0           | n/a        |
| path-tree        | 4.874 µs   | 8.207 µs    | 0           | n/a        |
| matchit          | 5.748 µs   | 6.097 µs    | 81          | 10.36 KB   |
| xitca-router     | 8.872 µs   | 9.956 µs    | 150         | 18.06 KB   |
| ntex-router      | 28.28 µs   | 46.43 µs    | 142         | 12.09 KB   |
| route-recognizer | 70.48 µs   | 109.3 µs    | 2813        | 184.2 KB   |
| actix-router     | 177.4 µs   | 348.8 µs    | 2142        | 121.4 KB   |

#### String Parameters

| Library          | Time (MBP) | Time (M93p) | Alloc Count | Alloc Size |
|:-----------------|-----------:|------------:|------------:|-----------:|
| wayfind          | 4.120 µs   | 8.324 µs    | 58          | 2.528 KB   |
| matchit          | 7.373 µs   | 9.338 µs    | 139         | 17.79 KB   |
| path-tree        | 7.373 µs   | 11.00 µs    | 58          | 7.424 KB   |
| xitca-router     | 10.03 µs   | 10.81 µs    | 208         | 25.49 KB   |
| ntex-router      | 29.99 µs   | 52.22 µs    | 200         | 19.52 KB   |
| route-recognizer | 71.43 µs   | 110.4 µs    | 2871        | 191.6 KB   |
| actix-router     | 181.8 µs   | 350.9 µs    | 2200        | 128.8 KB   |
