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
| matchit          | 5.457 µs | 1           | 128 B      |
| wayfind          | 7.500 µs | 0           | n/a        |
| path-tree        | 9.041 µs | 0           | n/a        |
| xitca-router     | 14.91 µs | 103         | 13.18 KB   |
| ntex-router      | 47.81 µs | 306         | 22.97 KB   |
| route-recognizer | 75.15 µs | 3596        | 195.6 KB   |
| actix-router     | 578.9 µs | 6934        | 447.4 KB   |

#### String Parameters

| Library          | Time     | Alloc Count | Alloc Size |
|:-----------------|---------:|------------:|-----------:|
| matchit          | 7.499 µs | 1           | 128 B      |
| wayfind          | 8.291 µs | 0           | n/a        |
| path-tree        | 10.41 µs | 0           | n/a        |
| xitca-router     | 15.41 µs | 103         | 13.18 KB   |
| ntex-router      | 48.41 µs | 306         | 22.97 KB   |
| route-recognizer | 76.28 µs | 3596        | 195.6 KB   |
| actix-router     | 568.3 µs | 6934        | 447.4 KB   |

### `path-tree` inspired benches

In a router of 320 templates, benchmark matching 80 paths.

#### Default

| Library          | Time     | Alloc Count | Alloc Size |
|:-----------------|---------:|------------:|-----------:|
| wayfind          | 3.666 µs | 0           | n/a        |
| path-tree        | 4.874 µs | 0           | n/a        |
| matchit          | 5.993 µs | 81          | 10.36 KB   |
| xitca-router     | 9.446 µs | 150         | 18.06 KB   |
| ntex-router      | 28.36 µs | 142         | 12.09 KB   |
| route-recognizer | 67.72 µs | 2813        | 184.2 KB   |
| actix-router     | 194.7 µs | 2142        | 121.4 KB   |

#### String Parameters

| Library          | Time     | Alloc Count | Alloc Size |
|:-----------------|---------:|------------:|-----------:|
| wayfind          | 4.082 µs | 0           | n/a        |
| path-tree        | 5.624 µs | 0           | n/a        |
| matchit          | 6.993 µs | 81          | 10.36 KB   |
| xitca-router     | 9.696 µs | 150         | 18.06 KB   |
| ntex-router      | 30.03 µs | 142         | 12.09 KB   |
| route-recognizer | 68.39 µs | 2813        | 184.2 KB   |
| actix-router     | 187.7 µs | 2142        | 121.4 KB   |

## `wayfind` benches

TODO.

The features `wayfind` provides come with inherent risk.
It is possible to insert a 'bad template' that poisons the performance of the entire router.
For example, having a top-level constrained wildcard route like `/{*path:constraint}`.
As such, it would be nice to have more `wayfind` focused benchmarks, to measure such scenarios.
