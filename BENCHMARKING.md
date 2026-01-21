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

We use [`divan`](https://github.com/nvzqz/divan) for our benchmarks, taking the 'fastest' results from its output to create the following tables.

### `matchit` inspired benches

In a router of 130 templates, benchmark matching 130 paths.

#### Default

| Library          |     Time | Allocs |   Memory |
|:-----------------|---------:|-------:|---------:|
| matchit          | 5.123 µs |      1 |    128 B |
| wayfind          | 5.707 µs |      0 |      n/a |
| path-tree        | 10.33 µs |      0 |      n/a |
| xitca-router     | 14.21 µs |    103 | 13.18 KB |
| ntex-router      | 64.48 µs |    306 | 22.97 KB |
| route-recognizer | 82.22 µs |   3596 | 195.6 KB |
| actix-router     | 1.731 ms |   6934 | 447.4 KB |

#### String Parameters

| Library          |     Time | Allocs |   Memory |
|:-----------------|---------:|-------:|---------:|
| wayfind          | 7.005 µs |    103 | 6.464 KB |
| matchit          | 10.79 µs |    104 | 13.31 KB |
| path-tree        |    15 µs |    103 | 13.18 KB |
| xitca-router     | 16.88 µs |    206 | 26.36 KB |
| ntex-router      | 67.48 µs |    409 | 36.16 KB |
| route-recognizer | 83.52 µs |   3699 | 208.8 KB |
| actix-router     | 1.738 ms |   7037 | 460.6 KB |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          |     Time | Allocs |   Memory |
|:-----------------|---------:|-------:|---------:|
| wayfind          | 2.666 µs |      0 |      n/a |
| path-tree        | 5.416 µs |      0 |      n/a |
| matchit          | 5.738 µs |     81 | 10.36 KB |
| xitca-router     |  8.95 µs |    150 | 18.06 KB |
| ntex-router      | 54.91 µs |    142 | 12.09 KB |
| route-recognizer | 72.63 µs |   2813 | 184.2 KB |
| actix-router     | 562.5 µs |   2142 | 121.4 KB |

#### String Parameters

| Library          |     Time | Allocs |   Memory |
|:-----------------|---------:|-------:|---------:|
| wayfind          | 3.474 µs |     58 | 2.528 KB |
| matchit          | 7.505 µs |    139 | 17.79 KB |
| path-tree        | 8.099 µs |     58 | 7.424 KB |
| xitca-router     | 10.21 µs |    208 | 25.49 KB |
| ntex-router      |  56.6 µs |    200 | 19.52 KB |
| route-recognizer | 72.85 µs |   2871 | 191.6 KB |
| actix-router     | 569.9 µs |   2200 | 128.8 KB |
