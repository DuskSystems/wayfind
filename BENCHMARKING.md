# Benchmarking

All benchmarks ran on a M1 Pro laptop running Asahi Linux.

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

| Library          | Time     | Alloc Count | Alloc Size |
|:-----------------|---------:|------------:|-----------:|
| matchit          | 5.458 µs | 1           | 128 B      |
| wayfind          | 7.583 µs | 0           | n/a        |
| path-tree        | 8.937 µs | 0           | n/a        |
| xitca-router     | 14.79 µs | 103         | 13.18 KB   |
| ntex-router      | 47.29 µs | 306         | 22.97 KB   |
| route-recognizer | 76.19 µs | 3596        | 195.6 KB   |
| actix-router     | 557.3 µs | 6934        | 447.4 KB   |

#### String Parameters

| Library          | Time     | Alloc Count | Alloc Size |
|:-----------------|---------:|------------:|-----------:|
| matchit          | 7.499 µs | 1           | 128 B      |
| wayfind          | 7.874 µs | 0           | n/a        |
| path-tree        | 10.68 µs | 0           | n/a        |
| xitca-router     | 15.29 µs | 103         | 13.18 KB   |
| ntex-router      | 48.24 µs | 306         | 22.97 KB   |
| route-recognizer | 77.27 µs | 3596        | 195.6 KB   |
| actix-router     | 563.8 µs | 6934        | 447.4 KB   |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time     | Alloc Count | Alloc Size |
|:-----------------|---------:|------------:|-----------:|
| wayfind          | 3.749 µs | 0           | n/a        |
| path-tree        | 5.082 µs | 0           | n/a        |
| matchit          | 6.118 µs | 81          | 10.36 KB   |
| xitca-router     | 9.488 µs | 150         | 18.06 KB   |
| ntex-router      | 28.48 µs | 142         | 12.09 KB   |
| route-recognizer | 67.78 µs | 2813        | 184.2 KB   |
| actix-router     | 209.8 µs | 2142        | 121.4 KB   |

#### String Parameters

| Library          | Time     | Alloc Count | Alloc Size |
|:-----------------|---------:|------------:|-----------:|
| wayfind          | 4.041 µs | 0           | n/a        |
| path-tree        | 5.707 µs | 0           | n/a        |
| matchit          | 7.159 µs | 81          | 10.36 KB   |
| xitca-router     | 9.738 µs | 150         | 18.06 KB   |
| ntex-router      | 28.82 µs | 142         | 12.09 KB   |
| route-recognizer | 68.41 µs | 2813        | 184.2 KB   |
| actix-router     | 186.2 µs | 2142        | 121.4 KB   |

## `wayfind` benches

TODO.

The features `wayfind` provides come with inherent risk.
It is possible to insert a 'bad template' that poisons the performance of the entire router.
For example, having a top-level constrained wildcard route like `/{*path:constraint}`.
As such, it would be nice to have more `wayfind` focused benchmarks, to measure such scenarios.
