# Benchmarking

All benchmarks ran on a MBP (aarch64-linux, 2021 Apple M1 Pro).

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
| xitca-router     | no               | yes               |

As such, we provide 2 sets of results per benchmark:
- one with the default behaviour of the router.
- one with the parameters extracted to `Vec<(&str, &str)>`.

We use [`divan`](https://github.com/nvzqz/divan) for our benchmarks, taking the 'median' results from its output to create the following tables.

### `matchit` inspired benches

In a router of 130 templates, benchmark matching 130 paths.

#### Default

| Library          |     Time | Allocs |   Memory |
|:-----------------|---------:|-------:|---------:|
| matchit          | 5.165 µs |      1 |    128 B |
| wayfind          | 6.249 µs |      0 |      n/a |
| path-tree        | 9.166 µs |      0 |      n/a |
| xitca-router     | 13.79 µs |    103 | 13.18 KB |
| ntex-router      | 61.77 µs |    306 | 22.97 KB |
| route-recognizer | 75.54 µs |   3596 | 195.6 KB |
| actix-router     | 1.611 ms |   6934 | 447.4 KB |

#### String Parameters

| Library          |     Time | Allocs |   Memory |
|:-----------------|---------:|-------:|---------:|
| wayfind          | 7.755 µs |    103 | 6.464 KB |
| matchit          | 10.62 µs |    104 | 13.31 KB |
| path-tree        | 13.46 µs |    103 | 13.18 KB |
| xitca-router     | 16.01 µs |    206 | 26.36 KB |
| ntex-router      | 64.11 µs |    409 | 36.16 KB |
| route-recognizer | 76.34 µs |   3699 | 208.8 KB |
| actix-router     | 1.617 ms |   7037 | 460.6 KB |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          |     Time | Allocs |   Memory |
|:-----------------|---------:|-------:|---------:|
| wayfind          | 3.020 µs |      0 |      n/a |
| path-tree        | 4.957 µs |      0 |      n/a |
| matchit          | 5.864 µs |     81 | 10.36 KB |
| xitca-router     | 8.825 µs |    150 | 18.06 KB |
| ntex-router      | 55.12 µs |    142 | 12.09 KB |
| route-recognizer | 77.13 µs |   2813 | 184.2 KB |
| actix-router     | 532.7 µs |   2142 | 121.4 KB |

#### String Parameters

| Library          |     Time | Allocs |   Memory |
|:-----------------|---------:|-------:|---------:|
| wayfind          | 3.849 µs |     58 | 2.528 KB |
| matchit          | 7.463 µs |    139 | 17.79 KB |
| path-tree        | 7.390 µs |     58 | 7.424 KB |
| xitca-router     | 9.841 µs |    208 | 25.49 KB |
| ntex-router      | 56.97 µs |    200 | 19.52 KB |
| route-recognizer | 77.16 µs |   2871 | 191.6 KB |
| actix-router     | 546.9 µs |   2200 | 128.8 KB |
